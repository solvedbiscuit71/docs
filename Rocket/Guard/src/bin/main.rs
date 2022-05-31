use guard::ApiKey;

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_sensitive(key: ApiKey) -> &'static str {
    if key.0 == "jamesbond" {
        "Welcome, Mr.bond"
    } else {
        "Hello, Agent"
    }
}

#[catch(403)]
fn catch_403() -> &'static str {
    "Missing Key 'api-key'"
}

#[catch(400)]
fn catch_400() -> &'static str {
    "Invalid key 'api-key'"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/sensitive", routes![get_sensitive])
        .register("/sensitive", catchers![catch_400, catch_403])
}
