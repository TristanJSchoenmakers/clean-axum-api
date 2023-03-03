use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialize_database() -> Result<PgPool, sqlx::Error> {
    print!("Connecting to database...   ");

    let database_url = "postgres://postgres:example@localhost/mytest";
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to database!");

    Ok(db)
}
