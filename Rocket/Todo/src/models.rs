use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("todo_service")]
pub struct Pg(sqlx::PgPool);

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
}
