#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

pub mod table;
pub mod table_fullcopy;
pub mod table_sharded;
pub mod table_sync;

pub use table::*;