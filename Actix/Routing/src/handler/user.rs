use actix_web::{web, Responder};
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

async fn get_users() -> impl Responder {
    web::Json(json!({
        "code": 200,
        "user": [
            "list of user..."
        ]
    }))
}

async fn post_user(new_user: web::Json<User>) -> impl Responder {
    let id = random::<u16>();

    web::Json(json!({
        "code": 200,
        "id": id,
        "user": new_user
    }))
}

async fn get_user_by_id(id: web::Path<u16>) -> impl Responder {
    web::Json(json!({
        "code": 200,
        "id": id.clone(),
        "user": {
            "name": "...",
            "email": "..."
        }
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{id}", web::get().to(get_user_by_id)).service(
        web::resource("")
            .route(web::get().to(get_users))
            .route(web::post().to(post_user)),
    );
}
