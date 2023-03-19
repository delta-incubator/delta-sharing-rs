use crate::error::Error;
use crate::utils::jwt::Claims;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;

pub async fn profile(claims: Claims) -> Result<Response, Error> {
    Ok((
        StatusCode::OK,
        Json(json!({"name": claims.name, "email": claims.email, "namespace": claims.namespace })),
    )
        .into_response())
}
