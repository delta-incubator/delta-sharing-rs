//! Authentication middleware for Delta Sharing server.

use std::task::{Context, Poll};

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use delta_sharing_core::{Authenticator, DeltaRecipient, Error as CoreError};
use futures_util::{future::BoxFuture, FutureExt};
use tower::{Layer, Service};

use crate::error::{Error, Result};

/// Authenticator that always marks the recipient as anonymous.
#[derive(Clone)]
pub struct AnonymousAuthenticator;

impl Authenticator for AnonymousAuthenticator {
    type Request = Request;
    type Recipient = DeltaRecipient;

    fn authenticate(&self, _request: &Self::Request) -> Result<Self::Recipient, CoreError> {
        Ok(DeltaRecipient::Anonymous)
    }
}

/// Middleware that authenticates requests using the given [`Authenticator`].
#[derive(Clone)]
pub struct AuthenticationMiddleware<S, T> {
    inner: S,
    authenticator: T,
}

impl<S, T> AuthenticationMiddleware<S, T> {
    /// Create new [`AuthenticationMiddleware`].
    pub fn new(inner: S, authenticator: T) -> Self {
        Self {
            inner,
            authenticator,
        }
    }

    /// Create a new [`AuthorizationLayer`] with the given [`Authenticator`].
    /// 
    /// This is a convenience method that is equivalent to calling [`AuthorizationLayer::new`].
    pub fn layer(authenticator: T) -> AuthorizationLayer<T> {
        AuthorizationLayer::new(authenticator)
    }
}

impl<S, T, R> Service<Request> for AuthenticationMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    T: Authenticator<Recipient = R, Request = Request>,
    T::Recipient: Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        match self.authenticator.authenticate(&req) {
            Ok(recipient) => {
                req.extensions_mut().insert(recipient);
                self.inner.call(req).boxed()
            }
            Err(e) => async { Ok(Error::from(e).into_response()) }.boxed(),
        }
    }
}

/// Layer that applies the [`AuthenticationMiddleware`].
#[derive(Clone)]
pub struct AuthorizationLayer<T> {
    authenticator: T,
}

impl<T> AuthorizationLayer<T> {
    /// Create a new [`AuthorizationLayer`] with the provided [`Authenticator`].
    pub fn new(authenticator: T) -> Self {
        Self { authenticator }
    }
}

impl<S, T: Clone + Send + Sync + 'static> Layer<S> for AuthorizationLayer<T> {
    type Service = AuthenticationMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationMiddleware {
            inner,
            authenticator: self.authenticator.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::{header, StatusCode};
    use tower::{ServiceBuilder, ServiceExt};

    use super::*;
    use crate::error::Result;

    async fn check_recipient(req: Request) -> Result<Response<Body>> {
        assert_eq!(
            req.extensions().get::<DeltaRecipient>(),
            Some(&DeltaRecipient::Anonymous)
        );
        Ok(Response::new(req.into_body()))
    }

    #[tokio::test]
    async fn test_authentication_middleware() {
        let authenticator = AnonymousAuthenticator {};
        let mut service = ServiceBuilder::new()
            .layer(AuthorizationLayer::new(authenticator))
            .service_fn(check_recipient);

        let request = Request::get("/")
            .header(header::AUTHORIZATION, "Bearer foo")
            .body(Body::empty())
            .unwrap();

        let response = service.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::get("/").body(Body::empty()).unwrap();
        let response = service.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
