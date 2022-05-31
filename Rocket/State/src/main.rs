use rocket::State;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicUsize, Ordering};

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_count(count: &State<AtomicUsize>) -> Value {
    let data = count.load(Ordering::Relaxed);

    json!({
        "status": 200,
        "count": data
    })
}

#[post("/")]
fn inc_count(count: &State<AtomicUsize>) -> Value {
    let data = count.load(Ordering::Relaxed);
    count.store(data + 1, Ordering::Relaxed);

    json!({
        "status": 200,
        "message": "incremented count"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_count, inc_count])
        .manage(AtomicUsize::new(0))
}
