use rocket::http::{Cookie, CookieJar};

#[macro_use]
extern crate rocket;

/*
 * Private Cookies (encryption by secret_key)
 * avaiable in 'secrets' features only
 *
 * Support:
 * get_private()
 * add_private()
 * remove_private()
 *
 * More on secret_key:
 * 1. it's a 256-bit key
 * 2. automatically generated if not set!
 */

#[post("/")]
fn set_id(jar: &CookieJar) -> &'static str {
    let id: u16 = 12345;
    jar.add(Cookie::new("id", id.to_string()));

    "Id has been set"
}

#[get("/")]
fn get_id(jar: &CookieJar) -> &'static str {
    let id = jar.get("id").map(|cookies| cookies.value());
    match id {
        Some(id) if id == "12345" => "Auth Success",
        Some(_) => "Auth Failed",
        None => "Login Please",
    }
}

#[delete("/")]
fn delete_id(jar: &CookieJar) -> &'static str {
    jar.remove(Cookie::named("id"));
    "Id has been removed"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/login", routes![set_id, get_id, delete_id])
}
