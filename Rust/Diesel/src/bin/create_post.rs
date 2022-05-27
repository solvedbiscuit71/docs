use diesel::prelude::*;
use diesel_demo::{
    connect_to_postgres,
    models::{NewPost, Post},
    schema::posts,
};

fn get_string<'a>(prompt: &'a str) -> String {
    let mut input = String::new();
    println!("prompt: {}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    let input = input.trim();
    input.to_string()
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    let title = get_string("Enter title");
    let body = get_string("Enter body");

    let db = connect_to_postgres();
    let post = create_post(&db, &title, &body);

    println!("{} ({})", post.title, post.id);
    println!("-------------");
    println!("{}", post.body);
}
