use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn greet() -> &'static str {
    "Hello World!"
}

async fn echo(input: web::Path<String>) -> impl Responder {
    format!("Echo: {input}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
     * HttpServer() -> is a factory i.e create this closure for individual threads!
     * App() -> is create for every thread so, app state is valid in that thread only.
     */

    HttpServer::new(|| {
        App::new()
            .service(greet) // -> greet() at GET "/"
            .service(web::resource("/echo/{input}").route(web::get().to(echo))) // -> echo() at GET "/echo/:input"
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
