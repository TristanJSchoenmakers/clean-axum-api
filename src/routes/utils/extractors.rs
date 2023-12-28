use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request},
    http::StatusCode,
    RequestPartsExt,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use validator::Validate;

/// Extractor json that also needs to be validated using [`validator`]
///
/// [`validator`]: https://docs.rs/validator/latest/validator/
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Validate,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => match value.0.validate() {
                Ok(_) => Ok(Self(value.0)),
                Err(err) => {
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

                    let payload = json!({
                        "code": "VALIDATION_ERROR",
                        "message": "Validation error occurred",
                        "errors": error_map
                    });

                    Err((StatusCode::BAD_REQUEST, axum::Json(payload)))
                }
            },
            Err(rejection) => {
                let payload = json!({
                    "code": "INVALID_JSON",
                    "message": rejection.body_text(),
                    "path": path,
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}
