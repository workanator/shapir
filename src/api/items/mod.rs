mod item;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::QueryOptions;
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
	fn uri(&self, part: Option<&str>, options: Option<QueryOptions>) -> String {
		let part = match part {
			Some(part) => part,
			None => ""
		};

		let options: String = match options {
			Some(opts) => opts.to_string(),
			None => "".to_owned()
		};

		match self {
			&Path::Home => format!("Items(home){}?{}", part, options),
			&Path::Favorites => format!("Items(favorites){}?{}", part, options),
			&Path::AllShared => format!("Items(allshared){}?{}", part, options),
			&Path::Connectors => format!("Items(connectors){}?{}", part, options),
			&Path::Box => format!("Items(box){}?{}", part, options),
			&Path::Top => format!("Items(top){}?{}", part, options),
			&Path::Id(ref id) => format!("Items({}){}?{}", id, part, options),
			&Path::Absolute(ref path) => format!("Items/ByPath?path={}&{}", path, options),
			&Path::Relative(ref id, ref path) => format!("Items({})/ByPath?path={}&{}", id, path, options),
			&Path::Parent(ref id) => format!("Items({})/Parent&{}", id, options),
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

	pub fn stat(&self, path: Path, options: Option<QueryOptions>) -> Result<MultiOption<Item>> {
		self.query(path.uri(None, options))
	}

	pub fn list(&self, path: Path, options: Option<QueryOptions>) -> Result<MultiOption<Item>> {
		match self.stat(path, None) {
			Ok(MultiOption::One(item)) => {
				self.query(Path::Id(item.id).uri(Some("/Children"), options))
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
