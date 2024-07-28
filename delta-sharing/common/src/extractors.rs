use axum::extract::{FromRequestParts, Path, Query};
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};

use crate::types::*;
use crate::Error;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ListSharesRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
        Ok(ListSharesRequest {
            pagination: Some(pagination),
        })
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for GetShareRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Path(share) = parts.extract::<Path<String>>().await?;
        Ok(GetShareRequest { share })
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ListSchemasRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
        let Path(share) = parts.extract::<Path<String>>().await?;
        Ok(ListSchemasRequest {
            share,
            pagination: Some(pagination),
        })
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ListShareTablesRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
        let Path(share) = parts.extract::<Path<String>>().await?;
        Ok(ListShareTablesRequest {
            share,
            pagination: Some(pagination),
        })
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ListSchemaTablesRequest {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(pagination) = parts.extract::<Query<Pagination>>().await?;
        let Path((share, schema)) = parts.extract::<Path<(String, String)>>().await?;
        Ok(ListSchemaTablesRequest {
            share,
            schema,
            pagination: Some(pagination),
        })
    }
}
