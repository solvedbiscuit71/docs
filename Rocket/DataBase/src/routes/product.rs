use rocket::{
    fairing::AdHoc,
    serde::json::{serde_json::json, Value},
};
use rocket_db_pools::Connection;

use crate::models::{Pg, Product, ProductType};

#[post("/", data = "<product>")]
async fn create_product(mut db: Connection<Pg>, product: ProductType) -> Value {
    let args = product.clone();
    let res = sqlx::query!(
        "INSERT INTO products (name, price, user_id) VALUES ($1, $2, $3)",
        args.name,
        args.price,
        args.user_id
    )
    .execute(&mut *db)
    .await;

    match res {
        Ok(_) => json!({
            "code": 200,
            "message": "successfully created"
        }),
        Err(e) => {
            let code = e.as_database_error().unwrap().code().unwrap();
            if code == "23503" {
                json!({
                    "code": 400,
                    "message": "user_id doesn't exists in users table"
                })
            } else {
                json!({
                    "code": 500,
                    "message": "uncaughted error"
                })
            }
        }
    }
}

#[get("/")]
async fn get_products(mut db: Connection<Pg>) -> Value {
    let res = sqlx::query!("SELECT * FROM products")
        .fetch_all(&mut *db)
        .await
        .ok()
        .unwrap_or(vec![]);

    let mut recs = vec![];
    res.iter().for_each(|rec| {
        recs.push(json!({
            "id": rec.id,
            "name": rec.name.clone().unwrap(),
            "price": rec.price,
            "user_id": rec.user_id
        }))
    });

    json!({
        "code": 200,
        "products": recs
    })
}

#[get("/<id>")]
async fn get_product_by_id(mut db: Connection<Pg>, id: i32) -> Value {
    let res = sqlx::query!("SELECT * FROM products WHERE id = $1", id)
        .fetch_one(&mut *db)
        .await
        .ok();

    match res {
        Some(rec) => {
            let product = Product::new(rec.name, rec.price, rec.user_id);
            json!({
                "code": 200,
                "product": product
            })
        }
        None => json!({
            "code": 404,
            "message": "id not found"
        }),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("product", |rocket| async {
        rocket.mount(
            "/product",
            routes![create_product, get_products, get_product_by_id],
        )
    })
}
