use api::{config::Config, routes};
use axum::Extension;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

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
        .unwrap();

    info!("listening on localhost:8000");

    // Start running the API
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(routes::app().layer(Extension(db)).into_make_service())
        .await
        .unwrap();
}
