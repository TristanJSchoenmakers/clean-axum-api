// This is imported by different tests that use different functions.
#![allow(dead_code)]
use api::routes;
use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::{self, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tracing::Level;

/// Sets up the API for testing
pub async fn setup_api(pool: PgPool) -> Router {
    routes::app()
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(pool))
}
