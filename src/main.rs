use api::{database, routes};
use axum::Extension;

#[tokio::main]
async fn main() {
    let db = database::initialize_database().await.unwrap();

    println!("listening on localhost:8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(routes::app().layer(Extension(db)).into_make_service())
        .await
        .unwrap();
}
