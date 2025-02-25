use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::{Result, RetryConfig, TemporaryToken, TokenCache};

/// Provides credentials for use when signing requests
#[async_trait]
pub trait CredentialProvider: std::fmt::Debug + Send + Sync {
    /// The type of credential returned by this provider
    type Credential;

    /// Return a credential
    async fn get_credential(&self) -> Result<Arc<Self::Credential>>;
}

/// A static set of credentials
#[derive(Debug)]
pub struct StaticCredentialProvider<T> {
    credential: Arc<T>,
}

impl<T> StaticCredentialProvider<T> {
    /// A [`CredentialProvider`] for a static credential of type `T`
    pub fn new(credential: T) -> Self {
        Self {
            credential: Arc::new(credential),
        }
    }
}

#[async_trait]
impl<T> CredentialProvider for StaticCredentialProvider<T>
where
    T: std::fmt::Debug + Send + Sync,
{
    type Credential = T;

    async fn get_credential(&self) -> Result<Arc<T>> {
        Ok(Arc::clone(&self.credential))
    }
}

/// A [`CredentialProvider`] that uses [`Client`] to fetch temporary tokens
#[derive(Debug)]
pub(crate) struct TokenCredentialProvider<T: TokenProvider> {
    inner: T,
    client: Client,
    retry: RetryConfig,
    cache: TokenCache<Arc<T::Credential>>,
}

impl<T: TokenProvider> TokenCredentialProvider<T> {
    pub(crate) fn new(inner: T, client: Client, retry: RetryConfig) -> Self {
        Self {
            inner,
            client,
            retry,
            cache: Default::default(),
        }
    }

    /// Override the minimum remaining TTL for a cached token to be used
    pub(crate) fn with_min_ttl(mut self, min_ttl: Duration) -> Self {
        self.cache = self.cache.with_min_ttl(min_ttl);
        self
    }
}

#[async_trait]
impl<T: TokenProvider> CredentialProvider for TokenCredentialProvider<T> {
    type Credential = T::Credential;

    async fn get_credential(&self) -> Result<Arc<Self::Credential>> {
        self.cache
            .get_or_insert_with(|| self.inner.fetch_token(&self.client, &self.retry))
            .await
    }
}

#[async_trait]
pub(crate) trait TokenProvider: std::fmt::Debug + Send + Sync {
    type Credential: std::fmt::Debug + Send + Sync;

    async fn fetch_token(
        &self,
        client: &Client,
        retry: &RetryConfig,
    ) -> Result<TemporaryToken<Arc<Self::Credential>>>;
}
