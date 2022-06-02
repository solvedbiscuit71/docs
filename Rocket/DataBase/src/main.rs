#[macro_use]
extern crate rocket;

mod models;
mod routes;

use models::Pg;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Pg::init())
        .attach(routes::user::stage())
}
