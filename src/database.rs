use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialize_database() -> Result<PgPool, sqlx::Error> {
    print!("Connecting to database...   ");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    println!("Connected to database!");

    Ok(db)
}
