use super::Kind;
use chrono::prelude::*;
use serde_json::{self, Value};
use ::{Error, Result};
use ::api::items::Path;
use ::api::users::UserId;


/// Share configuration.
///
/// Additional information can be found in [the official documentation](http://api.sharefile.com/rest/docs/resource.aspx?name=ShareFile.Api.Models.Share).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShareConfig {
    /// Share kind.
    kind: Kind,
    /// Share title.
    title: Option<String>,
    /// Folder location that contain the share files (`Send`); or the folder were files will be uploaded to (`Request`).
    parent: Option<Path>,
    /// List of shared Items (for `Send` shares only).
    items: Option<Vec<Path>>,
    /// List of users that have access to this share.
    recipients: Option<Vec<UserId>>,
    /// Date the share expires.
    expiration_date: Option<DateTime<Utc>>,
    /// If set to `true`, only authenticated users can download files from this share.
    require_login: Option<bool>,
    /// If set to `true`, users must provide Name, Email and Company information to download files from the share.
    require_user_info: Option<bool>,
    /// Maximum number of downloads each user can perform.
    max_downloads: Option<i32>,
}


impl ShareConfig {
    /// Create the new Share configuration for `Send`.
    pub fn send() -> ShareConfig {
        ShareConfig {
            kind: Kind::Send,
            title: None,
            parent: None,
            items: None,
            recipients: None,
            expiration_date: None,
            require_login: None,
            require_user_info: None,
            max_downloads: None,
        }
    }

    /// Create the new Share configuration for `Request`.
    pub fn request() -> ShareConfig {
        ShareConfig {
            kind: Kind::Request,
            title: None,
            parent: None,
            items: None,
            recipients: None,
            expiration_date: None,
            require_login: None,
            require_user_info: None,
            max_downloads: None,
        }
    }

    /// Test if the share is of type `Send`.
    pub fn is_send(&self) -> bool {
        self.kind.is_send()
    }

    /// Test if the share is of type `Request`.
    pub fn is_request(&self) -> bool {
        self.kind.is_request()
    }

    /// Set title.
    pub fn title<T>(mut self, title: T) -> Self
    where T: Into<String> {
        self.title = Some(title.into());
        self
    }

    /// Set parent.
    pub fn parent(mut self, parent: Path) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Set items.
    pub fn items(mut self, items: Vec<Path>) -> Self {
        self.items = Some(items);
        self
    }

    /// Set recipients.
    pub fn recipients(mut self, recipients: Vec<UserId>) -> Self {
        self.recipients = Some(recipients);
        self
    }

    /// Set expiration date.
    pub fn expiration_date(mut self, expiration_date: DateTime<Utc>) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    /// Set require login.
    pub fn require_login(mut self, require_login: bool) -> Self {
        self.require_login = Some(require_login);
        self
    }

    /// Set require user info.
    pub fn require_user_info(mut self, require_user_info: bool) -> Self {
        self.require_user_info = Some(require_user_info);
        self
    }

    /// Set maximum number of downloads.
    pub fn max_downloads(mut self, max_downloads: i32) -> Self {
        self.max_downloads = Some(max_downloads);
        self
    }

    /// Validate the share config.
    pub fn validate(&self) -> Result<()> {
        let mut errors: String = String::new();

        // Test required fields and data consistency
        if self.is_send() {
            // Test required Send fields
            // .. parent
            if let Some(ref parent) = self.parent {
                if !parent.is_id() {
                    errors.push_str("Share Parent item must be resolved to Id first.\n");
                }
            }

            // .. items
            if let Some(ref items) = self.items {
                for item in items {
                    if !item.is_id() {
                        errors.push_str("All Share Items must be resolved to Id first.\n");
                        break;
                    }
                }
            }
        }
        else {
            // Test required Request fields
            // .. parent
            if let Some(ref parent) = self.parent {
                if !parent.is_id() {
                    errors.push_str("Share Parent item must be resolved to Id first.\n");
                }
            }
            else {
                errors.push_str("Request Share requires Parent.\n");
            }
        }

        // Return the error if found any or success
        if errors.is_empty() {
            Ok(())
        }
        else {
            Err(Error::Other(errors))
        }
    }
}


// Convert Share Config into JSON Value
impl serde_json::value::ToJson for ShareConfig {
    fn to_json(&self) -> Value {
        use serde_json::value::Map;
        let mut object: Map<String, Value> = Map::new();

        // Check and correct max. downloads
        let mut max_downloads = self.max_downloads
            .clone()
            .unwrap_or(-1);

        if max_downloads <= 0 {
            max_downloads = -1;
        }

        // Add properties to the object
        // .. don't use stream IDs
        object.insert(
            "UsesStreamIDs".to_owned(),
            Value::Bool(false));
        // .. type
        object.insert(
            "ShareType".to_owned(),
            self.kind.to_json());
        // .. title
        if let Some(ref title) = self.title {
            object.insert(
                "Title".to_owned(),
                Value::String(title.clone()));
        }
        // .. require login
        object.insert(
            "RequireLogin".to_owned(),
            Value::Bool(
                self.require_login
                    .clone()
                    .unwrap_or(false)
            ));
        // .. require user info
        object.insert(
            "RequireUserInfo".to_owned(),
            Value::Bool(
                self.require_user_info
                    .clone()
                    .unwrap_or(false)
            ));
        // .. max. downloads
        object.insert(
            "MaxDownloads".to_owned(),
            Value::I64(max_downloads as i64));
        // .. expiration dtae
        if let Some(ref expiration_date) = self.expiration_date {
            object.insert(
                "ExpirationDate".to_owned(),
                Value::String(format!("{}", expiration_date.format("%Y-%m-%d"))));
        }
        // .. recipients
        if let Some(ref recipients) = self.recipients {
            let list: Vec<Value> = recipients.iter()
                .map(|v| {
                    let mut user_object: Map<String, Value> = Map::new();
                    user_object.insert("User".to_owned(), v.to_json());
                    Value::Object(user_object)
                })
                .collect();

            object.insert(
                "Recipients".to_owned(),
                Value::Array(list));
        }
        // .. parent
        if let Some(ref parent) = self.parent {
            object.insert(
                "Parent".to_owned(),
                parent.to_json());
        }
        // .. items
        if let Some(ref items) = self.items {
            let list: Vec<Value> = items.iter()
                .map(|v| v.to_json())
                .collect();

            object.insert(
                "Items".to_owned(),
                Value::Array(list));
        }

        Value::Object(object)
    }
}


// Convert Path into JSON Value
// Implement the trait here because we need specific behavior on
// convertion.
impl serde_json::value::ToJson for Path {
    fn to_json(&self) -> Value {
        use serde_json::value::Map;

        match self {
            &Path::Id(ref id) => {
                let mut object: Map<String, Value> = Map::new();
                object.insert("Id".to_owned(), Value::String(id.clone()));
                Value::Object(object)
            },
            _ => {
                Value::Null
            }
        }
    }
}
