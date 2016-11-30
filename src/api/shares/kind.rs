use serde_json::{self, Value};


/// Share Kind
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
	/// `Send` shares are used to send files and folders to the specified users.
	Send,
	/// `Request` shares are used to allow users to upload files to the share owner chosen location.
	Request,
}


impl Kind {
	/// Test if type is `Send`
	pub fn is_send(&self) -> bool {
		match self {
			&Kind::Send => true,
			_ => false,
		}
	}

	/// Test if type is `Request`
	pub fn is_request(&self) -> bool {
		match self {
			&Kind::Request => true,
			_ => false,
		}
	}
}


// Convert User ID into JSON Value
impl serde_json::value::ToJson for Kind {
	fn to_json(&self) -> Value {
		match self {
			&Kind::Send => {
				Value::String(String::from("Send"))
			},
			&Kind::Request => {
				Value::String(String::from("Request"))
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::Kind;
	use serde_json::value::ToJson;

	#[test]
	fn share_kind_send() {
		let share_kind = Kind::Send;
		assert_eq!(share_kind.is_send(), true);
		assert_eq!(share_kind.is_request(), false);
	}

	#[test]
	fn share_kind_send_to_json() {
		let share_kind = Kind::Send;
		let value = share_kind.to_json();
		assert!(value.is_string());

		if let Some(s) = value.as_str() {
			assert_eq!(s, "Send");
		}
		else {
			panic!("Cannot get contained value");
		}
	}

	#[test]
	fn share_kind_request() {
		let share_kind = Kind::Request;
		assert_eq!(share_kind.is_send(), false);
		assert_eq!(share_kind.is_request(), true);
	}

	#[test]
	fn share_kind_request_to_json() {
		let share_kind = Kind::Request;
		let value = share_kind.to_json();
		assert!(value.is_string());

		if let Some(s) = value.as_str() {
			assert_eq!(s, "Request");
		}
		else {
			panic!("Cannot get contained value");
		}
	}
}
