use serde_json::Value;
use ::Result;


/// Share Details
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Share;


impl Share {
    /// Construct share from the decoded JSON value.
    pub fn from_json(value: Value) -> Result<Share> {
        println!("SHARE {:?}", value);
        Ok(Share {})
    }
}
