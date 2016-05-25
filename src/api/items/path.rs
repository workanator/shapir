use serde_json::Value;
use ::{Error, Result};
use ::odata::Parameters;

/// Item path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Path {
	/// The Home folder
	Home,
	/// The Parent favorite item
	Favorites,
	/// The Parent Shared Folders items
	AllShared,
	/// The Parent Connectors item
	Connectors,
	/// The FileBox item
	Box,
	/// The Top item
	Top,
	/// The item identified by ID
	Id(String),
	/// The absolute path
	Absolute(String),
	/// The relative path
	Relative(String, String),
	/// Parent
	Parent(String),
}


impl Path {
	/// Get Item ID if the Path is Id or empty string otherwise.
	pub fn id(&self) -> String {
		match self {
			&Path::Id(ref id) => id.clone(),
			_ => String::new()
		}
	}

	/// Test if path points to the home folder
	pub fn is_home(&self) -> bool {
		match self {
			&Path::Home => true,
			_ => false
		}
	}

	/// Test if path points to the favorites folder
	pub fn is_favorites(&self) -> bool {
		match self {
			&Path::Favorites => true,
			_ => false
		}
	}

	/// Test if path points to the all shared folder
	pub fn is_allshared(&self) -> bool {
		match self {
			&Path::AllShared => true,
			_ => false
		}
	}

	/// Test if path points to the connectors folder
	pub fn is_connectors(&self) -> bool {
		match self {
			&Path::Connectors => true,
			_ => false
		}
	}

	/// Test if path points to the filebox folder
	pub fn is_box(&self) -> bool {
		match self {
			&Path::Box => true,
			_ => false
		}
	}

	/// Test if path points to the top folder
	pub fn is_top(&self) -> bool {
		match self {
			&Path::Top => true,
			_ => false
		}
	}

	/// Test if path holds the ID
	pub fn is_id(&self) -> bool {
		match self {
			&Path::Id(_) => true,
			_ => false
		}
	}

	/// Test if path is absolute
	pub fn is_absolute(&self) -> bool {
		match self {
			&Path::Absolute(_) => true,
			_ => false
		}
	}

	/// Test if path is relative
	pub fn is_relative(&self) -> bool {
		match self {
			&Path::Relative(_, _) => true,
			_ => false
		}
	}

	/// Test if path points to the parent item
	pub fn is_parent(&self) -> bool {
		match self {
			&Path::Parent(_) => true,
			_ => false
		}
	}

	/// Used internally to build OAuth URIs.
	pub fn entity_and_parameters(&self, segment: Option<&str>, parameters: Option<Parameters>) -> String {
		let segment = segment.map_or("", |s| s);
		let parameters = parameters.map_or(String::from(""), |p| p.to_string());

		match self {
			&Path::Home => format!("Items(home){}?{}", segment, parameters),
			&Path::Favorites => format!("Items(favorites){}?{}", segment, parameters),
			&Path::AllShared => format!("Items(allshared){}?{}", segment, parameters),
			&Path::Connectors => format!("Items(connectors){}?{}", segment, parameters),
			&Path::Box => format!("Items(box){}?{}", segment, parameters),
			&Path::Top => format!("Items(top){}?{}", segment, parameters),
			&Path::Id(ref id) => format!("Items({}){}?{}", id, segment, parameters),
			&Path::Absolute(ref path) => format!("Items/ByPath?path={}&{}", path, parameters),
			&Path::Relative(ref id, ref path) => format!("Items({})/ByPath?path={}&{}", id, path, parameters),
			&Path::Parent(ref id) => format!("Items({})/Parent&{}", id, parameters),
		}
	}

	// Parse the path from the result JSON
	pub fn from_json(value: Value) -> Result<Path> {
		value.find("Id")
			.ok_or(Error::new("Cannot find Item ID"))
			.map(|id| Path::Id(id.as_string().unwrap().to_owned()))
	}
}


#[cfg(test)]
mod tests {
	use super::Path;
	use serde_json::{self, Value};

	#[test]
	fn path_get_valid_id() {
		let path = Path::Id(String::from("123"));
		assert_eq!(path.id(), String::from("123"));
	}

	#[test]
	fn path_get_invalid_id() {
		let path = Path::Home;
		assert_eq!(path.id(), String::new());
	}

	#[test]
	fn path_home() {
		let path = Path::Home;
		assert!(path.is_home());
	}

	#[test]
	fn path_favorites() {
		let path = Path::Favorites;
		assert!(path.is_favorites());
	}

	#[test]
	fn path_allshared() {
		let path = Path::AllShared;
		assert!(path.is_allshared());
	}

	#[test]
	fn path_connectors() {
		let path = Path::Connectors;
		assert!(path.is_connectors());
	}

	#[test]
	fn path_box() {
		let path = Path::Box;
		assert!(path.is_box());
	}

	#[test]
	fn path_top() {
		let path = Path::Top;
		assert!(path.is_top());
	}

	#[test]
	fn path_id() {
		let path = Path::Id("id".to_owned());
		assert!(path.is_id());
	}

	#[test]
	fn path_absolute() {
		let path = Path::Absolute("path".to_owned());
		assert!(path.is_absolute());
	}

	#[test]
	fn path_relative() {
		let path = Path::Relative("this".to_owned(), "that".to_owned());
		assert!(path.is_relative());
	}

	#[test]
	fn path_parent() {
		let path = Path::Parent("up".to_owned());
		assert!(path.is_parent());
	}

	#[test]
	fn parse_valid_path() {
		let data: Value = serde_json::from_str("{\"Id\":\"fi111111-2222-3333-4444-555555555555\"}").unwrap();
		let path = Path::from_json(data).unwrap();
		assert!(path.is_id());
	}

	#[test]
	#[should_panic]
	fn parse_invalid_path() {
		let data: Value = serde_json::from_str("{\"code\":\"error\"}").unwrap();
		let _ = Path::from_json(data).unwrap();
	}

	#[test]
	fn parse_path_id() {
		let data: Value = serde_json::from_str("{\"Id\":\"fi111111-2222-3333-4444-555555555555\"}").unwrap();
		match Path::from_json(data).unwrap() {
			Path::Id(id) => assert_eq!(id, "fi111111-2222-3333-4444-555555555555"),
			_ => panic!("Parsed Path must be Id")
		};
	}
}
