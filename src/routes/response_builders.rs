//! Contains common axum responses for our routes

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::error;

/// Utility function for mapping any error into a `500 Internal Server Error` response.
pub fn internal_error<E>(err: E) -> (StatusCode, axum::Json<Value>)
where
    E: std::error::Error,
{
    error!("internal_error: {}", err);
    let body = json!({ "code": "INTERNAL_SERVER_ERROR", "message": err.to_string() });
    (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
}

/// Utility function for mapping an validator::ValidationErrors Error to an axum validation error http response
pub fn validation_error(err: validator::ValidationErrors) -> (StatusCode, axum::Json<Value>) {
    let error_map: HashMap<String, Vec<String>> = err
        .field_errors()
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.iter()
                    .filter_map(|v2| v2.message.as_ref().map(ToString::to_string))
                    .collect::<Vec<String>>(),
            )
        })
        .collect();

    let body = json!({
        "code": "VALIDATION_ERROR",
        "message": "Validation error occurred",
        "errors": error_map
    });

    (StatusCode::BAD_REQUEST, Json(body))
}
