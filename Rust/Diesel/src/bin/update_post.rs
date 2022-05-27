use diesel::prelude::*;
use diesel_demo::{
    connect_to_postgres,
    models::Post,
    schema::posts::dsl::{posts, published},
};

fn get_id<'a>(prompt: &'a str) -> i32 {
    let mut input = String::new();
    println!("prompt: {}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    let input: i32 = input.trim().parse().unwrap();
    input
}
fn main() {
    let id = get_id("Enter id");
    let db = connect_to_postgres();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&db);

    match post {
        Ok(post) => {
            println!("{} ({}) (published)", post.title, post.id);
            println!("-------------");
            println!("{}", post.body);
        }
        Err(err) => {
            println!("Message: {}", err);
        }
    }
}
