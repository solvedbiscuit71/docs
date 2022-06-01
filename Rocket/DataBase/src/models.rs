use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: Option<String>, email: Option<String>) -> Self {
        let name = match name {
            Some(name) => name,
            None => "undefined".to_owned(),
        };
        let email = match email {
            Some(email) => email,
            None => "undefined".to_owned(),
        };
        Self { name, email }
    }
}

pub type UserType = Json<User>;
