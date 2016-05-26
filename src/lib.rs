//! Unofficial [ShareFile REST API](http://api.sharefile.com/rest/) SDK. The SKD tries to follow
//! the composition of the REST API so with the `Connection` opened you can access API Entities
//! described in the official documentation.  
//!
//! The library utilizes:
//!
//! - [hyper](https://crates.io/crates/hyper) for all low-level HTTP requests.
//! - [serde_json](https://crates.io/crates/serde_json) as JSON encoding/decoding facility.
//! - And other great crates.

extern crate serde_json;
extern crate hyper;
extern crate url;
extern crate chrono;
extern crate md5;
extern crate rustc_serialize;


pub mod error;
pub mod connection;
pub mod odata;
pub mod api;


pub use connection::{Connection, ConnectionBuilder, ConnectionSettings};
pub use error::{Error, Result};
