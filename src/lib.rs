extern crate serde_json;
extern crate hyper;
extern crate url;


pub mod error;
pub mod connection;
pub mod odata;
pub mod api;


pub use connection::{Connection, ConnectionBuilder, ConnectionSettings};
pub use error::{Error, Result};
