use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

/* ---- crates ---- */

use todo_service::models::Pg;
use todo_service::router::{group, todo};

/* ---- launch 🚀 ---- */

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Pg::init())
        .attach(group::stage())
        .attach(todo::stage())
}
