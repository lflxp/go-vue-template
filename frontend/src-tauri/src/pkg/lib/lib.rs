use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
// use super::models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> usize {
//     use super::schema::posts;

//     let published: bool = true;
//     let new_post = NewPost { title, body, published };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post")
// }
