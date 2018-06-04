extern crate lmdb_rs as lmdb;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(client)]
pub use client::*;
pub use core::DB;
pub use error::{Error, Result};
pub use model::*;
#[cfg(server)]
pub use server::*;

#[cfg(client)]
pub mod client;
pub mod model;
#[cfg(server)]
pub mod server;

pub mod core;

mod error;

