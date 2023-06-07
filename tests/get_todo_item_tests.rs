use axum::{body::Body, http::Request};
use common::setup_api;
use hyper::{Method, StatusCode};
use pretty_assertions::assert_eq;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/todoitem/8ccf4b24-7b25-4781-8e4e-b22931dd6558")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert!(body.contains(r#"{"code":"NOT_FOUND","message":"Todo Item with id '8ccf4b24-7b25-4781-8e4e-b22931dd6558' not found"}"#));

    Ok(())
}
