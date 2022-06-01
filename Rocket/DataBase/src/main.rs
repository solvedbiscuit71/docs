#[macro_use]
extern crate rocket;

use rocket::{
    futures::TryFutureExt,
    response::{status::Created, Responder},
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::{sqlx, Connection, Database};

#[derive(Database)]
#[database("rocket_example")]
struct Pg(sqlx::PgPool);

mod models;
use models::{User, UserType};

#[post("/", data = "<user>")]
async fn create_user(mut db: Connection<Pg>, user: UserType) -> Option<Created<UserType>> {
    let args = user.clone();
    sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        args.name,
        args.email
    )
    .execute(&mut *db)
    .await
    .ok() // converts Result<T,E> to Option<T>
    .map(|_| Created::new("user").body(args))
}

#[get("/<id>")]
async fn get_user_by_id(mut db: Connection<Pg>, id: i32) -> Option<UserType> {
    sqlx::query!("SELECT name, email FROM users where id = $1", id)
        .fetch_one(&mut *db)
        .map_ok(|row| Json(User::new(row.name, row.email)))
        .await
        .ok()
}

#[put("/<id>", data = "<user>")]
async fn update_user<'r>(
    mut db: Connection<Pg>,
    id: i32,
    user: UserType,
) -> impl Responder<'r, 'static> {
    let res = sqlx::query!(
        "UPDATE users SET name = $1,email = $2 where id = $3",
        user.name,
        user.email,
        id
    )
    .execute(&mut *db)
    .await
    .ok();

    match res {
        Some(outcome) => {
            if outcome.rows_affected() == 0 {
                json!({
                    "code": 404,
                    "message": "id not found"
                })
            } else {
                json!({
                    "code": 200,
                    "message": "successfully updated"
                })
            }
        }
        None => json!({
            "code": 500,
            "message": "updated failed"
        }),
    }
}

#[delete("/<id>")]
async fn delete_user_by_id(mut db: Connection<Pg>, id: i32) -> Value {
    let res = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&mut *db)
        .await
        .ok();

    match res {
        Some(outcome) => {
            if outcome.rows_affected() == 0 {
                json!({
                    "code": 404,
                    "message": "id not found"
                })
            } else {
                json!({
                    "code": 200,
                    "message": "successfully deleted"
                })
            }
        }
        None => json!({
            "code": 500,
            "message": "deletion failed"
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Pg::init()).mount(
        "/user",
        routes![create_user, get_user_by_id, update_user, delete_user_by_id],
    )
}
