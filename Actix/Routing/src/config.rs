use crate::handler::{product, user};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/{id}", web::get().to(user::get_user_by_id))
            .service(
                web::resource("")
                    .route(web::get().to(user::get_users))
                    .route(web::post().to(user::post_user)),
            ),
    )
    .service(
        web::scope("/product")
            .route("/{id}", web::get().to(product::get_product_by_id))
            .service(
                web::resource("")
                    .route(web::get().to(product::get_products))
                    .route(web::post().to(product::post_product)),
            ),
    );
}
