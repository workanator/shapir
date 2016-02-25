use ::connection::Connection;
use ::odata::QueryOptions;
use ::api::MultiOption;
use ::Result;


/// Item path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Path<T: Into<String>> {
	/// Home folder
	Home,
	/// Item identified by ID
	Id(T),
	/// Absolute path
	Absolute(T),
}


impl<T: Into<String>> Path<T> {
	fn normalize(self) -> Path<String> {
		match self {
			Path::Home => Path::Home,
			Path::Id(id) => Path::Id(id.into()),
			Path::Absolute(path) => Path::Absolute(path.into()),
		}
	}
}


impl<T: Into<String>> Into<String> for Path<T> {
	fn into(self) -> String {
		match self {
			Path::Home => "Items".to_string(),
			Path::Id(id) => format!("Items({})", id.into()),
			Path::Absolute(path) => format!("Items/ByPath?path={}", path.into()),
		}
	}
}


/// Item action
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action<T: Into<String>> {
	/// Get item stats
	Stat(Path<T>),
	/// List children items
	List(Path<T>),
}


impl<T: Into<String>> Action<T> {
	fn normalize(self) -> Action<String> {
		match self {
			Action::Stat(path) => Action::Stat(path.normalize()),
			Action::List(path) => Action::List(path.normalize()),
		}
	}
}


/// Items Entities
pub struct Items {
	conn: Connection,
	action: Option<Action<String>>,
	options: Option<QueryOptions>,
}


impl Items {
	pub fn new(conn: Connection) -> Self {
		Items {
			conn: conn,
			action: None,
			options: None,
		}
	}

	pub fn reset(&mut self) -> &mut Self {
		self.action = None;
		self.options = None;
		self
	}

	pub fn action<T: Into<String>>(&mut self, action: Action<T>) -> &mut Self {
		self.action = Some(action.normalize());
		self
	}

	pub fn options(&mut self, options: QueryOptions) -> &mut Self {
		self.options = Some(options);
		self
	}

	pub fn query(&mut self) -> Result<MultiOption<()>> {
		Ok(MultiOption::None)
	}
}