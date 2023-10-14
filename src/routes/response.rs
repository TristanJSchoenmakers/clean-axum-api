//! Contains common axum responses for our routes

use std::collections::HashMap;

use axum::{http::StatusCode, Json};
use serde_json::json;
use tracing::error;

//TODO: transform in to a macro?
pub fn json_error(code: String, message: String) -> (StatusCode, String) {
    let body = Json(json!({ "code": code, "message": message }));
    (StatusCode::NOT_FOUND, body.to_string())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    error!("internal_error: {}", err);
    let body = Json(json!({ "code": "INTERNAL_SERVER_ERROR", "message": err.to_string() }));
    (StatusCode::INTERNAL_SERVER_ERROR, body.to_string())
}

pub fn validation_error(err: validator::ValidationErrors) -> (StatusCode, String) {
    let error_map: HashMap<String, Vec<String>> = err
        .field_errors()
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.into_iter()
                    // .filter_map(|v2| v2.message.as_ref().map(|s| s.to_string()))
                    .filter_map(|v2| v2.message.as_ref().map(ToString::to_string))
                    .collect::<Vec<String>>(),
            )
        })
        .collect();

    let body = Json(json!({
        "code": "VALIDATION_ERROR",
        "message": "Validation error occurred",
        "errors": error_map
    }));

    (StatusCode::BAD_REQUEST, body.to_string())
}
