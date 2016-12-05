use serde_json::Value;
use ::Result;


/// Share Access Right
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessRight {
	/// The owner of the share have the full control over the share
	///
	/// The detailed information about ShareAccess Right in ShareFile REST API docs is missing
	/// so lets assume access right is always FullControl until the more information become
	/// available.
	FullControl,
}


impl AccessRight {
    /// Construct access right from the decoded JSON value.
    pub fn from_json(_value: Value) -> Result<AccessRight> {
        Ok(AccessRight::FullControl)
    }

    /// Test if access right is full control
    pub fn is_full_control(&self) -> bool {
    	true
    }
}
