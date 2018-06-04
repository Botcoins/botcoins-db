use bincode::Error as BincodeError;
use lmdb::MdbError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;


macro_rules! error_derive {
	($name:ident, $($from:ident),*) => (
		#[derive(Deserialize, Serialize, Debug)]
		pub struct $name(pub String);

		impl StdError for $name {
			fn description<'a>(&'a self) -> &'a str {
				&self.0
			}
		}

		impl Display for $name {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				f.write_str(&self.0)
			}
		}

		impl From<String> for $name {
			fn from(s: String) -> Self {
				$name(s)
			}
		}

		impl Into<String> for $name {
			fn into(self) -> String {
				self.0
			}
		}

		impl<'a> From<&'a str> for $name {
			fn from(s: &'a str) -> Self {
				$name(s.to_string())
			}
		}

		$(impl From<$from> for $name {
			fn from(src: $from) -> $name {
				$name(format!("{:?}", src))
			}
		})*
	)
}

error_derive!(Error, BincodeError, MdbError);

pub type Result<T> = StdResult<T, Error>;

