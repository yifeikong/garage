//! Crate for handling the admin and metric HTTP APIs
#[macro_use]
extern crate log;
extern crate lazy_static;

mod metrics;
pub use metrics::run_admin_server;