use lmdb::Environment;
use serde::{de::DeserializeOwned, Serialize};
use std::mem;

pub struct DB { env: Environment }

impl DB {
	pub fn read<V: DeserializeOwned>(&self, key: &[u8]) -> V {
		unimplemented!()
	}

	pub fn read_u64<V: DeserializeOwned>(&self, key: u64) -> V {
		let key: [u8; 8] = unsafe { mem::transmute(key) };
		self.read(&key)
	}

	pub fn read_range<V: DeserializeOwned>(&self, start_key: &[u8], end_key: &[u8]) -> V {
		unimplemented!()
	}

	pub fn read_range_u64<V: DeserializeOwned>(&self, start_key: u64, end_key: u64) -> V {
		let start_key: [u8; 8] = unsafe { mem::transmute(start_key) };
		let end_key: [u8; 8] = unsafe { mem::transmute(end_key) };
		self.read_range(&start_key, &end_key)
	}

	pub fn write<V: Serialize>(&self, key: &[u8], value: V, overwrite_on_duplicate: bool) {
		unimplemented!()
	}

	pub fn write_bulk<V: Serialize>(&self, values: Vec<(&[u8], V)>, overwrite_on_duplicate: bool) {
		unimplemented!()
	}
}
