extern crate bincode;

use serde::{Deserialize, Serialize};

pub fn serialize<S: Serialize>(obj: &S) -> Vec<u8> {
	self::bincode::serialize(obj).unwrap()
}

pub fn deserialize<'de, R: Deserialize<'de>>(bytes: &'de [u8]) -> R {
	self::bincode::deserialize(bytes).unwrap()
}
