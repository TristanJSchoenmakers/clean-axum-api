use chrono::Utc;
use common::setup_api;
use hyper::{header, Body, Method, Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;
    let now = Utc::now();

    sqlx::query!(r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('55555555-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', 'some_title', '', 0, $1, false, $1, $1);
        "#,
        now
    )
     .execute(&mut conn)
     .await
     .unwrap();

    let app = setup_api(pool).await;
    let request = Request::builder()
        .method(Method::PATCH)
        .uri("/todoitem/55555555-1111-2222-3333-444444444444")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            r#"{
              "title": "Some updated Title",
              "note": "Some updated Note",
              "priority": "High",
              "done": true
            }"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert_eq!(body, "{\"success\":true}");

    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;
    let request = Request::builder()
        .method(Method::PATCH)
        .uri("/todoitem/8ccf4b24-7b25-4781-8e4e-b22931dd6558")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            r#"{
              "title": "Some updated Title",
              "note": "Some updated Note",
              "priority": "High",
              "done": true
            }"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert!(body.contains(r#"{"code":"NOT_FOUND","message":"Todo Item with id '8ccf4b24-7b25-4781-8e4e-b22931dd6558' not found"}"#));

    Ok(())
}
