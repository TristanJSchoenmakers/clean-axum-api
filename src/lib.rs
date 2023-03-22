/// Handles the database
pub mod database;

pub mod domain {
    pub mod todo_item;
}

pub mod routes {
    use axum::{
        routing::{get, post},
        Router,
    };

    mod create_todo_item;
    mod get_todo_item;

    pub fn app() -> Router {
        Router::new()
            .route("/", get(index))
            .route("/todoitem/:todo_item_id", get(get_todo_item::get_todo_item))
            .route("/todoitem", post(create_todo_item::create_todo_item))
    }

    async fn index() -> &'static str {
        "Hello!"
    }
}
