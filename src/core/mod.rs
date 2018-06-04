pub use error::*;
use lmdb::{DbFlags, EnvBuilder, Environment, ToMdbValue};
use lmdb::core::MdbResult;
use lmdb::Database;
use serde::{de::DeserializeOwned, Serialize};

mod db_serialization;

#[cfg(test)]
mod tests;

pub struct DB {
	env: Environment,
}

impl DB {
	pub fn init(dir: &str, initial_map_size: u64) -> Result<DB> {
		let env = EnvBuilder::new()
			.map_size(initial_map_size)
			.open(dir, 0o640)?;

		Ok(DB { env })
	}

	pub fn read<V: DeserializeOwned>(&self, key: &[u8]) -> Result<V> {
		db_serialization::deserialize(reader(&self.env, |db| {
			Ok(db.get(&key)?)
		})?)
	}

	pub fn read_range<V: DeserializeOwned>(&self, start_key: &[u8], end_key: &[u8]) -> Result<Vec<(Vec<u8>, V)>> {
		reader(&self.env, |db| {
			let mut res = vec![];

			for entry in db.keyrange(&start_key, &end_key)? {
				let (k, v) = (entry.get_key(), entry.get_value());
				res.push((k, db_serialization::deserialize(v)?))
			}

			Ok(res)
		})
	}

	pub fn read_range_vars<V: DeserializeOwned>(&self, start_key: &[u8], end_key: &[u8]) -> Result<Vec<V>> {
		reader(&self.env, |db| {
			let mut res = vec![];

			for entry in db.keyrange(&start_key, &end_key)? {
				res.push(db_serialization::deserialize(entry.get_value())?);
			}

			Ok(res)
		})
	}

	pub fn search_range<V: DeserializeOwned, F: Fn(&V) -> bool + Sized + 'static>(&self, start_key: &[u8], end_key: &[u8], search: F) -> Result<V> {
		reader(&self.env, |db| {
			for entry in db.keyrange(&start_key, &end_key)? {
				let v = db_serialization::deserialize(entry.get_value())?;
				if (search)(&v) {
					return Ok(v);
				}
			}

			Err("Term not found.".into())
		})
	}

	pub fn key_range(&self, start_key: &[u8], end_key: &[u8]) -> Result<Vec<Vec<u8>>> {
		reader(&self.env, |db| {
			let mut res = vec![];

			for entry in db.keyrange(&start_key, &end_key)? {
				res.push(entry.get_key())
			}

			Ok(res)
		})
	}

	pub fn write<V: Serialize>(&self, key: &[u8], value: V) -> Result<()> {
		Ok(writer(&self.env, |db| {
			let v = db_serialization::serialize(&value)?;
			let _ = db.set(&key, &v);

			Ok(())
		})?)
	}

	pub fn write_bulk<V: Serialize>(&self, values: Vec<(&[u8], V)>) -> Result<()> {
		let mut serialized = Vec::with_capacity(values.len());

		for (k, v) in values {
			let v = db_serialization::serialize(&v)?;
			serialized.push((k, v));
		}

		Ok(batched_set(&self.env, serialized)?)
	}

	pub fn delete(&self, key: &[u8]) -> Result<()> {
		Ok(writer(&self.env, |db| {
			Ok(db.del(&key)?)
		})?)
	}

	pub fn delete_bulk(&self, keys: Vec<&[u8]>) -> Result<()> {
		Ok(batched_del(&self.env, keys)?)
	}

	pub fn delete_range(&self, start_key: &[u8], end_key: &[u8]) -> Result<()> {
		let keys: Vec<Vec<u8>> = self.key_range(start_key, end_key)?;

		let mut key_refs: Vec<&[u8]> = vec![];
		for key in &keys {
			key_refs.push(&key[..]);
		}

		Ok(self.delete_bulk(key_refs)?)
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

fn writer<F: Fn(&Database) -> Result<()> + Sized>(env: &Environment, func: F) -> Result<()> {
	let db_handle = env.get_default_db(DbFlags::empty())?;
	let txn = env.new_transaction()?;
	func(&txn.bind(&db_handle))?;
	Ok(txn.commit()?)
}

fn reader<T, F: Fn(&Database) -> Result<T> + Sized>(env: &Environment, func: F) -> Result<T> {
	let db_handle = env.get_default_db(DbFlags::empty())?;
	let reader = env.get_reader()?;
	func(&reader.bind(&db_handle))
}
