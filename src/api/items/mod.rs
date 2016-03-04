mod path;
mod item;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::Parameters;
use ::api::MultiOption;
use ::Result;


pub use self::path::Path;
pub use self::item::{Item, Kind};


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
		self.query(path.entity_and_parameters(None, parameters))
	}

	pub fn list(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		match self.stat(path, None) {
			Ok(MultiOption::One(item)) => {
				self.query(Path::Id(item.id).entity_and_parameters(Some("/Children"), parameters))
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
