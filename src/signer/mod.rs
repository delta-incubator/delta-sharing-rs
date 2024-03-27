//! Traits and types for creating signed urls
//!
//! Signed URLs are used to provide access to data in object stores without
//! requiring the client to authenticate with the object store. The client
//! can access the data using the signed URL, which is valid for a limited
//! time.
//!
//! The Delta Sharing protocol requires signed urls to access data in the object
//! store. This module provides a trait for creating signed urls and
//! implementations of this trait for several popular (cloud) object stores.

#![warn(missing_docs)]

use std::time::Duration;
use std::{fmt, time::SystemTime};

use async_trait::async_trait;

pub mod s3;

/// Interface for creating signed URLs from object store specific URIs.
#[async_trait]
pub trait Signer: Send + Sync {
    /// Create a signed url from a (cloud) object store URI that is valid from
    /// now and expires after the specified duration.
    async fn sign(&self, uri: &str, expires_in: Duration) -> Result<SignedUrl, SignerError>;
}

/// A signed URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedUrl {
    url: String,
    valid_from: SystemTime,
    valid_duration: Duration,
}

impl SignedUrl {
    /// Create a new signed URL.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let url = "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo";
    /// let valid_from = SystemTime::now();
    /// let valid_duration = Duration::from_secs(60);
    /// let signed_url = SignedUrl::new(url, valid_from, valid_duration);
    ///
    /// assert_eq!(signed_url.url(), "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo");
    /// ```
    pub fn new(url: impl Into<String>, valid_from: SystemTime, valid_duration: Duration) -> Self {
        Self {
            url: url.into(),
            valid_from,
            valid_duration,
        }
    }

    /// Get the signed URL.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let url = "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo";
    /// let valid_from = SystemTime::now();
    /// let valid_duration = Duration::from_secs(60);
    /// let signed_url = SignedUrl::new(url, valid_from, valid_duration);
    ///
    /// assert_eq!(signed_url.url(), "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo");
    /// ```
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the time from which the signed url is valid.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let signed_url = SignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert!(signed_url.valid_from() <= SystemTime::now());
    /// ```
    pub fn valid_from(&self) -> SystemTime {
        self.valid_from
    }

    /// Get the duration for which the signed url is valid.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let signed_url = SignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(signed_url.valid_duration(), Duration::from_secs(60));
    /// ```
    pub fn valid_duration(&self) -> Duration {
        self.valid_duration
    }

    /// Get the time the presigned url expires.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let now = SystemTime::now();
    /// let signed_url = SignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo",
    ///     now,
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(signed_url.expires_at(), now + Duration::from_secs(60));
    /// ```
    pub fn expires_at(&self) -> SystemTime {
        self.valid_from + self.valid_duration
    }

    /// Get the expiration timestamp in milliseconds since UNIX epoch.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let signed_url = SignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo",
    ///     SystemTime::UNIX_EPOCH,
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(signed_url.expiration_timestamp_millis(), 60_000);
    /// ```
    pub fn expiration_timestamp_millis(&self) -> i64 {
        self.expires_at()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("expiration time is after epoch")
            .as_millis() as i64
    }

    /// Check if the presigned url is expired.
    ///
    /// # Example
    /// ```rust
    /// # use std::time::{Duration, SystemTime};
    /// # use std::thread::sleep;
    /// use delta_sharing::signer::SignedUrl;
    ///
    /// let signed_url = SignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key?sig=foo",
    ///     SystemTime::now(),
    ///     Duration::from_millis(100),
    /// );
    /// assert!(!signed_url.is_expired());
    /// sleep(Duration::from_millis(200));
    /// assert!(signed_url.is_expired());
    /// ```
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at()
    }
}

impl AsRef<str> for SignedUrl {
    fn as_ref(&self) -> &str {
        self.url()
    }
}

/// Error classes that can occur when creating a signed URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignerErrorKind {
    /// Error parsing the URI.
    ParseUriError,
    /// The specified expiration time is too long.
    ExpirationTooLong,
    /// Other error.
    Other,
}

impl fmt::Display for SignerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignerErrorKind::Other => write!(f, "OTHER"),
            SignerErrorKind::ParseUriError => write!(f, "PARSE_URI_ERROR"),
            SignerErrorKind::ExpirationTooLong => write!(f, "EXPIRATION_TOO_LONG"),
        }
    }
}

/// Error that can occur when creating a signed URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignerError {
    kind: SignerErrorKind,
    message: String,
}

impl SignerError {
    /// Create a new `SignerError`.
    pub fn new(kind: SignerErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// Get the error kind.
    pub fn kind(&self) -> SignerErrorKind {
        self.kind
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Create a new `SignerError` with the `ParseUriError` kind.
    pub fn parse_uri_error(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::ParseUriError, message)
    }

    /// Create a new `SignerError` with the `ExpirationTooLong` kind.
    pub fn expiration_too_long(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::ExpirationTooLong, message)
    }

    /// Create a new `SignerError` with the `Other` kind.
    pub fn other(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::Other, message)
    }
}

impl fmt::Display for SignerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.kind, self.message)
    }
}

impl std::error::Error for SignerError {}
