use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

/* ---- crates ---- */

use todo::models::Pg;
use todo::router::group;

/* ---- launch 🚀 ---- */

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Pg::init()).attach(group::stage())
}
