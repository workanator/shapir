pub mod error;

use std;
use std::io::Read;
use hyper::Url;
use hyper::client::{Client, Body};
use hyper::method::Method;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use url::ParseError;
use serde_json;
use serde_json::Value;
use ::{Result, Error};

pub struct Connection {
	client: Client,
	subdomain: Option<String>,
	username: Option<String>,
	password: Option<String>,
	client_id: Option<String>,
	client_secret: Option<String>,
	access_token: Option<Value>,
}


impl Connection {
	pub fn new() -> Connection {
		Connection {
			client: Client::new(),
			subdomain: None,
			username: None,
			password: None,
			client_id: None,
			client_secret: None,
			access_token: None,
		}
	}

	pub fn subdomain<T>(mut self, subdomain: T) -> Connection
		where T: Into<String> {
		self.subdomain = Some(subdomain.into());
		self
	}

	pub fn username<T>(mut self, username: T) -> Connection
		where T: Into<String> {
		self.username = Some(username.into());
		self
	}

	pub fn password<T>(mut self, password: T) -> Connection
		where T: Into<String> {
		self.password = Some(password.into());
		self
	}

	pub fn client_id<T>(mut self, client_id: T) -> Connection
		where T: Into<String> {
		self.client_id = Some(client_id.into());
		self
	}

	pub fn client_secret<T>(mut self, client_secret: T) -> Connection
		where T: Into<String> {
		self.client_secret = Some(client_secret.into());
		self
	}

	pub fn connect(mut self) -> Result<Connection> {
		let subdomain = match &self.subdomain {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Subdomain is required").result()
		};

		let username = match &self.username {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Username is required").result()
		};

		let password = match &self.password {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Password is required").result()
		};

		let client_id = match &self.client_id {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Client ID is required").result()
		};

		let client_secret = match &self.client_secret {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Client Secret is required").result()
		};

		let body = format!("grant_type=password&client_id={}&client_secret={}&username={}&password={}",
			client_id,
			client_secret,
			username,
			password);

		let url = match Connection::to_url(format!("https://{}.sharefile.com/oauth/token", subdomain)) {
			Ok(v) => v,
			Err(err) => return Error::new("Invalid OAuth URL").because(err).result()
		};

		let mut response = self.client.request(Method::Post, url)
			.header(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])))
			.body(Body::BufBody(body.as_bytes(), body.len()))
			.send()
			.unwrap();

		let mut json = String::new();
    	response.read_to_string(&mut json).unwrap();
		let data: Value = serde_json::from_str(&json).unwrap();

		if let Some(ref error) = data.lookup("error") {
			return Error::new(format!("{:?}", error)).result();
		};

		self.access_token = Some(data);
		Ok(self)
	}

	fn to_url(url: String) -> std::result::Result<Url, ParseError> {
		Url::parse(&url)
	}
}


#[cfg(test)]
mod tests {
	use super::Connection;

	#[test]
	fn new_connection() {
		let _ = Connection::new()
			.subdomain("webbula")
			.username("tcms@webbula.com")
			.password("passpass")
			.client_id("123")
			.client_secret("asdas")
			.connect();
	}
}