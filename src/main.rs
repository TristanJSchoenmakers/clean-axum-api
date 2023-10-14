use api::{config::Config, routes};
use axum::Extension;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::{self, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    // Initialize the logger
    tracing_subscriber::fmt::init();

    info!("Starting API");

    // Parse our configuration from the environment.
    let config = Config::parse();

    // Create a single database connection pool that's shared across the whole application.
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to initialize Postgres connection");

    info!("listening on localhost:8000");

    // Start running the API
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(
            routes::app()
                .layer(
                    trace::TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new())
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(Extension(db))
                .into_make_service(),
        )
        .await
        .unwrap();
}
