//! Error type used in the library


use std;
use std::fmt;
use std::ops::Deref;
use serde_json::Value;


pub type Result<T> = std::result::Result<T, Error>;


/// Error type
#[derive(Debug)]
pub struct Error {
	// Description
	what: String,
	// Cause
	cause: Option<Box<std::error::Error + Sized>>,
}


impl Error {
	/// Construct `Error` with message only
	pub fn new<S>(what: S) -> Self
		where S: Into<String> {
		Error {
			what: what.into(),
			cause: None,
		}
	}

	/// Consumes self and constructs `Error` with message and cause
	pub fn because<E>(self, cause: E) -> Self
		where E: 'static + std::error::Error + Sized {
		Error {
			what: self.what,
			cause: Some(Box::new(cause)),
		}
	}

	/// Consumes self and retuns `Err` variant of `Result<T, Error>`.
	pub fn result<T>(self) -> Result<T> {
		Err(self)
	}

	// Parse the error from the result JSON
	pub fn from_json(value: Value) -> Result<Value> {
		match value.clone().find("code") {
			Some(code_value) => {
				// Get error code
				let code = code_value.as_string()
					.unwrap();

				// Get error message
				let message = value.find("message")
					.unwrap()
					.find("value")
					.unwrap()
					.as_string()
					.unwrap();

				Error::new(format!("{}: {}", code, message)).result()
			},
			None => Ok(value)
		}
	}
}


impl std::error::Error for Error {
	fn description(&self) -> &str {
		&self.what
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match self.cause {
			None => None,
			Some(ref boxed) => Some(boxed.deref())
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.cause {
			None => write!(f, "{}", self.what),
			Some(ref boxed) => write!(f, "{} because {}", self.what, &boxed),
		}
    }
}


#[cfg(test)]
mod tests {
	use super::Error;
	use serde_json::{self, Value};

	#[test]
	fn parse_error_json() {
		let data: Value = serde_json::from_str("{\"code\":\"BadRequest\",\"message\":{\"lang\":\"en-US\",\"value\":\"Invalid Argument Items.Folder\"},\"reason\":\"BadRequest\"}").unwrap();
		let error = Error::from_json(data);
		assert!(error.is_err());
	}

	#[test]
	fn parse_success_json() {
		let data: Value = serde_json::from_str("{\"Id\":\"some-file-id\"}").unwrap();
		let error = Error::from_json(data);
		assert!(error.is_ok());
	}
}
