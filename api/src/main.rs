//! My Clean Rust API project
//!
//! ![](https://www.plantuml.com/plantuml/png/SoWkIImgAStDuNBAJrBGjLDmpCbCJbMmKiX8pSd9vt98pKi1IW80)

use api::{database, router};
use axum::Extension;

#[tokio::main]
async fn main() {
    let db = database::initialize_database().await.unwrap();

    println!("listening on localhost:8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router::app().layer(Extension(db)).into_make_service())
        .await
        .unwrap();
}
