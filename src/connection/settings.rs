use std::time::Duration;

/// Connection settings
///
/// For details on field purpose please refer the official REST API
/// [documentation](http://api.sharefile.com/rest/index/start.aspx)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionSettings {
    /// Read Timeout
    pub read_timeout: Option<Duration>,
    /// Write Timeout
    pub write_timeout: Option<Duration>,
	/// Subdomain (*required*)
	pub subdomain: Option<String>,
	/// Username (*required*)
	pub username: Option<String>,
	/// Password (*required*)
	pub password: Option<String>,
	/// Client ID (*required*)
	pub client_id: Option<String>,
	/// Client Secret (*required*)
	pub client_secret: Option<String>,
}


impl Default for ConnectionSettings {
	fn default() -> Self {
		ConnectionSettings {
			read_timeout: None,
            write_timeout: None,
			subdomain: None,
			username: None,
			password: None,
			client_id: None,
			client_secret: None,
		}
	}
}