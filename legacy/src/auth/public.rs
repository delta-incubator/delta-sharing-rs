//! Authentication middleware for public access
//!
//! The public access authentication middleware does not perform any
//! authentication and sets the [`RecipientId`] in the request extension to
//! anonymous.
//!
//! # Example
//! ```rust
//! use axum::extract::Request;
//! use axum::response::Response;
//! use axum::routing::get;
//! use axum::body::Body;
//! use tower::{ServiceBuilder, ServiceExt, Service, BoxError};
//!
//! use delta_sharing::auth::RecipientId;
//! use delta_sharing::auth::public::PublicAccessAuthLayer;
//!
//! async fn handler(req: Request<Body>) -> Result<Response<Body>, BoxError> {
//!     let recipient_id = req.extensions().get::<RecipientId>().unwrap();
//!     assert_eq!(recipient_id, &RecipientId::Unknown);
//!     Ok(Response::new(Body::empty()))
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//! let mut service = ServiceBuilder::new()
//!     .layer(PublicAccessAuthLayer::new())
//!     .service_fn(handler);
//!
//! let request = Request::get("/")
//!     .body(Body::empty())
//!     .unwrap();
//!
//! let res = service.ready().await.unwrap().call(request).await.unwrap();
//!
//! # Ok(())
//! # }
//! ```
use std::task::{Context, Poll};

use axum::extract::Request;
use tower::{Layer, Service};

use crate::auth::RecipientId;

/// Layer that applies the [`PublicAccessAuth`] middleware.
#[derive(Debug, Clone)]
pub struct PublicAccessAuthLayer;

impl Default for PublicAccessAuthLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicAccessAuthLayer {
    /// Create a new [`PublicAccessAuthLayer`].
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for PublicAccessAuthLayer {
    type Service = PublicAccessAuth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PublicAccessAuth { inner }
    }
}

/// Authentication middleware.
///
/// Does not perform any authentication and sets the [`RecipientId`] in the
/// request extension to anonymous.
#[derive(Debug, Clone)]
pub struct PublicAccessAuth<S> {
    inner: S,
}

impl<S> Service<Request> for PublicAccessAuth<S>
where
    S: Service<Request> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let id = RecipientId::unknown();
        tracing::info!(recipient_id=?id, "authenticated");

        req.extensions_mut().insert(id);
        self.inner.call(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{header, Request, StatusCode};
    use axum::response::Response;
    use tower::BoxError;
    use tower::ServiceBuilder;
    use tower::ServiceExt;

    #[tokio::test]
    async fn public_access_auth_with_bearer() {
        let mut service = ServiceBuilder::new()
            .layer(PublicAccessAuthLayer::new())
            .service_fn(check_recipient);

        let request = Request::get("/")
            .header(header::AUTHORIZATION, "Bearer foo")
            .body(Body::empty())
            .unwrap();

        let res = service.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn public_access_auth_without_bearer() {
        let mut service = ServiceBuilder::new()
            .layer(PublicAccessAuthLayer::new())
            .service_fn(check_recipient);

        let request = Request::get("/").body(Body::empty()).unwrap();
        let res = service.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    async fn check_recipient(req: Request<Body>) -> Result<Response<Body>, BoxError> {
        assert_eq!(
            req.extensions().get::<RecipientId>(),
            Some(&RecipientId::Unknown)
        );
        Ok(Response::new(req.into_body()))
    }
}
