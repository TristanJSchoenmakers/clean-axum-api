//! Contains the Router and API route handlers for the application.

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use self::{
    create_todo_item::create_todo_item, delete_todo_item::delete_todo_item,
    get_todo_item::get_todo_item, update_todo_item::update_todo_item,
};

pub mod create_todo_item;
pub mod delete_todo_item;
pub mod get_todo_item;
pub mod update_todo_item;

pub fn app() -> Router {
    Router::new()
        .route("/todoitem", post(create_todo_item))
        .route("/todoitem/:todo_item_id", delete(delete_todo_item))
        .route("/todoitem/:todo_item_id", get(get_todo_item))
        .route("/todoitem/update/:todo_item_id", patch(update_todo_item))
}
