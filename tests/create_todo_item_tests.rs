use axum::{body::Body, http::Request};
use common::setup_api;
use hyper::{header, Method, StatusCode};
use pretty_assertions::assert_eq;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/todoitem")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            r#"{
                    "title": "no one",
                    "note": "my note",
                    "priority": "Medium"
                }"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert!(body.contains(r#"{"todo_item_id":""#));

    Ok(())
}

#[sqlx::test]
fn incorrect_request(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/todoitem")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{
                          "note": "my note",
                          "priority": "Medium"
                        }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[sqlx::test]
fn invalid_domain(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;

    let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/todoitem")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                          "title": "TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE",
                          "note": "my note",
                          "priority": "Medium"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert_eq!(body, "{\"code\":\"VALIDATION_ERROR\",\"errors\":{\"title\":[\"cannot be longer than 25 characters\"]},\"message\":\"Validation error occurred\"}");

    Ok(())
}
