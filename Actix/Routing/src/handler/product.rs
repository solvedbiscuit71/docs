use actix_web::{web, Responder};
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct Product {
    name: String,
    quantity: u32,
}

async fn get_products() -> impl Responder {
    web::Json(json!({
        "code": 200,
        "product": [
            "list of product..."
        ]
    }))
}

async fn post_product(new_product: web::Json<Product>) -> impl Responder {
    let id = random::<u16>();

    web::Json(json!({
        "code": 200,
        "id": id,
        "product": new_product
    }))
}

async fn get_product_by_id(id: web::Path<u16>) -> impl Responder {
    web::Json(json!({
        "code": 200,
        "id": id.clone(),
        "product": {
            "name": "...",
            "quantity": "..."
        }
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{id}", web::get().to(get_product_by_id))
        .service(
            web::resource("")
                .route(web::get().to(get_products))
                .route(web::post().to(post_product)),
        );
}
