#[derive(Debug, Clone)]
pub struct ConnectionSettings {
	pub subdomain: Option<String>,
	pub username: Option<String>,
	pub password: Option<String>,
	pub client_id: Option<String>,
	pub client_secret: Option<String>,
}


impl Default for ConnectionSettings {
	fn default() -> Self {
		ConnectionSettings {
			subdomain: None,
			username: None,
			password: None,
			client_id: None,
			client_secret: None,
		}
	}
}