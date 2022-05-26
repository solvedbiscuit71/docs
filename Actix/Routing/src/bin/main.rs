/*
 * Routing:
 *
 * GET /user
 * POST /user
 * GET /user/id
 *
 * GET /product
 * POST /prodcut
 * GET /prodcut/id
 */

use actix_web::{App, HttpServer};
use routing::config::config_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    HttpServer::new(|| App::new().configure(config_routes))
        .bind(("localhost", PORT))?
        .run()
        .await
}
