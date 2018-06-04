use bincode;
use error::Result;
use serde::{Deserialize, Serialize};

pub fn serialize<S: Serialize>(obj: &S) -> Result<Vec<u8>> {
	Ok(bincode::serialize(obj)?)
}

pub fn deserialize<'de, R: Deserialize<'de>>(bytes: &'de [u8]) -> Result<R> {
	Ok(bincode::deserialize(bytes)?)
}
