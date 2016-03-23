/// Item kind
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
	/// Folder
	Folder,
	/// File
	File,
}


impl Kind {
	/// Test if that is folder
	pub fn is_folder(&self) -> bool {
		match self {
			&Kind::Folder => true,
			_ => false
		}
	}

	/// Test if that is file
	pub fn is_file(&self) -> bool {
		match self {
			&Kind::File => true,
			_ => false
		}
	}
}


#[cfg(test)]
mod tests {
	use super::Kind;

	#[test]
	fn test_is_folder() {
		let kind = Kind::Folder;
		assert!(kind.is_folder());
	}

	#[test]
	fn test_is_file() {
		let kind = Kind::File;
		assert!(kind.is_file());
	}
}