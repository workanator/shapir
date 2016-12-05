use std::cmp::{PartialOrd, Ord, Ordering};
use serde_json::{self, Value};
use email::Mailbox;


/// User ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserId {
    /// The user is identified by ID
    Id(String),
    /// The user is identified by e-mail address
    Email(Mailbox),
}


impl UserId {
    /// Create user ID from `id` given.
    pub fn from_id<T>(id: T) -> UserId
    where T: Into<String> {
        UserId::Id(id.into())
    }

    /// Create user ID from `email` given.
    pub fn from_email<T>(email: T) -> UserId
    where T: Into<String> {
        UserId::Email(Mailbox::new(email.into()))
    }

    /// Create user ID from `mailbox` given.
    pub fn from_mailbox(mailbox: Mailbox) -> UserId {
        UserId::Email(mailbox)
    }

    /// Get ID if the user is identified by ID or empty string otherwise.
    pub fn id(&self) -> String {
        match self {
            &UserId::Id(ref id) => id.clone(),
            _ => String::new(),
        }
    }

    /// Get e-mail address if the user is identified by e-mail address or empty string otherwise.
    pub fn email(&self) -> String {
        match self {
            &UserId::Email(ref mailbox) => mailbox.address.clone(),
            _ => String::new(),
        }
    }

    /// Test if the user identified by ID
    pub fn is_id(&self) -> bool {
        match self {
            &UserId::Id(_) => true,
            _ => false,
        }
    }

    /// Test if the user identified by e-mail address
    pub fn is_email(&self) -> bool {
        match self {
            &UserId::Email(_) => true,
            _ => false,
        }
    }
}


// Convert User ID into JSON Value
impl serde_json::value::ToJson for UserId {
    fn to_json(&self) -> Value {
        use serde_json::value::Map;
        let mut object: Map<String, Value> = Map::new();

        match self {
            &UserId::Id(ref id) => {
                object.insert("Id".to_owned(), Value::String(id.clone()));
            },
            &UserId::Email(ref mailbox) => {
                object.insert("Email".to_owned(), Value::String(mailbox.address.clone()));
            }
        };

        Value::Object(object)
    }
}


// Implement PartialOrd for UserId
impl PartialOrd for UserId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            &UserId::Id(ref id) => match other {
                &UserId::Id(ref other_id) => Some(id.cmp(other_id)),
                _ => None
            },
            &UserId::Email(ref mailbox) => match other {
                &UserId::Email(ref other_mailbox) => Some(mailbox.address.to_lowercase().cmp(&other_mailbox.address.to_lowercase())),
                _ => None
            }
        }
    }
}


// Implement Ord for UserId
impl Ord for UserId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            &UserId::Id(ref id) => match other {
                &UserId::Id(ref other_id) => id.cmp(other_id),
                _ => Ordering::Less,
            },
            &UserId::Email(ref mailbox) => match other {
                &UserId::Email(ref other_mailbox) => mailbox.address.to_lowercase().cmp(&other_mailbox.address.to_lowercase()),
                _ => Ordering::Greater,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::UserId;
    use serde_json::value::ToJson;

    #[test]
    fn user_id_from_id() {
        let user = UserId::from_id(String::from("ID!"));
        assert_eq!(user.is_id(), true);
        assert_eq!(user.id(), String::from("ID!"));
        assert_eq!(user.is_email(), false);
        assert_eq!(user.email(), String::new());
    }

    #[test]
    fn user_by_id_to_json() {
        let user = UserId::from_id(String::from("ID!"));
        let value = user.to_json();
        assert!(value.is_object());

        if let Some(object) = value.as_object() {
            match object.get("Id") {
                Some(id) => {
                    assert!(id.is_string());
                    if let Some(s) = id.as_str() {
                        assert_eq!(s, "ID!");
                    }
                    else {
                        panic!("Cannot get contained value");
                    }
                },
                None => panic!("Must contain Id property")
            }
        }
        else {
            panic!("Cannot get contained object");
        }
    }

    #[test]
    fn user_id_from_email() {
        let user = UserId::from_email(String::from("email@address.com"));
        assert_eq!(user.is_id(), false);
        assert_eq!(user.id(), String::new());
        assert_eq!(user.is_email(), true);
        assert_eq!(user.email(), String::from("email@address.com"));
    }

    #[test]
    fn user_by_email_to_json() {
        let user = UserId::from_email(String::from("email@address.com"));
        let value = user.to_json();
        assert!(value.is_object());

        if let Some(object) = value.as_object() {
            match object.get("Email") {
                Some(email) => {
                    assert!(email.is_string());
                    if let Some(s) = email.as_str() {
                        assert_eq!(s, "email@address.com");
                    }
                    else {
                        panic!("Cannot get contained value");
                    }
                },
                None => panic!("Must contain Id property")
            }
        }
        else {
            panic!("Cannot get contained object");
        }
    }
}
