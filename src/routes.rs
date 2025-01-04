//! Defines API-routes & API-handlers of our API using [`axum`].
//!
//! [`axum`]: https://github.com/tokio-rs/axum

use self::todo_item::{
    create_todo_item::create_todo_item, delete_todo_item::delete_todo_item,
    get_todo_item::get_todo_item, update_todo_item::update_todo_item,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod extractors;
mod response_builders;

mod todo_item {
    pub mod create_todo_item;
    pub mod delete_todo_item;
    pub mod get_todo_item;
    pub mod update_todo_item;
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Api is running" }))
        .route("/todoitem", post(create_todo_item))
        .route("/todoitem/{todo_item_id}", delete(delete_todo_item))
        .route("/todoitem/{todo_item_id}", get(get_todo_item))
        .route("/todoitem/{todo_item_id}", patch(update_todo_item))
}
