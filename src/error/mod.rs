//! Error type used in the library


mod service_error;


use std::{self, fmt};
use std::error::Error as StdError;
use serde_json::Value;

pub use std::io::Error as IoError;
pub use std::io::ErrorKind as IoErrorKind;
pub use hyper::error::Error as NetworkError;
pub use url::ParseError as UrlError;
pub use serde_json::error::Error as JsonError;
pub use self::service_error::ServiceError;
pub type Result<T> = std::result::Result<T, Error>;


/// Error type
#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Network(NetworkError),
    Url(UrlError),
    Json(JsonError),
    Service(ServiceError),
    Other(String),
}


impl Error {
    /// Construct `Result` type with IO error contained.
    pub fn io_result<T>(err: IoError) -> Result<T> {
        Err(Error::from(err))
    }

    /// Construct `Result` type with network error contained.
    pub fn network_result<T>(err: NetworkError) -> Result<T> {
        Err(Error::from(err))
    }

    /// Construct `Result` type with URL error contained.
    pub fn url_result<T>(err: UrlError) -> Result<T> {
        Err(Error::from(err))
    }

    /// Construct `Result` type with JSON error contained.
    pub fn json_result<T>(err: JsonError) -> Result<T> {
        Err(Error::from(err))
    }

    /// Construct `Result` type with Service error contained.
    pub fn service_result<T>(err: ServiceError) -> Result<T> {
        Err(Error::from(err))
    }

    /// Construct `Result` type with other (String) error contained.
    pub fn other_result<S: Into<String>, T>(err: S) -> Result<T> {
        Err(Error::from(err.into()))
    }

    // Parse the error from the result JSON
    pub fn from_json(value: Value) -> Result<Value> {
        ServiceError::from_json(&value)
            .map_or(Ok(value), |e| Err(Error::from(e)))
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Io(ref err) => err.description(),
            &Error::Network(ref err) => err.description(),
            &Error::Url(ref err) => err.description(),
            &Error::Json(ref err) => err.description(),
            &Error::Service(ref err) => err.description(),
            &Error::Other(ref msg) => &msg,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            &Error::Io(ref err) => err.cause(),
            &Error::Network(ref err) => err.cause(),
            &Error::Url(ref err) => err.cause(),
            &Error::Json(ref err) => err.cause(),
            &Error::Service(ref err) => err.cause(),
            &Error::Other(_) => None,
        }
    }
}


impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}


impl From<NetworkError> for Error {
    fn from(err: NetworkError) -> Error {
        Error::Network(err)
    }
}


impl From<UrlError> for Error {
    fn from(err: UrlError) -> Error {
        Error::Url(err)
    }
}


impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}


impl From<ServiceError> for Error {
    fn from(err: ServiceError) -> Error {
        Error::Service(err)
    }
}


impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}


impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::Other(String::from(err))
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Io(ref err) => fmt::Display::fmt(err, f),
            &Error::Network(ref err) => fmt::Display::fmt(err, f),
            &Error::Url(ref err) => fmt::Display::fmt(err, f),
            &Error::Json(ref err) => fmt::Display::fmt(err, f),
            &Error::Service(ref err) => fmt::Display::fmt(err, f),
            &Error::Other(ref msg) => write!(f, "{}", msg),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Error;
    use serde_json::{self, Value};

    #[test]
    fn parse_error_json() {
        let data: Value = serde_json::from_str("{\"code\":\"BadRequest\",\"message\":{\"lang\":\"en-US\",\"value\":\"Invalid Argument Items.Folder\"},\"reason\":\"BadRequest\"}").unwrap();
        let error = Error::from_json(data);
        assert!(error.is_err());
    }

    #[test]
    fn parse_success_json() {
        let data: Value = serde_json::from_str("{\"Id\":\"some-file-id\"}").unwrap();
        let error = Error::from_json(data);
        assert!(error.is_ok());
    }
}
