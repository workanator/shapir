//! Items Entity


mod path;
mod kind;
mod item;
mod content;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::Parameters;
use ::api::MultiOption;
use ::{Result, Error};


pub use self::path::Path;
pub use self::kind::Kind;
pub use self::item::Item;
pub use self::content::Content;


/// Items Entity implementation.
///
/// Items struct implemets methods of [Items API Entity](http://api.sharefile.com/rest/docs/resource.aspx?name=Items)
pub struct Items {
	conn: Connection,
	meta: bool,
}


impl Items {
	/// Create a new instance of Item Entities API
	pub fn new(conn: Connection) -> Self {
		Items {
			conn: conn,
			meta: false,
		}
	}

	/// Set the flag to inform all future API requests to include item meta information
	/// or omit it.
	pub fn include_meta(&mut self, include: bool) {
		self.meta = include;
	}

	/// Search for Item(s) at the `path` given and return found. `parameters` can be used
	/// to provide additional options to the API request, like `includeDeleted`.
	pub fn stat(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		self.query_items(path.entity_and_parameters(None, parameters))
	}

	/// List all items at the `path` given. `parameters` can be used
	/// to provide additional options to the API request, like `includeDeleted`.
	pub fn list(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
		match self.stat(path, None) {
			Ok(MultiOption::One(item)) => {
				self.query_items(Path::Id(item.id).entity_and_parameters(Some("/Children"), parameters))
			},
			Ok(other) => Ok(other),
			Err(err) => err.result()
		}
	}

	/// Download the item identified by `path`. The method returns reader which can be used
	/// to read data in any convenient manner.  
	///
	/// The snippet of how the remote file can be downloaded to local.
	///
	/// ```ignore
	/// // Lets assume we have the opened connection already
	/// let path = Path::Absolute("/my_folder/remote_file.txt");
	///	let mut stream = conn.items().download(path).unwrap();
	/// let mut file = File::create("local_file.txt").unwrap();
	/// let mut buf = [0; 1024];
	/// loop {
	/// 	match stream.read(&mut buf) {
	///			Ok(0) => break, // Zero bytes mean the end of file
	///			Ok(n) => file.write_all(&buf[0..n]).unwrap(),
	///			Err(err) => panic!("{:?}", err)
	///		};
	///	}
	/// ```
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
