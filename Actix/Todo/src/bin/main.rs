use actix_web::{App, HttpServer};
use todo::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = config::Env::new();

    HttpServer::new(|| App::new())
        .bind((env.host, env.port))?
        .run()
        .await
}
