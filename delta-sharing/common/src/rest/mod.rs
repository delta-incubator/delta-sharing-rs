pub use auth::*;
pub use catalog::get_router as get_catalog_router;
pub use credentials::get_router as get_credentials_router;
pub use sharing::get_router as get_sharing_router;

mod auth;
mod catalog;
mod credentials;
mod sharing;

#[cfg(any(test, feature = "integration"))]
pub mod integration {
    use axum::{
        body::Body,
        http::{self, Method, Request},
    };
    use http_body_util::BodyExt;

    pub use super::catalog::integration::*;
    pub use super::credentials::integration::*;

    pub async fn collect_body<T>(response: axum::http::Response<Body>) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).unwrap()
    }

    pub fn create_request<T>(method: Method, uri: &str, body: Option<T>) -> Request<Body>
    where
        T: serde::Serialize,
    {
        if let Some(body) = body {
            Request::builder()
                .method(method)
                .uri(uri)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap()
        } else {
            Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::InMemoryResourceStore;
    use crate::policy::ConstantPolicy;
    use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
    use crate::{Policy, ProvidesPolicy, ProvidesResourceStore, ResourceStore};
    use std::sync::Arc;

    #[derive(Clone)]
    struct Handler {
        store: InMemoryResourceStore,
        policy: Arc<dyn Policy>,
    }

    impl Default for Handler {
        fn default() -> Self {
            Self {
                store: InMemoryResourceStore::new(),
                policy: Arc::new(ConstantPolicy::default()),
            }
        }
    }

    impl ProvidesResourceStore for Handler {
        fn store(&self) -> &dyn ResourceStore {
            &self.store
        }
    }

    impl ProvidesPolicy for Handler {
        fn policy(&self) -> &Arc<dyn Policy> {
            &self.policy
        }
    }

    #[tokio::test]
    async fn test_catalog_router() {
        let app = get_catalog_router(Handler::default())
            .layer(AuthenticationLayer::new(AnonymousAuthenticator));
        super::integration::test_catalog_router(app).await;
    }

    #[tokio::test]
    async fn test_credentials_router() {
        let app = get_credentials_router(Handler::default())
            .layer(AuthenticationLayer::new(AnonymousAuthenticator));
        super::integration::test_credentials_router(app).await;
    }
}
