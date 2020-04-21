use crate::data::*;
use crate::membership::{Ring, System};
use crate::table::*;

#[derive(Clone)]
pub struct TableShardedReplication {
	pub replication_factor: usize,
	pub read_quorum: usize,
	pub write_quorum: usize,
}

impl TableReplication for TableShardedReplication {
	// Sharded replication schema:
	// - based on the ring of nodes, a certain set of neighbors
	//   store entries, given as a function of the position of the
	//   entry's hash in the ring
	// - reads are done on all of the nodes that replicate the data
	// - writes as well

	fn read_nodes(&self, hash: &Hash, system: &System) -> Vec<UUID> {
		let ring = system.ring.borrow().clone();
		ring.walk_ring(&hash, self.replication_factor)
	}
	fn read_quorum(&self) -> usize {
		self.read_quorum
	}

	fn write_nodes(&self, hash: &Hash, system: &System) -> Vec<UUID> {
		let ring = system.ring.borrow().clone();
		ring.walk_ring(&hash, self.replication_factor)
	}
	fn write_quorum(&self) -> usize {
		self.write_quorum
	}
	fn max_write_errors(&self) -> usize {
		self.replication_factor - self.write_quorum
	}
	fn epidemic_writes(&self) -> bool {
		false
	}

	fn replication_nodes(&self, hash: &Hash, ring: &Ring) -> Vec<UUID> {
		ring.walk_ring(&hash, self.replication_factor)
	}
	fn split_points(&self, ring: &Ring) -> Vec<Hash> {
		let mut ret = vec![];

		ret.push([0u8; 32].into());
		for entry in ring.ring.iter() {
			ret.push(entry.location.clone());
		}
		ret.push([0xFFu8; 32].into());
		ret
	}
}