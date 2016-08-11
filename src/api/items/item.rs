use chrono::{DateTime, UTC};
use serde_json::Value;
use ::api::MultiOption;
use super::{Path, Kind};
use ::error::{Result, Error, IoError, IoErrorKind};


/// Item details
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
	/// `Kind` of the item
	pub kind: Kind,
	/// ID of the item
	pub id: String,
	/// URL
	pub url: String,
	/// Item name
	pub name: String,
	/// Item file name
	pub filename: String,
	/// Item description
	pub description: String,
	/// Item size
	pub size: u64,
	/// Item creation date and time
	pub creation_date: DateTime<UTC>,
	/// Meta information as it returned from ShareFile REST API
	pub meta: Option<Value>,
}


impl Item {
	/// Get the `Path` pointing to the item
	pub fn path(&self) -> Path {
		Path::Id(self.id.clone())
	}

	/// Construct item(s) from the decoded JSON value. If `with_meta` is `true` then
	/// `Item.meta` field of the each item will be filled with the JSON value
	/// representing that item.
	pub fn from_value(value: Value, with_meta: bool) -> Result<MultiOption<Item>> {
		// Check if we have one item or many
		match value.find("odata.count") {
			Some(_) => {
				let mut items = Vec::new();
				for val in value.find("value").unwrap().as_array().unwrap() {
					match Item::item_from_value(val, with_meta) {
						Ok(item) => items.push(item),
						Err(e) => return Err(e)
					};
				}

				Ok(MultiOption::Many(items))
			},
			None => {
				match Item::item_from_value(&value, with_meta) {
					Ok(item) => Ok(MultiOption::One(item)),
					Err(e) => return Err(e)
				}
			}
		}
	}

	fn item_from_value(value: &Value, with_meta: bool) -> Result<Item> {
		// Which kind the item of
		let kind = match value.find("odata.type") {
			Some(otype) => match otype.as_string().unwrap() {
				"ShareFile.Api.Models.Folder" => Kind::Folder,
				"ShareFile.Api.Models.File" => Kind::File,
				k => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, format!("Unknown item kind {}.", k)))
			},
			None => {
				return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.odata.type property is missing."));
			}
		};

		// Get item ID
		let id = match value.find("Id") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.Id property is missing."))
		};

		// Get item URL
		let url = match value.find("url") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.url property is missing."))
		};

		// Get item name
		let name = match value.find("Name") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.Name property is missing."))
		};

		// Get item file name
		let filename = match value.find("FileName") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.FileName property is missing."))
		};

		// Get item description
		let description = match value.find("Description") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.Description property is missing."))
		};

		// Get item size
		let size = match value.find("FileSizeBytes") {
			Some(v) => v.as_u64().unwrap(),
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.FileSizeBytes property is missing."))
		};

		// Get item creation date and time
		let creation_date = match value.find("CreationDate") {
			Some(v) => match v.as_string().unwrap().parse::<DateTime<UTC>>() {
				Ok(dt) => dt,
				Err(err) => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, format!("Item.CreationDate property is invalid because {}", err)))
			},
			None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "Item.CreationDate property is missing."))
		};

		// Add meta to the item if requested
		let meta = if with_meta {
			Some(value.clone())
		}
		else {
			None
		};

		// COnstruct and return item
		Ok(Item {
			kind: kind,
			id: id.to_owned(),
			url: url.to_owned(),
			name: name.to_owned(),
			filename: filename.to_owned(),
			description: description.to_owned(),
			size: size,
			creation_date: creation_date,
			meta: meta,
		})
	}
}
