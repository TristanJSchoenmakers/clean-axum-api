//! My Clean Rust API project
//!
//! ![](https://www.plantuml.com/plantuml/png/SoWkIImgAStDuNBAJrBGjLDmpCbCJbMmKiX8pSd9vt98pKi1IW80)

use axum::Extension;
use clean_rust::{database, router};

#[tokio::main]
async fn main() {
    let db = database::initialize_database().await.unwrap();

    println!("listening on localhost:8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router::app().layer(Extension(db)).into_make_service())
        .await
        .unwrap();
}
