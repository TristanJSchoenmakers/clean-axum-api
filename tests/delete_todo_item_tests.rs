use axum::http::{Request, StatusCode};
use common::RequestBuilderExt;
use common::get_body_json;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    sqlx::query!(r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('11111111-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', '', '', 0, current_timestamp, false, current_timestamp, current_timestamp);
        "#)
        .execute(&pool)
        .await?;
    let app = api::app(pool);
    let request = Request::delete("/todoitem/11111111-1111-2222-3333-444444444444").empty_body();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_body_json(response).await;
    assert_eq!(body["success"].as_bool(), Some(true));
    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::delete("/todoitem/99999999-1111-2222-3333-000000000000").empty_body();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = get_body_json(response).await;
    assert_eq!(body["code"].as_str(), Some("NOT_FOUND"));
    assert_eq!(
        body["message"].as_str(),
        Some("Todo Item with id '99999999-1111-2222-3333-000000000000' not found")
    );
    Ok(())
}
