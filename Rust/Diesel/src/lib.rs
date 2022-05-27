#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn connect_to_postgres() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL in environment");
    PgConnection::establish(&url).expect("Couldn't connect to DataBase")
}
