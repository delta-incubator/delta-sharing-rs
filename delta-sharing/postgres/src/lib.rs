pub use crate::error::{Error, Result};
pub use graph::*;

mod constants;
mod error;
mod graph;
mod pagination;
mod resources;

#[cfg(all(test, feature = "integration-pg"))]
mod tests {
    use std::sync::Arc;

    use delta_sharing_common::memory::InMemoryResourceStore;
    use delta_sharing_common::rest::integration::{test_catalog_router, test_credentials_router};
    use delta_sharing_common::rest::{
        get_catalog_router, get_credentials_router, get_external_locations_router,
        get_schemas_router, AnonymousAuthenticator, AuthenticationLayer,
    };
    use delta_sharing_common::{
        ConstantPolicy, Policy, ProvidesPolicy, ProvidesResourceStore, ProvidesSecretManager,
        ResourceStore, SecretManager,
    };

    use super::*;

    #[derive(Clone)]
    struct Handler {
        store: GraphStore,
        policy: Arc<dyn Policy>,
        secrets: Arc<InMemoryResourceStore>,
    }

    impl Handler {
        fn new(pool: sqlx::PgPool) -> Self {
            Self {
                store: GraphStore::new(pool.into()),
                policy: Arc::new(ConstantPolicy::default()),
                secrets: Arc::new(InMemoryResourceStore::new()),
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

    impl ProvidesSecretManager for Handler {
        fn secret_manager(&self) -> &dyn SecretManager {
            self.secrets.as_ref()
        }
    }

    #[sqlx::test]
    async fn test_catalog(pool: sqlx::PgPool) {
        let handler = Handler::new(pool);
        let router = get_catalog_router(handler.clone())
            .merge(get_schemas_router(handler))
            .layer(AuthenticationLayer::new(AnonymousAuthenticator));
        test_catalog_router(router.clone()).await;
    }

    #[sqlx::test]
    async fn test_credentials(pool: sqlx::PgPool) {
        let handler = Handler::new(pool);
        let router = get_credentials_router(handler.clone())
            .merge(get_external_locations_router(handler))
            .layer(AuthenticationLayer::new(AnonymousAuthenticator));
        test_credentials_router(router).await;
    }
}
