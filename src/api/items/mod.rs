//! Items Entity


mod path;
mod kind;
mod item;
mod content;

use std::collections::BTreeMap;
use hyper::method::Method;
use serde_json::Value;
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
    upload_chunk_size: Option<usize>,
}


impl Items {
    /// Create a new instance of Item Entities API
    pub fn new(conn: Connection) -> Self {
        Items {
            conn: conn,
            meta: false,
            upload_chunk_size: None,
        }
    }

    /// Create a configured instance of Item Entities API
    pub fn configured(conn: Connection, meta: bool, upload_chunk_size: Option<usize>) -> Self {
        Items {
            conn: conn,
            meta: meta,
            upload_chunk_size: upload_chunk_size,
        }
    }

    /// Set the flag to inform all future API requests to include item meta information
    /// or omit it.
    pub fn include_meta(&mut self, include: bool) {
        self.meta = include;
    }

    /// Get the upload chunk size. The uploading is going chunk by chunk and each chunk
    /// uploading requires HTTP request to be made.
    pub fn upload_chunk_size(&self) -> Option<usize> {
        self.upload_chunk_size
    }

    /// Set the upload chunk size.
    pub fn set_upload_chunk_size(&mut self, chunk_size: Option<usize>) {
        self.upload_chunk_size = chunk_size;
    }

    /// Resolve given `path` to the Item ID. On success returns `Some(Path::Id(id))`
    /// and None otherwise.
    pub fn resolve_path(&self, path: Path) -> Option<Path> {
        if let &Path::Id(_) = &path {
            // The path is the ID already
            Some(path)
        }
        else {
            // Resolve the path to the ID
            match self.stat(path, None) {
                Ok(MultiOption::One(item)) => Some(item.path()),
                _ => None
            }
        }
    }

    /// Search for Item(s) at the `path` given and return found. `parameters` can be used
    /// to provide additional options to the API request, like `includeDeleted`.
    pub fn stat(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
        use std::error::Error;

        self.get_items(path.entity_and_parameters(None, parameters))
            .or_else(|err| if err.description().starts_with("NotFound") {
                    Ok(MultiOption::None)
                }
                else {
                    Err(err)
                })
    }

    /// List all items at the `path` given. `parameters` can be used
    /// to provide additional options to the API request, like `includeDeleted`.
    pub fn list(&self, path: Path, parameters: Option<Parameters>) -> Result<MultiOption<Item>> {
        match self.stat(path, None) {
            Ok(MultiOption::One(item)) => match item.kind {
                Kind::Folder => {
                    self.get_items(item.path().entity_and_parameters(Some("/Children"), parameters))
                },
                Kind::File => {
                    Ok(MultiOption::One(item))
                }
            },
            Ok(other) => Ok(other),
            Err(e) => Err(e)
        }
    }

    /// Create folder with `parent` item and `name` given. On success returns the `Path` with the ID
    /// of the folder created.
    pub fn mkdir<T>(&self, parent: Path, name: T, description: Option<T>, overwite: bool) -> Result<Path>
    where T: Into<String> {
        if let Some(path) = self.resolve_path(parent) {
            // Prepare folder details
            let mut data = BTreeMap::new();
            data.insert(String::from("Name"), Value::String(name.into()));

            if let Some(desc) = description {
                data.insert(String::from("Description"), Value::String(desc.into()));
            }

            let body = Value::Object(data);

            // Create folder
            let parameters = Parameters::new()
                .custom(vec![
                    ("overwrite", super::bool_to_string(overwite)),
                    ("passthrough", String::from("false")) ]);

            let url = path.entity_and_parameters(Some("/Folder"), Some(parameters));
            
            self.conn.query_json(Method::Post, url, None, Some(body))
                .and_then(|v| Path::from_json(v))
        }
        else {
            Err(Error::from("Cannot resolve parent ID"))
        }
    }

    /// Removes the item with the ID given. `single_version` set to `true` will delete
    /// only the specified version rather than all sibling files with the same filename
    /// and `force_sync` set to `true` will block the operation from taking place
    /// asynchronously.
    pub fn remove(&self, path: Path, single_version: bool, force_sync: bool) -> Result<()> {
        self.resolve_path(path)
            .ok_or(Error::from("The Item is not found"))
            .and_then(|path| {
                let parameters = Parameters::new()
                    .custom(vec![
                        ("singleversion", super::bool_to_string(single_version)),
                        ("forceSync", super::bool_to_string(force_sync)) ]);

                let url = path.entity_and_parameters(None, Some(parameters));

                self.conn.query_string(Method::Delete, url, None, None)
                    .map(|_| ())
            })
    }

    /// Removes multiple items. All items in bulk delete must be children of the same parent.
    /// `delete_premanently` set to `true` will remove items from
    /// the Recycle Bin or bypass it entirely and `force_sync` set to `true` will block
    /// the operation from taking place asynchronously.
    pub fn remove_bulk(&self, parent: Path, items: Vec<Path>, delete_premanently: bool, force_sync: bool) -> Result<()> {
        self.resolve_path(parent)
            .ok_or(Error::from("The Parent Item is not found"))
            .and_then(|parent| {
                // Prepare item list
                let items: Vec<Value> = items.into_iter()
                    .map(|path| self.resolve_path(path)) // Resolve each Path into an Item ID
                    .filter(|path| path.is_some()) // Remove not resolved Paths
                    .map(|path| path.unwrap()) // Extract Paths from Option
                    .filter(|path| path.is_id()) // Remove those Paths which does not contain the ID
                    .map(|path| Value::String(path.id())) // Extract Item IDs and convert into Value::String
                    .collect();

                let body = Value::Array(items);

                // Perform Bulk Delete
                let parameters = Parameters::new()
                    .custom(vec![
                        ("deletePermanently", super::bool_to_string(delete_premanently)),
                        ("forceSync", super::bool_to_string(force_sync)) ]);

                let url = parent.entity_and_parameters(Some("/BulkDelete"), Some(parameters));

                self.conn.query_json(Method::Post, url, None, Some(body))
                    .map(|_| ())
            })
    }

    /// Download the item identified by `path`. The method returns reader which can be used
    /// to read data in any convenient manner.  
    ///
    /// The snippet of how the remote file can be downloaded to local.
    ///
    /// ```ignore
    /// // Lets assume we have the opened connection already
    /// let path = Path::Absolute(String::from("/my_folder/remote_file.txt"));
    /// let mut stream = conn.items().download(path).unwrap();
    /// let mut file = File::create("local_file.txt").unwrap();
    /// let mut buf = [0; 1024];
    /// loop {
    ///     match stream.read(&mut buf) {
    ///         Ok(0) => break, // Zero bytes mean the end of file
    ///         Ok(n) => file.write_all(&buf[0..n]).unwrap(),
    ///         Err(err) => panic!("{:?}", err)
    ///     };
    /// }
    /// ```
    pub fn download(&self, path: Path) -> Result<Content> {
        if path.is_id() {
            // We have the ID alredy so just start download
            Content::open_for_read(self.conn.clone(), path)
        }
        else {
            // We have a path which should be resolved to the id first
            match self.stat(path, None) {
                Ok(MultiOption::One(item)) => Content::open_for_read(self.conn.clone(), item.path()),
                Ok(MultiOption::Many(_)) => Err(Error::from("There are more than one Item on path")),
                Ok(MultiOption::None) => Err(Error::from("The Item is not found")),
                Err(e) => Err(e)
            }
        }
    }

    /// Upload the local file/stream into the folder identified by `parent`. The method
    /// returns the writer which can be used to write data in any convenient manner.
    ///
    /// The snippet of how the local file can be uploaded.
    ///
    /// ```ignore
    /// // Lets assume we have the opened connection already
    /// let file = File::open("local_file.txt").unwrap();
    /// let metadata = file.metadata().unwrap();
    /// let mut file = BufReader::new(file);
    ///
    /// let parent = Path::Absolute(String::from("/my_folder"));
    /// let mut stream = items.upload(parent, String::from("file.txt"), metadata.len(), false, true).unwrap();
    /// let mut buf = [0; 1024];
    ///
    /// loop {
    ///     match file.read(&mut buf) {
    ///         Ok(0) => break,
    ///         Ok(n) => stream.write_all(&buf[0..n]).unwrap(),
    ///         Err(err) => panic!("{:?}", err)
    ///     };
    /// }
    /// ```
    pub fn upload(&self, parent: Path, name: String, size: u64, unzip: bool, overwite: bool) -> Result<Content> {
        if parent.is_id() {
            // We have the ID alredy so just start download
            Content::open_for_write(self.conn.clone(), parent, name, size, unzip, overwite, self.upload_chunk_size)
        }
        else {
            // We have a path which should be resolved to the id first
            match self.stat(parent, None) {
                Ok(MultiOption::One(item)) => Content::open_for_write(self.conn.clone(), item.path(), name, size, unzip, overwite, self.upload_chunk_size),
                Ok(MultiOption::Many(_)) => Err(Error::from("There are more than one Item on path")),
                Ok(MultiOption::None) => Err(Error::from("The Item is not found")),
                Err(e) => Err(e)
            }
        }
    }

    // Do API request which returns Item Collection (GET)
    fn get_items(&self, uri: String) -> Result<MultiOption<Item>> {
        self.conn.query_json(Method::Get, uri, None, None)
            .and_then(|data| Item::from_value(data, self.meta))
    }
}
