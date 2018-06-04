pub use error::*;
use lmdb::{DbFlags, Environment, ToMdbValue};
use lmdb::core::MdbResult;
use lmdb::Database;
use serde::{de::DeserializeOwned, Serialize};
use std::mem;

mod db_serialization;

pub struct DB { env: Environment }

impl DB {
	pub fn read<V: DeserializeOwned>(&self, key: &[u8]) -> Result<V> {
		db_serialization::deserialize(reader(&self.env, |db| {
			db.get(&key)
		})?)
	}

	pub fn read_u64<V: DeserializeOwned>(&self, key: u64) -> Result<V> {
		let key: [u8; 8] = unsafe { mem::transmute(key) };
		self.read(&key)
	}

	pub fn read_range<V: DeserializeOwned>(&self, start_key: &[u8], end_key: &[u8]) -> Result<Vec<(Vec<u8>, V)>> {
		reader(&self.env, |db| {
			let mut res = vec![];

			for e in db.keyrange(&start_key, &end_key)? {
				let (k, v) = (e.get_key(), e.get_value());
				res.push((k, db_serialization::deserialize(v)))
			}

			Ok(res)
		})
	}

	pub fn read_range_u64<V: DeserializeOwned>(&self, start_key: u64, end_key: u64) -> Result<Vec<(Vec<u8>, V)>> {
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

	pub fn delete<V: Serialize>(&self, key: &[u8]) {
		unimplemented!()
	}

	pub fn delete_range<V: Serialize>(&self, begin_key: &[u8], end_key: &[u8]) {
		unimplemented!()
	}
}

fn batched_del<K: ToMdbValue + Sized>(env: &Environment, keys: Vec<K>) -> MdbResult<()> {
	batched_op(env, keys.len(), move |db, i| db.del(&keys[i]))
}

fn batched_set<K: ToMdbValue + Sized, V: ToMdbValue + Sized>(env: &Environment, pairs: Vec<(K, V)>) -> MdbResult<()> {
	batched_op(env, pairs.len(), move |db, i| db.set(&pairs[i].0, &pairs[i].1))
}

fn batched_op<F: Fn(&Database, usize) -> MdbResult<()> + Sized>(env: &Environment, count: usize, func: F) -> MdbResult<()> {
	let db_handle = env.get_default_db(DbFlags::empty())?;
	for h in 0..(count as f64 / 100.).ceil() as usize {
		let mut txn = env.new_transaction()?;
		{
			let mut db = txn.bind(&db_handle);
			for j in 0..100 {
				let i = h * 100 + j;
				if i >= count { break; }
				func(&db, i)?
			}
		}
		txn.commit()?;
	}
	Ok(())
}

fn writer<F: Fn(&Database) + Sized>(env: &Environment, func: F) -> MdbResult<()> {
	let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
	let txn = env.new_transaction().unwrap();
	func(&txn.bind(&db_handle));
	txn.commit()
}

fn reader<T, F: Fn(&Database) -> T + Sized>(env: &Environment, func: F) -> T {
	let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
	let reader = env.get_reader().unwrap();
	func(&reader.bind(&db_handle))
}