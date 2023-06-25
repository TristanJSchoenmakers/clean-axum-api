use api::domain::entities::todo_item::TodoItem;
use axum::{body::Body, http::Request};
use chrono::Utc;
use common::setup_api;
use hyper::{Method, StatusCode};
use pretty_assertions::assert_eq;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;
    let now = Utc::now();

    sqlx::query!(r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('22222222-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', 'some_title', '', 0, $1, false, $1, $1);
        "#,
        now
    )
     .execute(&mut conn)
     .await
     .unwrap();

    let app = setup_api(pool).await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/todoitem/22222222-1111-2222-3333-444444444444")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    let todo_item: TodoItem = serde_json::from_str(&body).unwrap();
    assert_eq!(
        todo_item.id.to_string(),
        "22222222-1111-2222-3333-444444444444".to_string()
    );
    assert_eq!(
        todo_item.list_id.to_string(),
        "11111111-1111-2222-3333-444444444444".to_string()
    );
    assert_eq!(todo_item.title, "some_title".to_string());
    assert_eq!(todo_item.created_at.timestamp(), now.timestamp());

    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
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
