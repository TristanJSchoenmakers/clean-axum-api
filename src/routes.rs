use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use self::{create_todo_item::create_todo_item, delete_todo_item::delete_todo_item};

mod create_todo_item;
mod delete_todo_item;
mod get_todo_item;
mod update_todo_item;

pub fn app() -> Router {
    Router::new()
        .route("/todoitem", post(create_todo_item))
        .route("/todoitem/:todo_item_id", delete(delete_todo_item))
        .route("/todoitem/:todo_item_id", get(get_todo_item::get_todo_item))
        .route(
            "/todoitem/update/:id",
            patch(update_todo_item::update_todo_item),
        )
}