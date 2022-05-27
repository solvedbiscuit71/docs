use diesel::prelude::*;
use diesel_demo::connect_to_postgres;
use diesel_demo::models::Post;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let db = connect_to_postgres();
    let res = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&db)
        .expect("Error loading posts");

    res.iter().for_each(|post| {
        println!("{}", post.title);
        println!("-------------");
        println!("{}", post.body);
    });
}
