use std::io::Read;
use std::sync::Arc;
use hyper::client::{Client, Body};
use hyper::client::response::Response;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, Authorization, Bearer};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json::{self, ser, Value};
use ::{Result, Error};
use super::{ConnectionSettings, ConnectionBuilder, ConnectionHelper};


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


/// Authentication and communication facility  
///
/// `Connection` is used for initial authentication and further access to API Entities.
/// Usual application uses the SDK starts with opening the connection to the REST API.
///
/// The struct has public methods for performing custom HTTP requests and API calls.
/// Most time there is no need to execute those methods manualy and they are used internally
/// by API Entities.
///
/// # Examples
///
/// ```should_panic
/// use shapir::Connection;
/// 
/// let conn = Connection::new()
/// 	.subdomain("your-subdomain")
/// 	.username("your.username@mail.com")
/// 	.password("your-password")
/// 	.client_id("client-id")
/// 	.client_secret("client-secret")
/// 	.connect()
/// 	.unwrap();
/// ```

#[derive(Clone)]
pub struct Connection {
	client: Arc<Client>,
	settings: ConnectionSettings,
	auth: Option<AuthData>,
	endpoint: String,
}


impl Connection {
	/// Create the instance of `ConnectionBuilder` which helps configuring
	/// the connection sesstings.
	pub fn new() -> ConnectionBuilder {
		ConnectionBuilder::new()
	}

	/// Create the configured `Connection` using given settings.
	pub fn configured(settings: ConnectionSettings) -> Connection {
		Connection {
			client: Arc::new(Client::new()),
			settings: settings,
			auth: None,
			endpoint: "".to_string(),
		}
	}

	/// Connect to ShareFile REST API. Behind the scene it does login and obtains authentication tokens
	/// used in all API requests to the API.
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

		let form_data: String = form_urlencoded::Serializer::new(String::new())
			.append_pair("grant_type", "password")
			.append_pair("client_id", &client_id)
			.append_pair("client_secret", &client_secret)
			.append_pair("username", &username)
			.append_pair("password", &password)
			.finish();
		let form_data_len = form_data.len();

		let url = match super::url::to_url(format!("https://{}.sharefile.com/oauth/token", subdomain)) {
			Ok(v) => v,
			Err(err) => return Error::new("Invalid OAuth URL").because(err).result()
		};

		// Try to authenticate on ShareFile
		let response = self.client.request(Method::Post, url)
			.header(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])))
			.body(Body::BufBody(&form_data.into_bytes()[..], form_data_len))
			.send();

		let mut response = match response {
			Ok(response) => response,
			Err(err) => return Error::new("Authentication failed").because(err).result()
		};

		// Parse response into JSON Value and then into AuthData
		let mut json = String::new();
		response.read_to_string(&mut json).unwrap();
		match AuthData::parse_value(serde_json::from_str(&json).unwrap()) {
			Ok(data) => {
				self.endpoint = format!("https://{}.sf-api.com/sf/v3/", data.subdomain);
				self.auth = Some(data);
				Ok(self)
			},
			Err(err) => {
				Err(err)
			}
		}
	}

	/// Perform the low-level custom HTTP request. Hyper's `Response` is returned on success.
	pub fn custom_request(&self, method: Method, url: String, headers: Option<Headers>, body: Option<&[u8]>) -> Result<Response> {
		// Parse URL string into the internal representation
		let url = match super::url::to_url(url) {
			Ok(v) => v,
			Err(err) => return Error::new("Invalid request URL").because(err).result()
		};

		// Build request
		let mut request = self.client.request(method, url);

		if let Some(headers) = headers {
			request = request.headers(headers);
		}

		if let Some(body) = body {
			if body.len() > 0 {
				request = request.body(Body::BufBody(body, body.len()));
			}
		}

		// .. send and unwrap to string
		match request.send() {
			Ok(response) => Ok(response),
			Err(err) => Error::new("Custom request failed").because(err).result()
		}
	}

	/// Perform the call to the API. Hyper's `Response` is returned on success.
	pub fn query(&self, method: Method, uri: String, headers: Option<Headers>, body: Option<String>) -> Result<Response> {
		if let Some(ref auth) = self.auth {
			// Parse URL string into the internal representation
			let url = match super::url::to_url(format!("{}{}", self.endpoint, uri)) {
				Ok(v) => v,
				Err(err) => return Error::new("Invalid request URL").because(err).result()
			};

			// Unwrap body so it lives long enough
			let body = match body {
				Some(data) => data,
				None => "".to_string()
			};

			// Build request
			let mut request = self.client.request(method, url);

			if let Some(headers) = headers {
				request = request.headers(headers);
			}

			request = request.header(Authorization(Bearer { token: auth.access_token.to_owned() }));

			if body.len() > 0 {
				request = request.body(Body::BufBody(body.as_bytes(), body.len()));
			}

			// .. send and unwrap to string
			match request.send() {
				Ok(response) => Ok(response),
				Err(err) => Error::new("Query failed").because(err).result()
			}
		}
		else {
			Error::new("Not authenticated").result()
		}
	}

	/// Perform the call to the API. Response body is returned on success.
	pub fn query_string(&self, method: Method, uri: String, headers: Option<Headers>, body: Option<String>) -> Result<String> {
		self.query(method, uri, headers, body)
			.and_then(|mut response| {
				if response.status.is_success() {
					let mut data = String::new();
					response.read_to_string(&mut data)
						.and(Ok(data))
						.or(Error::new("Failed to read response body").result())
				}
				else {
					Error::new(format!("API request failed with status {}", response.status)).result()
				}
			})
	}

	/// Perform the call to the API which returns JSON. JSON Value is returned on success.
	pub fn query_json(&self, method: Method, uri: String, headers: Option<Headers>, body: Option<Value>) -> Result<Value> {
		let body = body.map(|ref value| ser::to_string(value).unwrap());
		let headers = headers.or(Some(ConnectionHelper::json_headers()));

		self.query_string(method, uri, headers, body)
			.and_then(|json| {
				serde_json::from_str(&json)
					.or_else(|err| Error::new("JSON parse failed").because(err).result()) // Return JSON parsing error
					.and_then(|data| Error::from_json(data)) // Try to parse the error from the response JSON or just pass it through
			})
	}

	/// Get [Items](http://api.sharefile.com/rest/docs/resource.aspx?name=Items) API Entity.
	pub fn items(&self) -> ::api::items::Items {
		::api::items::Items::new(self.clone())
	}

}
