use std::time::Duration;
use ::Result;
use super::ConnectionSettings;
use super::Connection;


/// Connection configuration builder
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionBuilder {
	settings: ConnectionSettings,
}


impl ConnectionBuilder {
	/// Create the new instance of the builder
	pub fn new() -> Self {
		ConnectionBuilder {
			settings: ConnectionSettings::default(),
		}
	}

	/// Set `read_timeout` setting
	pub fn read_timeout(mut self, timeout: Duration) -> Self {
		self.settings.read_timeout = Some(timeout);
		self
	}

	/// Set `write_timeout` setting
	pub fn write_timeout(mut self, timeout: Duration) -> Self {
		self.settings.write_timeout = Some(timeout);
		self
	}

	/// Set `subdomain` setting
	pub fn subdomain<T>(mut self, subdomain: T) -> Self
		where T: Into<String> {
		self.settings.subdomain = Some(subdomain.into());
		self
	}

	/// Set `username` setting
	pub fn username<T>(mut self, username: T) -> Self
		where T: Into<String> {
		self.settings.username = Some(username.into());
		self
	}

	/// Set `password` setting
	pub fn password<T>(mut self, password: T) -> Self
		where T: Into<String> {
		self.settings.password = Some(password.into());
		self
	}

	/// Set `client_id` setting
	pub fn client_id<T>(mut self, client_id: T) -> Self
		where T: Into<String> {
		self.settings.client_id = Some(client_id.into());
		self
	}

	/// Set `client_secret` setting
	pub fn client_secret<T>(mut self, client_secret: T) -> Self
		where T: Into<String> {
		self.settings.client_secret = Some(client_secret.into());
		self
	}

	/// Creates the configured instance of the `Connection` and tries to connect.
	pub fn connect(self) -> Result<Connection> {
		Connection::configured(self.settings)
			.connect()
	}
}
