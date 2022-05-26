use crate::handler::{product, user};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(user::config))
        .service(web::scope("/product").configure(product::config));
}
