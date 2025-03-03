#![allow(unused, dead_code)]

use std::time::Duration;

use aws::AmazonConfig;
use azure::AzureConfig;
use gcp::GoogleConfig;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Body, Client, IntoUrl, Method, RequestBuilder};
use serde::Serialize;

use self::azure::credential::AzureCredentialExt;
use self::token::{TemporaryToken, TokenCache};

pub mod aws;
pub mod azure;
mod backoff;
mod client;
mod config;
mod credential;
mod error;
pub mod gcp;
#[cfg(test)]
mod mock_server;
mod pagination;
mod retry;
mod token;
mod util;

pub use client::{Certificate, ClientConfigKey, ClientOptions};
pub use credential::*;
pub use error::*;
pub use pagination::stream_paginated;
pub use retry::RetryConfig;

#[derive(Clone)]
enum Credential {
    Aws(AmazonConfig),
    Google(GoogleConfig),
    Azure(AzureConfig),
    Unauthenticated,
}

#[derive(Clone)]
pub struct CloudClient {
    credential: Credential,
    http_client: Client,
    retry_config: RetryConfig,
}

impl CloudClient {
    pub fn new_aws<I, K, V>(options: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: Into<String>,
    {
        let config = options
            .into_iter()
            .fold(
                aws::AmazonBuilder::new(),
                |builder, (key, value)| match key.as_ref().parse() {
                    Ok(k) => builder.with_config(k, value),
                    Err(_) => builder,
                },
            )
            .build()?;

        Ok(Self {
            http_client: config.client_options.client()?,
            retry_config: config.retry_config.clone(),
            credential: Credential::Aws(config),
        })
    }

    pub fn new_google<I, K, V>(options: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: Into<String>,
    {
        let config = options
            .into_iter()
            .fold(
                gcp::GoogleBuilder::new(),
                |builder, (key, value)| match key.as_ref().parse() {
                    Ok(k) => builder.with_config(k, value),
                    Err(_) => builder,
                },
            )
            .build()?;

        Ok(Self {
            http_client: config.client_options.client()?,
            retry_config: config.retry_config.clone(),
            credential: Credential::Google(config),
        })
    }

    pub fn new_azure<I, K, V>(options: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: Into<String>,
    {
        let config = options
            .into_iter()
            .fold(
                azure::AzureBuilder::new(),
                |builder, (key, value)| match key.as_ref().parse() {
                    Ok(k) => builder.with_config(k, value),
                    Err(_) => builder,
                },
            )
            .build()?;

        Ok(Self {
            http_client: config.client_options.client()?,
            retry_config: config.retry_config.clone(),
            credential: Credential::Azure(config),
        })
    }

    pub fn new_unauthenticated() -> Self {
        Self {
            http_client: Client::new(),
            retry_config: RetryConfig::default(),
            credential: Credential::Unauthenticated,
        }
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> CloudRequestBuilder {
        CloudRequestBuilder {
            builder: self.http_client.request(method, url),
            client: self.clone(),
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::PUT, url)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::DELETE, url)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::HEAD, url)
    }

    pub fn patch<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::PATCH, url)
    }

    pub fn options<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::OPTIONS, url)
    }

    pub fn trace<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::TRACE, url)
    }

    pub fn connect<U: IntoUrl>(&self, url: U) -> CloudRequestBuilder {
        self.request(Method::CONNECT, url)
    }
}

pub struct CloudRequestBuilder {
    builder: RequestBuilder,
    client: CloudClient,
}

impl CloudRequestBuilder {
    /// Add a `Header` to this Request.
    pub fn header<K, V>(mut self, key: K, value: V) -> CloudRequestBuilder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.builder = self.builder.header(key, value);
        self
    }

    /// Add a set of Headers to the existing ones on this Request.
    ///
    /// The headers will be merged in to any already set.
    pub fn headers(mut self, headers: HeaderMap) -> CloudRequestBuilder {
        self.builder = self.builder.headers(headers);
        self
    }

    /// Set the request body.
    pub fn body<T: Into<Body>>(mut self, body: T) -> CloudRequestBuilder {
        self.builder = self.builder.body(body);
        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished. It affects only this request and overrides
    /// the timeout configured using `ClientBuilder::timeout()`.
    pub fn timeout(mut self, timeout: Duration) -> CloudRequestBuilder {
        self.builder = self.builder.timeout(timeout);
        self
    }

    /// Modify the query string of the URL.
    ///
    /// Modifies the URL of this request, adding the parameters provided.
    /// This method appends and does not overwrite. This means that it can
    /// be called multiple times and that existing query parameters are not
    /// overwritten if the same key is used. The key will simply show up
    /// twice in the query string.
    /// Calling `.query(&[("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
    ///
    /// # Note
    /// This method does not support serializing a single key-value
    /// pair. Instead of using `.query(("key", "val"))`, use a sequence, such
    /// as `.query(&[("key", "val")])`. It's also possible to serialize structs
    /// and maps into a key-value pair.
    ///
    /// # Errors
    /// This method will fail if the object you provide cannot be serialized
    /// into a query string.
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> CloudRequestBuilder {
        self.builder = self.builder.query(query);
        self
    }

    /// Send a JSON body.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> CloudRequestBuilder {
        self.builder = self.builder.json(json);
        self
    }

    pub async fn send(mut self) -> Result<reqwest::Response> {
        match &self.client.credential {
            Credential::Azure(az) => {
                let credential = az.get_credential().await?;
                self.builder = self.builder.with_azure_authorization(&credential);
            }
            Credential::Aws(_aws) => {
                todo!()
            }
            Credential::Google(gcp) => {
                let credential = gcp.get_credential().await?;
                self.builder = self.builder.bearer_auth(&credential.bearer);
            }
            Credential::Unauthenticated => {
                // Do nothing
            }
        };
        let response = self.builder.send().await?;
        Ok(response)
    }
}
