#![allow(unused_parens)]

use axum::body::Body;
use axum::extract::{FromRequest, FromRequestParts, Json, Path, Query, Request};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{RequestExt, RequestPartsExt};
use serde::Deserialize;

use crate::models::sharing::v1::*;
use crate::Error;

pub use auth::*;
pub use catalog::get_router as get_catalog_router;
pub use credentials::get_router as get_credentials_router;
pub use repo::get_router as get_sharing_repo_router;
pub use router::get_router as get_sharing_router;

mod auth;
mod catalog;
mod credentials;
mod repo;
mod router;

/// Macro to implement FromRequestParts for simple path-only requests
macro_rules! impl_path_request {
    ($request_type:ident { $($field:ident: $field_type:ty),+ }) => {
        impl<S: Send + Sync> FromRequestParts<S> for $request_type {
            type Rejection = Error;

            async fn from_request_parts(
                parts: &mut Parts,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                let Path(($($field),+)) = parts.extract::<Path<($($field_type),+)>>().await?;
                Ok($request_type {
                    $($field),+
                })
            }
        }
    };
}

#[derive(Deserialize)]
struct Pagination {
    max_results: Option<i32>,
    page_token: Option<String>,
}

/// Macro to implement FromRequestParts for requests with pagination
macro_rules! impl_paginated_request {
    // Matcher for requests with only pagination (no path parameters)
    ($request_type:ident {}) => {
        impl<S: Send + Sync> FromRequestParts<S> for $request_type {
            type Rejection = Error;

            async fn from_request_parts(
                parts: &mut Parts,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
                Ok($request_type {
                    max_results: pagination.max_results,
                    page_token: pagination.page_token,
                })
            }
        }
    };
    // Matcher for requests with both path parameters and pagination
    ($request_type:ident { $($field:ident: $field_type:ty),+ }) => {
        impl<S: Send + Sync> FromRequestParts<S> for $request_type {
            type Rejection = Error;

            async fn from_request_parts(
                parts: &mut Parts,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
                let Path(($($field),*)) = parts.extract::<Path<($($field_type),*)>>().await?;
                Ok($request_type {
                    $($field,)*
                    max_results: pagination.max_results,
                    page_token: pagination.page_token,
                })
            }
        }
    };
}

/// Macro to implement FromRequest for JSON body requests
macro_rules! impl_json_request {
    ($request_type:ident) => {
        impl<S: Send + Sync> FromRequest<S> for $request_type {
            type Rejection = Response;

            async fn from_request(
                req: Request<Body>,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                let Json(request) = req.extract().await.map_err(IntoResponse::into_response)?;
                Ok(request)
            }
        }
    };
    // For requests with both path parameters and JSON body
    ($request_type:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl<S: Send + Sync> FromRequest<S> for $request_type {
            type Rejection = Response;

            async fn from_request(
                req: Request<Body>,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                let (mut parts, body) = req.into_parts();
                let Path(($($field),*)) = parts
                    .extract::<Path<($($field_type),*)>>()
                    .await
                    .map_err(IntoResponse::into_response)?;
                let req = Request::from_parts(parts, body);
                let Json(json_body) = req.extract().await.map_err(IntoResponse::into_response)?;
                Ok($request_type {
                    $($field,)*
                    ..json_body
                })
            }
        }
    };
}

impl_path_request!(GetShareRequest { name: String });
impl_path_request!(DeleteShareRequest { name: String });
impl_path_request!(DeleteSharingSchemaRequest {
    share: String,
    name: String
});
impl_path_request!(GetTableMetadataRequest {
    share: String,
    schema: String,
    name: String
});

// Implement for paginated requests
impl_paginated_request!(ListSharesRequest {});
impl_paginated_request!(ListSharingSchemasRequest { share: String });
impl_paginated_request!(ListShareTablesRequest { name: String });
impl_paginated_request!(ListSchemaTablesRequest {
    share: String,
    name: String
});

// Implement for JSON body requests
impl_json_request!(CreateShareRequest);
impl_json_request!(CreateSharingSchemaRequest { share: String });

#[derive(Deserialize)]
struct GetTableVersionQuery {
    starting_timestamp: Option<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for GetTableVersionRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = parts.extract::<Query<GetTableVersionQuery>>().await?;
        let Path((share, schema, name)) = parts.extract::<Path<(String, String, String)>>().await?;
        Ok(GetTableVersionRequest {
            share,
            schema,
            name,
            starting_timestamp: query.starting_timestamp,
        })
    }
}
