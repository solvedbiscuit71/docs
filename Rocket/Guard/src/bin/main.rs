use guard::router::sensitive;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(sensitive::stage())
}
