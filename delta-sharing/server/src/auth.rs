use std::sync::Arc;
use std::task::{Context, Poll};

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use delta_sharing_core::policies::{Authenticator, DeltaRecipient};
use delta_sharing_core::Error as CoreError;
use futures_util::{future::BoxFuture, FutureExt};
use tower::{Layer, Service};

use crate::error::{Error, Result};

pub struct AnonymousAuthenticator;

impl Authenticator for AnonymousAuthenticator {
    type Request = Request;
    type Recipient = DeltaRecipient;

    fn authenticate(&self, _: &Self::Request) -> Result<Self::Recipient, CoreError> {
        Ok(DeltaRecipient::Anonymous)
    }
}

/// Middleware that authenticates requests.
#[derive(Clone)]
pub struct AuthenticationMiddleware<S, T: Clone + Send + Sync + 'static> {
    inner: S,
    authenticator: Arc<dyn Authenticator<Recipient = T, Request = Request>>,
}

impl<S, T: Clone + Send + Sync> Service<Request> for AuthenticationMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
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
pub struct AuthorizationLayer<T: Clone + Send + Sync + 'static> {
    authenticator: Arc<dyn Authenticator<Recipient = T, Request = Request>>,
}

impl<T: Clone + Send + Sync + 'static> AuthorizationLayer<T> {
    /// Create a new [`AuthorizationLayer`].
    pub fn new(authenticator: Arc<dyn Authenticator<Recipient = T, Request = Request>>) -> Self {
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
    use std::sync::Arc;

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
        let authenticator = Arc::new(AnonymousAuthenticator);
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
