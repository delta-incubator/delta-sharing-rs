// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::gcp::credential::{
    ApplicationDefaultCredentials, InstanceCredentialProvider, ServiceAccountCredentials,
};
use crate::gcp::{credential, GcpCredential, GcpCredentialProvider};
use crate::gcp::{GoogleClient, GoogleConfig};
use crate::{
    ClientConfigKey, ClientOptions, Result, RetryConfig, StaticCredentialProvider,
    TokenCredentialProvider,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

const TOKEN_MIN_TTL: Duration = Duration::from_secs(4 * 60);

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("One of service account path or service account key may be provided.")]
    ServiceAccountPathAndKeyProvided,

    #[error("Configuration key: '{}' is not known.", key)]
    UnknownConfigurationKey { key: String },

    #[error("GCP credential error: {}", source)]
    Credential { source: credential::Error },
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::UnknownConfigurationKey { key } => Self::UnknownConfigurationKey { key },
            _ => Self::Generic {
                source: Box::new(err),
            },
        }
    }
}

/// Configure a connection to Google Cloud Storage.
///
/// If no credentials are explicitly provided, they will be sourced
/// from the environment as documented [here](https://cloud.google.com/docs/authentication/application-default-credentials).
///
/// # Example
/// ```
/// # use cloud_client::gcp::GoogleBuilder;
/// let gcs = GoogleBuilder::from_env().build();
/// ```
#[derive(Debug, Clone)]
pub struct GoogleBuilder {
    /// Path to the service account file
    service_account_path: Option<String>,
    /// The serialized service account key
    service_account_key: Option<String>,
    /// Path to the application credentials file.
    application_credentials_path: Option<String>,
    /// Retry config
    retry_config: RetryConfig,
    /// Client options
    client_options: ClientOptions,
    /// Credentials
    credentials: Option<GcpCredentialProvider>,
}

/// Configuration keys for [`GoogleBuilder`]
///
/// Configuration via keys can be done via [`GoogleBuilder::with_config`]
///
/// # Example
/// ```
/// # use cloud_client::gcp::{GoogleBuilder, GoogleConfigKey};
/// let builder = GoogleBuilder::new()
///     .with_config("google_service_account".parse().unwrap(), "my-service-account");
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GoogleConfigKey {
    /// Path to the service account file
    ///
    /// Supported keys:
    /// - `google_service_account`
    /// - `service_account`
    /// - `google_service_account_path`
    /// - `service_account_path`
    ServiceAccount,

    /// The serialized service account key.
    ///
    /// Supported keys:
    /// - `google_service_account_key`
    /// - `service_account_key`
    ServiceAccountKey,

    /// Application credentials path
    ///
    /// See [`GoogleBuilder::with_application_credentials`].
    ApplicationCredentials,

    /// Client options
    Client(ClientConfigKey),
}

impl AsRef<str> for GoogleConfigKey {
    fn as_ref(&self) -> &str {
        match self {
            Self::ServiceAccount => "google_service_account",
            Self::ServiceAccountKey => "google_service_account_key",
            Self::ApplicationCredentials => "google_application_credentials",
            Self::Client(key) => key.as_ref(),
        }
    }
}

impl FromStr for GoogleConfigKey {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "google_service_account"
            | "service_account"
            | "google_service_account_path"
            | "service_account_path" => Ok(Self::ServiceAccount),
            "google_service_account_key" | "service_account_key" => Ok(Self::ServiceAccountKey),
            "google_application_credentials" => Ok(Self::ApplicationCredentials),
            _ => match s.strip_prefix("google_").unwrap_or(s).parse() {
                Ok(key) => Ok(Self::Client(key)),
                Err(_) => Err(Error::UnknownConfigurationKey { key: s.into() }.into()),
            },
        }
    }
}

impl Default for GoogleBuilder {
    fn default() -> Self {
        Self {
            service_account_path: None,
            service_account_key: None,
            application_credentials_path: None,
            retry_config: Default::default(),
            client_options: ClientOptions::new().with_allow_http(true),
            credentials: None,
        }
    }
}

impl GoogleBuilder {
    /// Create a new [`GoogleBuilder`] with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create an instance of [`GoogleBuilder`] with values pre-populated from environment variables.
    ///
    /// Variables extracted from environment:
    /// * GOOGLE_SERVICE_ACCOUNT: location of service account file
    /// * GOOGLE_SERVICE_ACCOUNT_PATH: (alias) location of service account file
    /// * SERVICE_ACCOUNT: (alias) location of service account file
    /// * GOOGLE_SERVICE_ACCOUNT_KEY: JSON serialized service account key
    ///
    /// # Example
    /// ```
    /// use cloud_client::gcp::GoogleBuilder;
    ///
    /// let gcs = GoogleBuilder::from_env()
    ///     .build();
    /// ```
    pub fn from_env() -> Self {
        let mut builder = Self::default();

        if let Ok(service_account_path) = std::env::var("SERVICE_ACCOUNT") {
            builder.service_account_path = Some(service_account_path);
        }

        for (os_key, os_value) in std::env::vars_os() {
            if let (Some(key), Some(value)) = (os_key.to_str(), os_value.to_str()) {
                if key.starts_with("GOOGLE_") {
                    if let Ok(config_key) = key.to_ascii_lowercase().parse() {
                        builder = builder.with_config(config_key, value);
                    }
                }
            }
        }

        builder
    }

    /// Set an option on the builder via a key - value pair.
    pub fn with_config(mut self, key: GoogleConfigKey, value: impl Into<String>) -> Self {
        match key {
            GoogleConfigKey::ServiceAccount => self.service_account_path = Some(value.into()),
            GoogleConfigKey::ServiceAccountKey => self.service_account_key = Some(value.into()),
            GoogleConfigKey::ApplicationCredentials => {
                self.application_credentials_path = Some(value.into())
            }
            GoogleConfigKey::Client(key) => {
                self.client_options = self.client_options.with_config(key, value)
            }
        };
        self
    }

    /// Get config value via a [`GoogleConfigKey`].
    ///
    /// # Example
    /// ```
    /// use cloud_client::gcp::{GoogleBuilder, GoogleConfigKey};
    ///
    /// let builder = GoogleBuilder::from_env()
    ///     .with_service_account_key("foo");
    /// let service_account_key = builder.get_config_value(&GoogleConfigKey::ServiceAccountKey).unwrap_or_default();
    /// assert_eq!("foo", &service_account_key);
    /// ```
    pub fn get_config_value(&self, key: &GoogleConfigKey) -> Option<String> {
        match key {
            GoogleConfigKey::ServiceAccount => self.service_account_path.clone(),
            GoogleConfigKey::ServiceAccountKey => self.service_account_key.clone(),
            GoogleConfigKey::ApplicationCredentials => self.application_credentials_path.clone(),
            GoogleConfigKey::Client(key) => self.client_options.get_config_value(key),
        }
    }

    /// Set the path to the service account file.
    ///
    /// This or [`GoogleBuilder::with_service_account_key`] must be
    /// set.
    ///
    /// Example `"/tmp/gcs.json"`.
    ///
    /// Example contents of `gcs.json`:
    ///
    /// ```json
    /// {
    ///    "gcs_base_url": "https://localhost:4443",
    ///    "disable_oauth": true,
    ///    "client_email": "",
    ///    "private_key": ""
    /// }
    /// ```
    pub fn with_service_account_path(mut self, service_account_path: impl Into<String>) -> Self {
        self.service_account_path = Some(service_account_path.into());
        self
    }

    /// Set the service account key. The service account must be in the JSON
    /// format.
    ///
    /// This or [`GoogleBuilder::with_service_account_path`] must be
    /// set.
    pub fn with_service_account_key(mut self, service_account: impl Into<String>) -> Self {
        self.service_account_key = Some(service_account.into());
        self
    }

    /// Set the path to the application credentials file.
    ///
    /// <https://cloud.google.com/docs/authentication/provide-credentials-adc>
    pub fn with_application_credentials(
        mut self,
        application_credentials_path: impl Into<String>,
    ) -> Self {
        self.application_credentials_path = Some(application_credentials_path.into());
        self
    }

    /// Set the credential provider overriding any other options
    pub fn with_credentials(mut self, credentials: GcpCredentialProvider) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set the retry configuration
    pub fn with_retry(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Set the proxy_url to be used by the underlying client
    pub fn with_proxy_url(mut self, proxy_url: impl Into<String>) -> Self {
        self.client_options = self.client_options.with_proxy_url(proxy_url);
        self
    }

    /// Set a trusted proxy CA certificate
    pub fn with_proxy_ca_certificate(mut self, proxy_ca_certificate: impl Into<String>) -> Self {
        self.client_options = self
            .client_options
            .with_proxy_ca_certificate(proxy_ca_certificate);
        self
    }

    /// Set a list of hosts to exclude from proxy connections
    pub fn with_proxy_excludes(mut self, proxy_excludes: impl Into<String>) -> Self {
        self.client_options = self.client_options.with_proxy_excludes(proxy_excludes);
        self
    }

    /// Sets the client options, overriding any already set
    pub fn with_client_options(mut self, options: ClientOptions) -> Self {
        self.client_options = options;
        self
    }

    /// Configure a connection to Google Cloud Storage, returning a
    /// new [`GoogleClient`] and consuming `self`
    pub fn build(self) -> Result<GoogleConfig> {
        // First try to initialize from the service account information.
        let service_account_credentials =
            match (self.service_account_path, self.service_account_key) {
                (Some(path), None) => Some(
                    ServiceAccountCredentials::from_file(path)
                        .map_err(|source| Error::Credential { source })?,
                ),
                (None, Some(key)) => Some(
                    ServiceAccountCredentials::from_key(&key)
                        .map_err(|source| Error::Credential { source })?,
                ),
                (None, None) => None,
                (Some(_), Some(_)) => return Err(Error::ServiceAccountPathAndKeyProvided.into()),
            };

        // Then try to initialize from the application credentials file, or the environment.
        let application_default_credentials =
            ApplicationDefaultCredentials::read(self.application_credentials_path.as_deref())?;

        let disable_oauth = service_account_credentials
            .as_ref()
            .map(|c| c.disable_oauth)
            .unwrap_or(false);

        let credentials = if let Some(credentials) = self.credentials {
            credentials
        } else if disable_oauth {
            Arc::new(StaticCredentialProvider::new(GcpCredential {
                bearer: "".to_string(),
            })) as _
        } else if let Some(credentials) = service_account_credentials.clone() {
            Arc::new(TokenCredentialProvider::new(
                credentials.token_provider()?,
                self.client_options.client()?,
                self.retry_config.clone(),
            )) as _
        } else if let Some(credentials) = application_default_credentials.clone() {
            match credentials {
                ApplicationDefaultCredentials::AuthorizedUser(token) => Arc::new(
                    TokenCredentialProvider::new(
                        token,
                        self.client_options.client()?,
                        self.retry_config.clone(),
                    )
                    .with_min_ttl(TOKEN_MIN_TTL),
                ) as _,
                ApplicationDefaultCredentials::ServiceAccount(token) => {
                    Arc::new(TokenCredentialProvider::new(
                        token.token_provider()?,
                        self.client_options.client()?,
                        self.retry_config.clone(),
                    )) as _
                }
            }
        } else {
            Arc::new(
                TokenCredentialProvider::new(
                    InstanceCredentialProvider::default(),
                    self.client_options.metadata_client()?,
                    self.retry_config.clone(),
                )
                .with_min_ttl(TOKEN_MIN_TTL),
            ) as _
        };

        Ok(GoogleConfig::new(
            credentials,
            self.retry_config,
            self.client_options,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const FAKE_KEY: &str = r#"{"private_key": "private_key", "private_key_id": "private_key_id", "client_email":"client_email", "disable_oauth":true}"#;

    #[test]
    fn gcs_test_service_account_key_and_path() {
        let mut tfile = NamedTempFile::new().unwrap();
        write!(tfile, "{FAKE_KEY}").unwrap();
        let _ = GoogleBuilder::new()
            .with_service_account_key(FAKE_KEY)
            .with_service_account_path(tfile.path().to_str().unwrap())
            .build()
            .unwrap_err();
    }

    #[test]
    fn gcs_test_config_from_map() {
        let google_service_account = "object_store:fake_service_account".to_string();
        let options = HashMap::from([("google_service_account", google_service_account.clone())]);

        let builder = options
            .iter()
            .fold(GoogleBuilder::new(), |builder, (key, value)| {
                builder.with_config(key.parse().unwrap(), value)
            });

        assert_eq!(
            builder.service_account_path.unwrap(),
            google_service_account.as_str()
        );
    }

    #[test]
    fn gcs_test_config_aliases() {
        // Service account path
        for alias in [
            "google_service_account",
            "service_account",
            "google_service_account_path",
            "service_account_path",
        ] {
            let builder =
                GoogleBuilder::new().with_config(alias.parse().unwrap(), "/fake/path.json");
            assert_eq!("/fake/path.json", builder.service_account_path.unwrap());
        }

        // Service account key
        for alias in ["google_service_account_key", "service_account_key"] {
            let builder = GoogleBuilder::new().with_config(alias.parse().unwrap(), FAKE_KEY);
            assert_eq!(FAKE_KEY, builder.service_account_key.unwrap());
        }
    }

    #[tokio::test]
    async fn gcs_test_proxy_url() {
        let mut tfile = NamedTempFile::new().unwrap();
        write!(tfile, "{FAKE_KEY}").unwrap();
        let service_account_path = tfile.path();
        let gcs = GoogleBuilder::new()
            .with_service_account_path(service_account_path.to_str().unwrap())
            .with_proxy_url("https://example.com")
            .build();
        assert!(gcs.is_ok());
    }

    #[test]
    fn gcs_test_service_account_key_only() {
        let _ = GoogleBuilder::new()
            .with_service_account_key(FAKE_KEY)
            .build()
            .unwrap();
    }

    #[test]
    fn gcs_test_config_get_value() {
        let google_service_account = "object_store:fake_service_account".to_string();
        let builder = GoogleBuilder::new()
            .with_config(GoogleConfigKey::ServiceAccount, &google_service_account);

        assert_eq!(
            builder
                .get_config_value(&GoogleConfigKey::ServiceAccount)
                .unwrap(),
            google_service_account
        );
    }

    #[test]
    fn gcp_test_client_opts() {
        let key = "GOOGLE_PROXY_URL";
        if let Ok(config_key) = key.to_ascii_lowercase().parse() {
            assert_eq!(
                GoogleConfigKey::Client(ClientConfigKey::ProxyUrl),
                config_key
            );
        } else {
            panic!("{} not propagated as ClientConfigKey", key);
        }
    }
}
