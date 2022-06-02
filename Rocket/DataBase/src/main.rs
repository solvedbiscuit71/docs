#[macro_use]
extern crate rocket;

mod cors;
mod models;
mod routes;

use models::Pg;
use rocket::serde::json::{json, Value};
use rocket_db_pools::Database;

#[catch(404)]
fn catch_404() -> Value {
    json!({
        "code": 404,
        "message": "invalid api end-point"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Pg::init())
        .attach(cors::cors())
        .attach(routes::user::stage())
        .attach(routes::product::stage())
        .register("/", catchers![catch_404])
}
