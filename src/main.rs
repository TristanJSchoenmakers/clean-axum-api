use api::config::Config;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
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
        .expect("Failed to initialize Postgres connection");

    info!("listening on localhost:8000");

    axum::serve(
        TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        api::app(db).into_make_service(),
    )
    .await
    .unwrap();
}
