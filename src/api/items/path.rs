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
	/// Used internally to build OAuth URIs.
	pub fn entity_and_parameters(&self, segment: Option<&str>, parameters: Option<Parameters>) -> String {
		let segment = match segment {
			Some(segment) => segment,
			None => ""
		};

		let parameters: String = match parameters {
			Some(opts) => opts.to_string(),
			None => "".to_owned()
		};

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
}
