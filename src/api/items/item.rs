use chrono::{DateTime, UTC};
use serde_json::Value;
use ::api::MultiOption;
use ::{Result, Error};


/// Item kind
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
	/// Folder
	Folder,
	/// File
	File,
}


/// Item
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
	pub kind: Kind,
	pub id: String,
	pub url: String,
	pub name: String,
	pub filename: String,
	pub description: String,
	pub size: u64,
	pub creation_date: DateTime<UTC>,
	pub meta: Option<Value>,
}


impl Item {
	pub fn from_value(value: Value, with_meta: bool) -> Result<MultiOption<Item>> {
		// Check if we have one item or many
		match value.find("odata.count") {
			Some(_) => {
				let mut items = Vec::new();
				for val in value.find("value").unwrap().as_array().unwrap() {
					match Item::item_from_value(val, with_meta) {
						Ok(item) => items.push(item),
						Err(err) => return err.result()
					};
				}

				Ok(MultiOption::Many(items))
			},
			None => {
				match Item::item_from_value(&value, with_meta) {
					Ok(item) => Ok(MultiOption::One(item)),
					Err(err) => return err.result()
				}
			}
		}
	}

	fn item_from_value(value: &Value, with_meta: bool) -> Result<Item> {
		let kind = match value.find("odata.type") {
			Some(otype) => match otype.as_string().unwrap() {
				"ShareFile.Api.Models.Folder" => Kind::Folder,
				"ShareFile.Api.Models.File" => Kind::File,
				k => return Error::new(format!("Unknown item kind {}.", k)).result()
			},
			None => {
				return Error::new("Item.odata.type property is missing.").result();
			}
		};

		let id = match value.find("Id") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Item.Id property is missing.").result()
		};

		let url = match value.find("url") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Item.url property is missing.").result()
		};

		let name = match value.find("Name") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Item.Name property is missing.").result()
		};

		let filename = match value.find("FileName") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Item.FileName property is missing.").result()
		};

		let description = match value.find("Description") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Item.Description property is missing.").result()
		};

		let size = match value.find("FileSizeBytes") {
			Some(v) => v.as_u64().unwrap(),
			None => return Error::new("Item.FileSizeBytes property is missing.").result()
		};

		let creation_date = match value.find("CreationDate") {
			Some(v) => match v.as_string().unwrap().parse::<DateTime<UTC>>() {
				Ok(dt) => dt,
				Err(err) => return Error::new("Item.CreationDate property is invalid").because(err).result()
			},
			None => return Error::new("Item.CreationDate property is missing.").result()
		};

		let meta = if with_meta {
			Some(value.clone())
		}
		else {
			None
		};

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
