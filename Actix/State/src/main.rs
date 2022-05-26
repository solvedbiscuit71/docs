use std::{cell::Cell, sync::Mutex};

use actix_web::web::{self, Data};
use actix_web::{get, post, App, HttpServer, Responder};
use serde_json::json;

/*
 * For global shared state, we wrap our state in a [`actix_web::web::Data`] and move it into
 * the factory closure. The closure is called once-per-thread, and we clone our state
 * and attach to each instance of the [`App`] with `.app_data(state.clone())`.
 *
 * For thread-local state, we construct our state within the factory closure and attach to
 * the app with `.app_data(Data::new(state))`.
 */

#[post("/")]
async fn inc_count(local: Data<Cell<i32>>, global: Data<Mutex<i32>>) -> impl Responder {
    let count = local.get();
    local.set(count + 1);

    let mut counter = global.lock().unwrap();
    *counter += 1;

    web::Json(json!({
        "status": 200,
        "message": "incremented count"
    }))
}

#[get("/")]
async fn get_count(local: Data<Cell<i32>>, global: Data<Mutex<i32>>) -> impl Responder {
    let local_count = local.get();

    let counter = global.lock().unwrap();
    let global_count = *counter;

    web::Json(json!({
        "status": 200,
        "global_count": global_count,
        "local_count": local_count
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let global_state = Data::new(Mutex::new(0));

    HttpServer::new(move || {
        let local_state = Cell::new(0);
        App::new()
            .app_data(global_state.clone())
            .app_data(Data::new(local_state))
            .service(get_count)
            .service(inc_count)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
