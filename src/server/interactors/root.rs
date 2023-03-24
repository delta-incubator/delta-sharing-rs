use crate::error::Error;
use crate::server::schemas::Claims;
use crate::utils::jwt::Claims as JwtClaims;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RootResponse {
    pub claims: Claims,
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = 200, description = "Logged-in successfully", body = RootResponse),
        (status = 401, description = "Authorization failed", body = ErrorResponse),
    )
)]
pub async fn get(claims: JwtClaims) -> Result<Response, Error> {
    Ok((
        StatusCode::OK,
        Json(RootResponse {
            claims: Claims {
                name: claims.name,
                email: claims.email,
                namespace: claims.namespace,
                role: claims.role,
                exp: claims.exp,
            },
        }),
    )
        .into_response())
}
