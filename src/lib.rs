/// Handles the database
pub mod database;

pub mod domain {
    pub mod todo_item;
}

/// Http Routes
pub mod routes {
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
}

#[cfg(test)]
mod test_util {
    use super::database::initialize_database;
    use axum::{Extension, Router};

    pub async fn setup_axum() -> Router {
        let db = initialize_database().await.unwrap();
        crate::routes::app().layer(Extension(db))
    }
}
