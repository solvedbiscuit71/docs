use actix_web::{web, Responder};
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
}

pub async fn get_users() -> impl Responder {
    web::Json(json!({
        "code": 200,
        "user": [
            "list of user..."
        ]
    }))
}

pub async fn post_user(new_user: web::Json<User>) -> impl Responder {
    let id = random::<u16>();

    web::Json(json!({
        "code": 200,
        "id": id,
        "user": new_user
    }))
}

pub async fn get_user_by_id(id: web::Path<u16>) -> impl Responder {
    web::Json(json!({
        "code": 200,
        "id": id.clone(),
        "user": {
            "name": "...",
            "email": "..."
        }
    }))
}
