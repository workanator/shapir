//! Error code and message returned by API response


use std::fmt;
use std::error;
use serde_json::Value;


#[derive(Debug)]
pub struct ServiceError {
    code: Option<String>,
    message: String,
}


impl ServiceError {
    /// Construct new service error
    pub fn new<T: Into<String>>(code: Option<T>, message: T) -> ServiceError {
        ServiceError {
            code: code.map(|c| c.into()),
            message: message.into(),
        }
    }

    /// Try to construct the service error from JSON
    pub fn from_json(value: &Value) -> Option<ServiceError> {
        value.find("code")
            .and_then(|code| {
                // Error code
                let code = code.as_string()
                    .unwrap();

                // Error message
                let message = value.find("message")
                    .unwrap()
                    .find("value")
                    .unwrap()
                    .as_string()
                    .unwrap();

                Some(ServiceError {
                    code: Some(String::from(code)),
                    message: String::from(message),
                })
            })
    }

    /// Get error code
    pub fn code(&self) -> &Option<String> {
        &self.code
    }

    /// Get error message
    pub fn message(&self) -> &String {
        &self.message
    }
}


impl error::Error for ServiceError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            Some(ref code) => write!(f, "{}: {}", code, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}
