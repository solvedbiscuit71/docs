use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .show_files_listing() lists all files in static/images and send an HTML responds
            .service(Files::new("/images", "static/images").show_files_listing())
            .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
