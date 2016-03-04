mod item;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::Parameters;
use ::api::MultiOption;
use ::Result;


pub use self::item::{Item, Kind};


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
	fn uri(&self, part: Option<&str>, parameters: Option<Parameters>) -> String {
		let part = match part {
			Some(part) => part,
			None => ""
		};

		let parameters: String = match parameters {
			Some(opts) => opts.to_string(),
			None => "".to_owned()
		};

		match self {
			&Path::Home => format!("Items(home){}?{}", part, parameters),
			&Path::Favorites => format!("Items(favorites){}?{}", part, parameters),
			&Path::AllShared => format!("Items(allshared){}?{}", part, parameters),
			&Path::Connectors => format!("Items(connectors){}?{}", part, parameters),
			&Path::Box => format!("Items(box){}?{}", part, parameters),
			&Path::Top => format!("Items(top){}?{}", part, parameters),
			&Path::Id(ref id) => format!("Items({}){}?{}", id, part, parameters),
			&Path::Absolute(ref path) => format!("Items/ByPath?path={}&{}", path, parameters),
			&Path::Relative(ref id, ref path) => format!("Items({})/ByPath?path={}&{}", id, path, parameters),
			&Path::Parent(ref id) => format!("Items({})/Parent&{}", id, parameters),
		}
	}
}


/// Items Entities
pub struct Items {
	conn: Connection,
	meta: bool,
}


impl Items {
	pub fn new(conn: Connection) -> Self {
		Items {
			conn: conn,
			meta: false,
		}
	}

	pub fn include_meta(&mut self, include: bool) {
		self.meta = include;
	}

	pub fn stat(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		self.query(path.uri(None, parameters))
	}

	pub fn list(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		match self.stat(path, None) {
			Ok(MultiOption::One(item)) => {
				self.query(Path::Id(item.id).uri(Some("/Children"), parameters))
			},
			Ok(other) => Ok(other),
			Err(err) => err.result()
		}
	}

	fn query(&self, uri: String) -> Result<MultiOption<Item>> {
		match self.conn.query(Method::Get, uri, None, None) {
			Ok(json) => Item::from_value(serde_json::from_str(&json).unwrap(), self.meta),
			Err(err) => err.result()
		}
	}
}
