use rand::random;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
struct Product<'r> {
    name: &'r str,
    quantity: u32,
}

#[get("/<id>")]
fn get_product_by_id(id: u16) -> Value {
    json!({
        "code": 200,
        "id": id,
        "product": {
            "name": "...",
            "quantity": "..."
        }
    })
}

#[get("/")]
fn get_products() -> Value {
    json!({
        "code": 200,
        "product": [
            "list of product..."
        ]
    })
}

#[post("/", data = "<product>")]
fn post_product(product: Json<Product>) -> Value {
    let id = random::<u16>();
    json!({
        "code": 200,
        "id": id,
        "product": {
            "name": product.name,
            "quantity": product.quantity
        }
    })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("product", |rocket| async {
        rocket.mount(
            "/product",
            routes![get_product_by_id, get_products, post_product],
        )
    })
}
