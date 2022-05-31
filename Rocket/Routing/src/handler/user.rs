use rand::random;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
struct User<'r> {
    name: &'r str,
    email: &'r str,
}

#[get("/<id>")]
fn get_user_by_id(id: u16) -> Value {
    json!({
        "code": 200,
        "id": id,
        "user": {
            "name": "...",
            "email": "..."
        }
    })
}

#[get("/")]
fn get_users() -> Value {
    json!({
        "code": 200,
        "user": [
            "list of user..."
        ]
    })
}

#[post("/", data = "<user>")]
fn post_user(user: Json<User>) -> Value {
    let id = random::<u16>();
    json!({
        "code": 200,
        "id": id,
        "user": {
            "name": user.name,
            "email": user.email
        }
    })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("user", |rocket| async {
        rocket.mount("/user", routes![get_user_by_id, get_users, post_user])
    })
}
