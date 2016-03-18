mod path;
mod item;
mod content;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::Parameters;
use ::api::MultiOption;
use ::{Result, Error};


pub use self::path::Path;
pub use self::item::{Item, Kind};
pub use self::content::Content;


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
		self.query_items(path.entity_and_parameters(None, parameters))
	}

	pub fn list(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		match self.stat(path, None) {
			Ok(MultiOption::One(item)) => {
				self.query_items(Path::Id(item.id).entity_and_parameters(Some("/Children"), parameters))
			},
			Ok(other) => Ok(other),
			Err(err) => err.result()
		}
	}

	pub fn download(&self, path: Path) -> Result<Content> {
		if let Path::Id(id) = path {
			// We have the ID alredy so just start download
			Content::open_for_read(self.conn.clone(), id)
		}
		else {
			// We have a path which should be resolved to the id first
			match self.stat(path, None) {
				Ok(MultiOption::One(item)) => Content::open_for_read(self.conn.clone(), item.id),
				Ok(MultiOption::Many(_)) => Error::new("There are more than one Item on path").result(),
				Ok(MultiOption::None) => Error::new("The Item is not found").result(),
				Err(err) => err.result()
			}
		}
	}

	fn query_items(&self, uri: String) -> Result<MultiOption<Item>> {
		match self.conn.query_string(Method::Get, uri, None, None) {
			Ok(json) => Item::from_value(serde_json::from_str(&json).unwrap(), self.meta),
			Err(err) => err.result()
		}
	}
}
