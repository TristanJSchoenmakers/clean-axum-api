//! My Clean Rust API project
//!
//! ![](https://www.plantuml.com/plantuml/png/SoWkIImgAStDuNBAJrBGjLDmpCbCJbMmKiX8pSd9vt98pKi1IW80)

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

    pub fn app() -> Router {
        Router::new()
            .route("/", get(index))
            .route("/todoitem", post(create_todo_item::create_todo_item))
    }

    async fn index() -> &'static str {
        "Hello!"
    }
}
