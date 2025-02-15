#![allow(unused_parens)]

use axum::extract::{FromRequestParts, Path, Query};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use serde::Deserialize;

use crate::models::v1::*;
use crate::Error;

macro_rules! impl_get_request {
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

impl_get_request!(GetShareRequest { name: String });
impl_get_request!(GetTableMetadataRequest {
    share: String,
    schema: String,
    name: String
});

#[derive(Deserialize)]
struct Pagination {
    max_results: Option<i32>,
    page_token: Option<String>,
}

macro_rules! impl_list_request {
    // Matcher for requests with only pagination (no path parameters)
    ($request_type:ident {}) => {
        impl<S: Send + Sync> FromRequestParts<S> for $request_type {
            type Rejection = Error;

            async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
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

            async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
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

impl_list_request!(ListSharesRequest {});
impl_list_request!(ListSchemasRequest { share: String });
impl_list_request!(ListShareTablesRequest { name: String });
impl_list_request!(ListSchemaTablesRequest {
    share: String,
    name: String
});

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
