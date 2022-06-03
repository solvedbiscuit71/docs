use crate::models::{Pg, Todo};
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, put};
use rocket::{fairing::AdHoc, post, routes};
use rocket_db_pools::Connection;

use super::group::check_for_uuid;

#[post("/", data = "<todo>")]
async fn create_todo(mut db: Connection<Pg>, todo: Json<Todo>) -> Value {
    if todo.body.is_none() || todo.is_checked.is_none() || todo.group_id.is_none() {
        return json!({
            "status": 300,
            "message": "missing arguments (body, is_checked, group_id)"
        });
    }
    let id = uuid::Uuid::new_v4().to_string();
    let res = sqlx::query!(
        "INSERT INTO todos VALUES ($1, $2, $3, $4)",
        id,
        todo.body,
        todo.is_checked,
        todo.group_id
    )
    .execute(&mut *db)
    .await;

    match res {
        Ok(res) => {
            json!({
                "status": 200,
                "id": id,
                "rows_affected": res.rows_affected()
            })
        }
        Err(err) => {
            json!({
                "status": 400,
                "error": err.to_string()
            })
        }
    }
}

#[get("/")]
async fn read_all_todos(mut db: Connection<Pg>) -> Value {
    let todos = sqlx::query!("SELECT * FROM todos")
        .fetch_all(&mut *db)
        .await
        .map(|recs| {
            let mut todos = vec![];
            recs.into_iter().for_each(|rec| {
                todos.push(Todo {
                    id: Some(rec.id),
                    body: Some(rec.body),
                    is_checked: Some(rec.is_checked),
                    group_id: Some(rec.group_id),
                })
            });
            todos
        })
        .ok();

    json!({
        "status": 200,
        "todos": todos
    })
}

#[get("/?<group_id>")]
async fn read_todo_by_group_id(mut db: Connection<Pg>, group_id: &str) -> Value {
    if !check_for_uuid(group_id) {
        return json!({
            "status": 300,
            "message": "invalid group_id"
        });
    }

    let todos = sqlx::query!("SELECT * FROM todos where group_id = $1", group_id)
        .fetch_all(&mut *db)
        .await
        .map(|recs| {
            let mut todos = vec![];
            recs.into_iter().for_each(|rec| {
                todos.push(Todo {
                    id: Some(rec.id),
                    body: Some(rec.body),
                    is_checked: Some(rec.is_checked),
                    group_id: Some(rec.group_id),
                })
            });
            todos
        })
        .ok();

    json!({
        "status": 200,
        "todos": todos
    })
}

#[put("/<id>", data = "<todo>")]
async fn update_todo(mut db: Connection<Pg>, id: &str, todo: Json<Todo>) -> Value {
    if !check_for_uuid(id) {
        return json!({
            "status": 300,
            "message": "invalid id"
        });
    }

    if todo.body.is_none() || todo.is_checked.is_none() {
        return json!({
            "status": 300,
            "message": "missing arguments (body, is_checked)"
        });
    }

    let res = sqlx::query!(
        "update todos set body = $1, is_checked = $2 where id = $3",
        todo.body,
        todo.is_checked,
        id
    )
    .execute(&mut *db)
    .await;

    match res {
        Ok(res) => {
            json!({
                "status": 200,
                "id": id,
                "rows_affected": res.rows_affected()
            })
        }
        Err(err) => {
            json!({
                "status": 400,
                "error": err.to_string()
            })
        }
    }
}

#[delete("/<id>")]
async fn delete_todo(mut db: Connection<Pg>, id: &str) -> Value {
    if !check_for_uuid(id) {
        return json!({
            "status": 300,
            "message": "invalid id"
        });
    }

    let res = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&mut *db)
        .await;

    match res {
        Ok(res) => {
            json!({
                "status": 200,
                "id": id,
                "rows_affected": res.rows_affected()
            })
        }
        Err(err) => {
            json!({
                "status": 400,
                "error": err.to_string()
            })
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("todo", |rocket| async {
        rocket.mount(
            "/todo",
            routes![
                create_todo,
                read_all_todos,
                read_todo_by_group_id,
                update_todo,
                delete_todo
            ],
        )
    })
}
