use common::setup_api;
use hyper::{Body, Method, Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn happy_path(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query!(r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('11111111-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', '', '', 0, current_timestamp, false, current_timestamp, current_timestamp);
        "#)
        .execute(&mut conn)
        .await
        .unwrap();

    let app = setup_api(pool).await;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri("/todoitem/11111111-1111-2222-3333-444444444444")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    println!("{}", body);
    assert_eq!(body, "{\"success\":true}");

    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = setup_api(pool).await;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri("/todoitem/99999999-1111-2222-3333-000000000000")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body = String::from_utf8_lossy(&body[..]);
    assert_eq!(
        body,
        r#"{"code":"NOT_FOUND","message":"Todo Item with id '99999999-1111-2222-3333-000000000000' not found"}"#
    );

    Ok(())
}
