#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/echo/<input>")]
fn echo(input: &str) -> String {
    format!("Echo: {}", input)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, echo])
}
