use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Database};

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

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub name: String,
    pub price: i32,
    pub user_id: i32,
}

impl Product {
    pub fn new(name: Option<String>, price: Option<i32>, user_id: Option<i32>) -> Self {
        Self {
            name: name.unwrap_or("undefined".to_string()),
            price: price.unwrap(),
            user_id: user_id.unwrap(),
        }
    }
}

pub type UserType = Json<User>;
pub type ProductType = Json<Product>;

#[derive(Database)]
#[database("rocket_example")]
pub struct Pg(sqlx::PgPool);
