extern crate serde_json;
extern crate hyper;
extern crate url;

mod connection;


pub use connection::Connection;
pub use connection::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
