extern crate lmdb_rs as lmdb;
extern crate serde;

#[cfg(client)]
pub use client::*;
pub use core::DB;
pub use model::*;
#[cfg(server)]
pub use server::*;

#[cfg(client)]
pub mod client;
pub mod model;
#[cfg(server)]
pub mod server;

pub mod core;

