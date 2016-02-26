mod item;

use hyper::method::Method;
use serde_json;
use ::connection::Connection;
use ::odata::QueryOptions;
use ::api::MultiOption;
use ::{Result, Error};


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
}


impl Path {
	fn uri(&self, action: &Action) -> String {
		match self {
			&Path::Home => format!("Items(home){}?", action.to_string()),
			&Path::Favorites => format!("Items(favorites){}?", action.to_string()),
			&Path::AllShared => format!("Items(allshared){}?", action.to_string()),
			&Path::Connectors => format!("Items(connectors){}?", action.to_string()),
			&Path::Box => format!("Items(box){}?", action.to_string()),
			&Path::Top => format!("Items(top){}?", action.to_string()),
			&Path::Id(ref id) => format!("Items({}){}?", id, action.to_string()),
			&Path::Absolute(ref path) => format!("Items{}/ByPath?path={}&", action.to_string(), path),
		}
	}
}


/// Item action
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
	/// Get item stats
	Stat,
	/// List children items
	List,
}

impl ToString for Action {
	fn to_string(&self) -> String {
		match self {
			&Action::Stat => "".to_owned(),
			&Action::List => "/Children".to_owned(),
		}
	}
}


/// Items Entities
pub struct Items {
	conn: Connection,
	path: Option<Path>,
	action: Option<Action>,
	options: Option<QueryOptions>,
}


impl Items {
	pub fn new(conn: Connection) -> Self {
		Items {
			conn: conn,
			path: None,
			action: None,
			options: None,
		}
	}

	pub fn reset(&mut self) -> &mut Self {
		self.path = None;
		self.action = None;
		self.options = None;
		self
	}

	pub fn path(&mut self, path: Path) -> &mut Self {
		self.path = Some(path);
		self
	}

	pub fn action(&mut self, action: Action) -> &mut Self {
		self.action = Some(action);
		self
	}

	pub fn options(&mut self, options: QueryOptions) -> &mut Self {
		self.options = Some(options);
		self
	}

	pub fn query(&mut self) -> Result<MultiOption<Item>> {
		// Build URI
		let uri = {
			let path = match self.path {
				Some(ref path) => path,
				None => return Error::new("Path is required").result()
			};

			let action = match self.action {
				Some(ref action) => action,
				None => return Error::new("Action is required").result()
			};

			match self.options {
				Some(ref options) => format!("{}{}", path.uri(action), options.to_string()),
				None => path.uri(action)
			}
		};

		// Reset the state so the API can be reused
		self.reset();

		match self.conn.query(Method::Get, uri, None, None) {
			Ok(json) => Item::from_value(serde_json::from_str(&json).unwrap()),
			Err(err) => err.result()
		}
	}
}