use axum::{
    routing::{get, post},
    Router,
};

mod create_transaction;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/transaction", post(create_transaction::create_user))
}

async fn index() -> &'static str {
    "Hello!"
}
