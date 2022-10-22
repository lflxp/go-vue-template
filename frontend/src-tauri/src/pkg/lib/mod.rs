pub mod models;
pub mod schema;
pub mod lib;

pub use models::*;
pub use schema::*;
pub use lib::*;

use diesel::prelude::*;
// use std::io::{stdin, Read};
use tauri;

#[tauri::command]
pub fn sqlitedemo() -> String {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    // // 插入数据
    // println!("What would you like your title to be?");
    // let mut title1 = String::new();
    // stdin().read_line(&mut title1).unwrap();
    // let title1 = &title1[..(title1.len() - 1)]; // Drop the newline character
    // println!(
    //     "\nOk! Let's write {} (Press {} when finished)\n",
    //     title1, EOF
    // );
    // let mut body1 = String::new();
    // stdin().read_to_string(&mut body1).unwrap();

    // let _ = create_post(connection, title1, &body1);
    // println!("\nSaved draft {}", title1);

    // 查询数据
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    let mut postsed:Vec<Post> = Vec::new();
    for post in results {
        println!("{}--------{}", post.title,post.body);
        postsed.push(post);
    }
    // format!("{:?}",results);
    return serde_json::to_string(&postsed).unwrap();
}

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";