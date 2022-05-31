#[macro_use]
extern crate rocket;

use routing::handler::{product, user};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(user::stage())
        .attach(product::stage())
}
