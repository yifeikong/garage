use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use futures::stream::*;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use garage_util::data::*;
use garage_util::error::Error;

use garage_rpc::system::System;
use garage_rpc::*;

use crate::crdt::Crdt;
use crate::data::*;
use crate::gc::*;
use crate::merkle::*;
use crate::replication::*;
use crate::schema::*;
use crate::sync::*;

const TABLE_RPC_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Table<F: TableSchema + 'static, R: TableReplication + 'static> {
	pub system: Arc<System>,
	pub data: Arc<TableData<F, R>>,
	pub merkle_updater: Arc<MerkleUpdater<F, R>>,
	pub syncer: Arc<TableSyncer<F, R>>,
	endpoint: Arc<Endpoint<TableRpc<F>, Self>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum TableRpc<F: TableSchema> {
	Ok,

	ReadEntry(F::P, F::S),
	ReadEntryResponse(Option<ByteBuf>),

	// Read range: read all keys in partition P, possibly starting at a certain sort key offset
	ReadRange(F::P, Option<F::S>, Option<F::Filter>, usize),

	Update(Vec<Arc<ByteBuf>>),
}

impl<F: TableSchema> Rpc for TableRpc<F> {
	type Response = Result<TableRpc<F>, Error>;
}

impl<F, R> Table<F, R>
where
	F: TableSchema + 'static,
	R: TableReplication + 'static,
{
	// =============== PUBLIC INTERFACE FUNCTIONS (new, insert, get, etc) ===============

	pub fn new(instance: F, replication: R, system: Arc<System>, db: &sled::Db) -> Arc<Self> {
		let endpoint = system
			.netapp
			.endpoint(format!("garage_table/table.rs/Rpc:{}", F::TABLE_NAME));

		let data = TableData::new(system.clone(), instance, replication, db);

		let merkle_updater = MerkleUpdater::launch(&system.background, data.clone());

		let syncer = TableSyncer::launch(system.clone(), data.clone(), merkle_updater.clone());
		TableGc::launch(system.clone(), data.clone());

		let table = Arc::new(Self {
			system,
			data,
			merkle_updater,
			syncer,
			endpoint,
		});

		table.endpoint.set_handler(table.clone());

		table
	}

	pub async fn insert(&self, e: &F::E) -> Result<(), Error> {
		let hash = e.partition_key().hash();
		let who = self.data.replication.write_nodes(&hash);
		//eprintln!("insert who: {:?}", who);

		let e_enc = Arc::new(ByteBuf::from(rmp_to_vec_all_named(e)?));
		let rpc = TableRpc::<F>::Update(vec![e_enc]);

		self.system
			.rpc
			.try_call_many(
				&self.endpoint,
				&who[..],
				rpc,
				RequestStrategy::with_priority(PRIO_NORMAL)
					.with_quorum(self.data.replication.write_quorum())
					.with_timeout(TABLE_RPC_TIMEOUT),
			)
			.await?;
		Ok(())
	}

	pub async fn insert_many(&self, entries: &[F::E]) -> Result<(), Error> {
		let mut call_list: HashMap<_, Vec<_>> = HashMap::new();

		for entry in entries.iter() {
			let hash = entry.partition_key().hash();
			let who = self.data.replication.write_nodes(&hash);
			let e_enc = Arc::new(ByteBuf::from(rmp_to_vec_all_named(entry)?));
			for node in who {
				call_list.entry(node).or_default().push(e_enc.clone());
			}
		}

		let call_futures = call_list.drain().map(|(node, entries)| async move {
			let rpc = TableRpc::<F>::Update(entries);

			let resp = self
				.system
				.rpc
				.call(
					&self.endpoint,
					node,
					rpc,
					RequestStrategy::with_priority(PRIO_NORMAL).with_timeout(TABLE_RPC_TIMEOUT),
				)
				.await?;
			Ok::<_, Error>((node, resp))
		});
		let mut resps = call_futures.collect::<FuturesUnordered<_>>();
		let mut errors = vec![];

		while let Some(resp) = resps.next().await {
			if let Err(e) = resp {
				errors.push(e);
			}
		}
		if errors.len() > self.data.replication.max_write_errors() {
			Err(Error::Message("Too many errors".into()))
		} else {
			Ok(())
		}
	}

	pub async fn get(
		self: &Arc<Self>,
		partition_key: &F::P,
		sort_key: &F::S,
	) -> Result<Option<F::E>, Error> {
		let hash = partition_key.hash();
		let who = self.data.replication.read_nodes(&hash);
		//eprintln!("get who: {:?}", who);

		let rpc = TableRpc::<F>::ReadEntry(partition_key.clone(), sort_key.clone());
		let resps = self
			.system
			.rpc
			.try_call_many(
				&self.endpoint,
				&who[..],
				rpc,
				RequestStrategy::with_priority(PRIO_NORMAL)
					.with_quorum(self.data.replication.read_quorum())
					.with_timeout(TABLE_RPC_TIMEOUT)
					.interrupt_after_quorum(true),
			)
			.await?;

		let mut ret = None;
		let mut not_all_same = false;
		for resp in resps {
			if let TableRpc::ReadEntryResponse(value) = resp {
				if let Some(v_bytes) = value {
					let v = self.data.decode_entry(v_bytes.as_slice())?;
					ret = match ret {
						None => Some(v),
						Some(mut x) => {
							if x != v {
								not_all_same = true;
								x.merge(&v);
							}
							Some(x)
						}
					}
				}
			} else {
				return Err(Error::Message("Invalid return value to read".to_string()));
			}
		}
		if let Some(ret_entry) = &ret {
			if not_all_same {
				let self2 = self.clone();
				let ent2 = ret_entry.clone();
				self.system
					.background
					.spawn_cancellable(async move { self2.repair_on_read(&who[..], ent2).await });
			}
		}
		Ok(ret)
	}

	pub async fn get_range(
		self: &Arc<Self>,
		partition_key: &F::P,
		begin_sort_key: Option<F::S>,
		filter: Option<F::Filter>,
		limit: usize,
	) -> Result<Vec<F::E>, Error> {
		let hash = partition_key.hash();
		let who = self.data.replication.read_nodes(&hash);

		let rpc = TableRpc::<F>::ReadRange(partition_key.clone(), begin_sort_key, filter, limit);

		let resps = self
			.system
			.rpc
			.try_call_many(
				&self.endpoint,
				&who[..],
				rpc,
				RequestStrategy::with_priority(PRIO_NORMAL)
					.with_quorum(self.data.replication.read_quorum())
					.with_timeout(TABLE_RPC_TIMEOUT)
					.interrupt_after_quorum(true),
			)
			.await?;

		let mut ret = BTreeMap::new();
		let mut to_repair = BTreeMap::new();
		for resp in resps {
			if let TableRpc::Update(entries) = resp {
				for entry_bytes in entries.iter() {
					let entry = self.data.decode_entry(entry_bytes.as_slice())?;
					let entry_key = self.data.tree_key(entry.partition_key(), entry.sort_key());
					match ret.remove(&entry_key) {
						None => {
							ret.insert(entry_key, Some(entry));
						}
						Some(Some(mut prev)) => {
							let must_repair = prev != entry;
							prev.merge(&entry);
							if must_repair {
								to_repair.insert(entry_key.clone(), Some(prev.clone()));
							}
							ret.insert(entry_key, Some(prev));
						}
						Some(None) => unreachable!(),
					}
				}
			}
		}
		if !to_repair.is_empty() {
			let self2 = self.clone();
			self.system.background.spawn_cancellable(async move {
				for (_, v) in to_repair.iter_mut() {
					self2.repair_on_read(&who[..], v.take().unwrap()).await?;
				}
				Ok(())
			});
		}
		let ret_vec = ret
			.iter_mut()
			.take(limit)
			.map(|(_k, v)| v.take().unwrap())
			.collect::<Vec<_>>();
		Ok(ret_vec)
	}

	// =============== UTILITY FUNCTION FOR CLIENT OPERATIONS ===============

	async fn repair_on_read(&self, who: &[Uuid], what: F::E) -> Result<(), Error> {
		let what_enc = Arc::new(ByteBuf::from(rmp_to_vec_all_named(&what)?));
		self.system
			.rpc
			.try_call_many(
				&self.endpoint,
				who,
				TableRpc::<F>::Update(vec![what_enc]),
				RequestStrategy::with_priority(PRIO_NORMAL)
					.with_quorum(who.len())
					.with_timeout(TABLE_RPC_TIMEOUT),
			)
			.await?;
		Ok(())
	}
}

#[async_trait]
impl<F, R> EndpointHandler<TableRpc<F>> for Table<F, R>
where
	F: TableSchema + 'static,
	R: TableReplication + 'static,
{
	async fn handle(
		self: &Arc<Self>,
		msg: &TableRpc<F>,
		_from: NodeID,
	) -> Result<TableRpc<F>, Error> {
		match msg {
			TableRpc::ReadEntry(key, sort_key) => {
				let value = self.data.read_entry(key, sort_key)?;
				Ok(TableRpc::ReadEntryResponse(value))
			}
			TableRpc::ReadRange(key, begin_sort_key, filter, limit) => {
				let values = self.data.read_range(key, begin_sort_key, filter, *limit)?;
				Ok(TableRpc::Update(values))
			}
			TableRpc::Update(pairs) => {
				self.data.update_many(pairs)?;
				Ok(TableRpc::Ok)
			}
			m => Err(Error::unexpected_rpc_message(m)),
		}
	}
}
