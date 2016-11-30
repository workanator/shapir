use serde_json::{self, Value};


/// User ID
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UserId {
    /// The user is identified by ID
    Id(String),
    /// The user is identified by e-mail address
    Email(String),
}


impl UserId {
    /// Create user ID from `id` given
    pub fn from_id(id: String) -> UserId {
        UserId::Id(id)
    }

    /// Create user ID from `email` given
    pub fn from_email(email: String) -> UserId {
        UserId::Email(email)
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
            &UserId::Email(ref email) => email.clone(),
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
            &UserId::Email(ref email) => {
                object.insert("Email".to_owned(), Value::String(email.clone()));
            }
        };

        Value::Object(object)
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
