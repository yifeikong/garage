use std::path::PathBuf;

use tokio::sync::watch;

use garage_util::background::*;
use garage_util::config::*;
use garage_util::error::Error;

use garage_api::run_api_server;
use garage_model::garage::Garage;
use garage_web::run_web_server;
use garage_admin::*;

use crate::admin::*;

async fn wait_from(mut chan: watch::Receiver<bool>) {
	while !*chan.borrow() {
		if chan.changed().await.is_err() {
			return;
		}
	}
}

pub async fn run_server(config_file: PathBuf) -> Result<(), Error> {
	info!("Loading configuration...");
	let config = read_config(config_file).expect("Unable to read config file");

	info!("Opening database...");
	let mut db_path = config.metadata_dir.clone();
	db_path.push("db");
	let db = sled::Config::default()
		.path(&db_path)
		.cache_capacity(config.sled_cache_capacity)
		.flush_every_ms(Some(config.sled_flush_every_ms))
		.open()
		.expect("Unable to open sled DB");

	info!("Initializing background runner...");
	let watch_cancel = netapp::util::watch_ctrl_c();
	let (background, await_background_done) = BackgroundRunner::new(16, watch_cancel.clone());

	info!("Initializing Garage main data store...");
	let garage = Garage::new(config.clone(), db, background);

	let run_system = tokio::spawn(garage.system.clone().run(watch_cancel.clone()));

	info!("Create admin RPC handler...");
<<<<<<< HEAD
	AdminRpcHandler::new(garage.clone());

	info!("Initializing API server...");
	let api_server = tokio::spawn(run_api_server(
		garage.clone(),
		wait_from(watch_cancel.clone()),
	));

	info!("Initializing web server...");
	let web_server = tokio::spawn(run_web_server(
		garage.clone(),
		wait_from(watch_cancel.clone()),
	));

	info!("Initializing admin web server...");
	let admin_server = tokio::spawn(run_admin_server(
		garage.clone(),
		wait_from(watch_cancel.clone()),
	));

	// Stuff runs

	// When a cancel signal is sent, stuff stops
	if let Err(e) = api_server.await? {
		warn!("API server exited with error: {}", e);
	}
	if let Err(e) = web_server.await? {
		warn!("Web server exited with error: {}", e);
	}
	if let Err(e) = admin_server.await? {
		warn!("Admin web server exited with error: {}", e);
	}

	// Remove RPC handlers for system to break reference cycles
	garage.system.netapp.drop_all_handlers();

	// Await for netapp RPC system to end
	run_system.await?;

	// Break last reference cycles so that stuff can terminate properly
	garage.break_reference_cycles();
	drop(garage);

	// Await for all background tasks to end
	await_background_done.await?;
=======
	AdminRpcHandler::new(garage.clone()).register_handler(&mut rpc_server);

	info!("Initializing RPC and API servers...");
	let run_rpc_server = Arc::new(rpc_server).run(wait_from(watch_cancel.clone()));
	let api_server = run_api_server(garage.clone(), wait_from(watch_cancel.clone()));
	let web_server = run_web_server(garage.clone(), wait_from(watch_cancel.clone()));
	let admin_server = run_admin_server(garage.clone(), wait_from(watch_cancel.clone()));

	futures::try_join!(
		bootstrap.map(|()| {
			info!("Bootstrap done");
			Ok(())
		}),
		run_rpc_server.map(|rv| {
			info!("RPC server exited");
			rv
		}),
		api_server.map(|rv| {
			info!("API server exited");
			rv
		}),
		web_server.map(|rv| {
			info!("Web server exited");
			rv
		}),
		admin_server.map(|rv| {
			info!("Admin HTTP server exited");
			rv
		}),
		await_background_done.map(|rv| {
			info!("Background runner exited: {:?}", rv);
			Ok(())
		}),
		shutdown_signal(send_cancel),
	)?;
>>>>>>> Add configuration block

	info!("Cleaning up...");

	Ok(())
}
