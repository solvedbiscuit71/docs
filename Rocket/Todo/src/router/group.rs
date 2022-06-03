use crate::models::{Group, Pg};
use rocket::{
    fairing::AdHoc,
    routes,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

use rocket::{delete, get, post};

#[post("/", data = "<group>")]
async fn create_group(mut db: Connection<Pg>, group: Json<Group>) -> Value {
    if group.name.is_none() || group.desc.is_none() {
        return json!({
            "status": 300,
            "message": "missing arguments (name, desc)"
        });
    }
    let id = uuid::Uuid::new_v4().to_string();
    let name = group.name.clone().unwrap();
    let desc = group.desc.clone().unwrap();

    let res = sqlx::query!("INSERT INTO groups VALUES ($1, $2, $3)", id, name, desc)
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
async fn read_groups(mut db: Connection<Pg>) -> Value {
    let groups = sqlx::query!("SELECT * FROM groups")
        .fetch_all(&mut *db)
        .await
        .map(|res| {
            let mut groups = vec![];
            for rec in res.into_iter() {
                groups.push(Group {
                    id: Some(rec.id),
                    name: Some(rec.name),
                    desc: Some(rec.desc),
                })
            }
            groups
        })
        .ok();

    json!({
        "status": 200,
        "groups": groups
    })
}

fn check_for_uuid(id: &str) -> bool {
    if id.len() != 36 {
        return false;
    }

    if id[8..9] != *"-" || id[13..14] != *"-" || id[18..19] != *"-" || id[23..24] != *"-" {
        return false;
    }

    true
}

#[delete("/<id>")]
async fn delete_group_by_id(mut db: Connection<Pg>, id: &str) -> Value {
    if !check_for_uuid(id) {
        return json!({
            "status": 300,
            "message": "invalid id"
        });
    }

    let res = sqlx::query!("DELETE FROM groups WHERE id = $1", id)
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
    AdHoc::on_ignite("group", |rocket| async {
        rocket.mount(
            "/group",
            routes![create_group, read_groups, delete_group_by_id],
        )
    })
}
