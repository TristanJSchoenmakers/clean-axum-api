// source: https://itnext.io/creating-a-rust-web-app-with-rocket-and-diesel-58f5f6cacd27

use crate::diesel::RunQueryDsl;
use rocket::{fairing::AdHoc, serde::json::Json};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

#[macro_use]
extern crate diesel;
use diesel::{table, Insertable, Queryable};

table! {
    blog_posts (id) {
            id -> Int4,
            title -> Varchar,
            body -> Text,
            published -> Bool,
    }
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "blog_posts"]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// ./src/main.rs
use rocket::State;

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!("Hello, {}! You are {} years old.", config.name, config.age)
}

#[get("/")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    })
}

#[post("/", data = "<blog_post>")]
async fn create_blog_post(connection: Db, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first post".to_string(),
        body: "This is my first post".to_string(),
        published: true,
    })
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .attach(Db::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, get_config])
        .mount(
            "/blog",
            routes![
                get_all_blog_posts,
                get_blog_post,
                create_blog_post,
                get_random_blog_post
            ],
        )
}
