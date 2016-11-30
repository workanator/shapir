//! REST API Entities implementation


mod multi_option;
pub mod items;
pub mod users;

pub use self::multi_option::MultiOption;

fn bool_to_string(v: bool) -> String {
	if v {
		String::from("true")
	}
	else {
		String::from("false")
	}
}
