//! Shares Entity


mod kind;
mod access_right;
mod config;
mod share;


use hyper::method::Method;
use serde_json::value::ToJson;
use ::connection::Connection;
use ::odata::Parameters;
use ::Result;


pub use self::kind::Kind;
pub use self::access_right::AccessRight;
pub use self::config::ShareConfig;
pub use self::share::Share;


/// Shares Entity implementation.
///
/// Shares struct implemets methods of [Shares API Entity](http://api.sharefile.com/rest/docs/resource.aspx?name=Shares)
pub struct Shares {
    conn: Connection,
}


impl Shares {
    /// Create a new instance of Share Entities API
    pub fn new(conn: Connection) -> Self {
        Shares {
            conn: conn,
        }
    }

    /// Create a new share from the configuration given. 
    pub fn create(&self, config: &ShareConfig, notify: bool) -> Result<Share> {
        // Prepare request body
        let body = config.to_json();

        // Prepare additional parameters
        let parameters = Parameters::new()
            .custom(vec![ ("notify", super::bool_to_string(notify)) ]);

        // Prepare URL
        let url = format!("Shares?{}", parameters.to_string());

        // Send the request to the API to create the share
        self.conn.query_json(Method::Post, url, None, Some(body))
            .and_then(|v| Share::from_json(v))
    }
}
