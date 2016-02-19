use std::io::Read;
use std::sync::Arc;
use hyper::client::{Client, Body};
use hyper::method::Method;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;
use serde_json::Value;
use ::{Result, Error};
use super::{ConnectionSettings, ConnectionBuilder};


#[derive(Debug, Clone)]
struct AuthData {
	subdomain: String,
	token_type: String,
	access_token: String,
	refresh_token: String,
}

impl AuthData {
	fn parse_value(value: Value) -> Result<AuthData> {
		// Test if the value contains error
		if let Some(ref error) = value.lookup("error") {
			return Error::new(format!("{:?}", error)).result();
		};

		// Get auth values
		let subdomain = match value.find("subdomain") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Auth Subdomain is missing.").result()
		};

		let token_type = match value.find("token_type") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Auth Token Type is missing.").result()
		};

		let access_token = match value.find("access_token") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Auth Access Token is missing.").result()
		};

		let refresh_token = match value.find("refresh_token") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("Auth Refresh Token is missing.").result()
		};

		// Fill in auth struct
		Ok(AuthData {
			subdomain: subdomain.to_string(),
			token_type: token_type.to_string(),
			access_token: access_token.to_string(),
			refresh_token: refresh_token.to_string(),
		})
	}
}


#[derive(Clone)]
pub struct Connection {
	client: Arc<Client>,
	settings: ConnectionSettings,
	access_token: Option<AuthData>,
}


impl Connection {
	pub fn new() -> ConnectionBuilder {
		ConnectionBuilder::new()
	}

	pub fn configured(settings: ConnectionSettings) -> Connection {
		Connection {
			client: Arc::new(Client::new()),
			settings: settings,
			access_token: None,
		}
	}

	pub fn connect(mut self) -> Result<Connection> {
		use url::form_urlencoded;

		// Prepare authentication request body and URL
		let subdomain = match &self.settings.subdomain {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Subdomain is required").result()
		};

		let username = match &self.settings.username {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Username is required").result()
		};

		let password = match &self.settings.password {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Password is required").result()
		};

		let client_id = match &self.settings.client_id {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Client ID is required").result()
		};

		let client_secret = match &self.settings.client_secret {
			&Some(ref v) => v.clone(),
			&None => return Error::new("Client Secret is required").result()
		};

		let mut form_data: Vec<(&str, String)> = Vec::new();
		form_data.push(("grant_type", "password".to_string()));
		form_data.push(("client_id", client_id));
		form_data.push(("client_secret", client_secret));
		form_data.push(("username", username));
		form_data.push(("password", password));

		let body = form_urlencoded:: serialize(form_data);

		let url = match super::url::to_url(format!("https://{}.sharefile.com/oauth/token", subdomain)) {
			Ok(v) => v,
			Err(err) => return Error::new("Invalid OAuth URL").because(err).result()
		};

		// Try to authenticate on ShareFile
		let mut response = self.client.request(Method::Post, url)
			.header(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])))
			.body(Body::BufBody(body.as_bytes(), body.len()))
			.send()
			.unwrap();

		// Parse response into JSON Value and then into AuthData
		let mut json = String::new();
		response.read_to_string(&mut json).unwrap();
		match AuthData::parse_value(serde_json::from_str(&json).unwrap()) {
			Ok(data) => {
				self.access_token = Some(data);
				Ok(self)
			},
			Err(err) => {
				Err(err)
			}
		}
	}
}
