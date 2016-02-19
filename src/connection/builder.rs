use ::Result;
use super::ConnectionSettings;
use super::Connection;


#[derive(Debug, Clone)]
pub struct ConnectionBuilder {
	settings: ConnectionSettings,
}


impl ConnectionBuilder {
	pub fn new() -> Self {
		ConnectionBuilder {
			settings: ConnectionSettings::default(),
		}
	}

	pub fn subdomain<T>(mut self, subdomain: T) -> Self
		where T: Into<String> {
		self.settings.subdomain = Some(subdomain.into());
		self
	}

	pub fn username<T>(mut self, username: T) -> Self
		where T: Into<String> {
		self.settings.username = Some(username.into());
		self
	}

	pub fn password<T>(mut self, password: T) -> Self
		where T: Into<String> {
		self.settings.password = Some(password.into());
		self
	}

	pub fn client_id<T>(mut self, client_id: T) -> Self
		where T: Into<String> {
		self.settings.client_id = Some(client_id.into());
		self
	}

	pub fn client_secret<T>(mut self, client_secret: T) -> Self
		where T: Into<String> {
		self.settings.client_secret = Some(client_secret.into());
		self
	}

	pub fn connect(self) -> Result<Connection> {
		Connection::configured(self.settings)
			.connect()
	}
}
