pub mod config;
pub mod domain {
    pub mod todo_item;
}
pub mod routes;

#[cfg(test)]
mod test_util {
    use crate::{config::Config, routes};
    use axum::{Extension, Router};
    use clap::Parser;
    use sqlx::postgres::PgPoolOptions;

    pub async fn setup_axum() -> Router {
        let config = Config::parse();

        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .unwrap();

        routes::app().layer(Extension(db))
    }
}
