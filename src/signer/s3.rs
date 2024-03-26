//! UrlSigner for S3 object paths.

use std::time::Duration;

use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_s3::{presigning::PresigningConfig, Client};

use http::Uri;
use std::str::FromStr;

use super::{SignedUrl, Signer, SignerError};

/// Signing configuration for the S3 object store.
pub struct S3UrlSigner {
    /// The configuration for the S3 signer.
    client: Client,
}

impl S3UrlSigner {
    /// Create a new `S3UrlSigner` from the provided S3 SDK client.
    pub fn new(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        Self { client }
    }
}

#[async_trait]
impl Signer for S3UrlSigner {
    async fn sign(&self, uri: &str, expires_in: Duration) -> Result<SignedUrl, SignerError> {
        let s3_uri = S3Uri::from_str(uri)?;
        let presign_config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| SignerError::other(e.to_string()))?;

        let start_time = presign_config.start_time();
        let expires = presign_config.expires();
        let req = self
            .client
            .get_object()
            .bucket(s3_uri.bucket())
            .key(s3_uri.key())
            .presigned(presign_config)
            .await
            .map_err(|e| SignerError::other(format!("Failed to sign URL: {e}")))?;

        Ok(SignedUrl::new(
            req.uri().to_string(),
            start_time.into(),
            expires,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct S3Uri {
    bucket: String,
    key: String,
}

impl S3Uri {
    fn new(bucket: String, key: String) -> Self {
        Self { bucket, key }
    }

    fn from_s3_uri(uri: &Uri) -> Result<Self, SignerError> {
        let bucket = uri
            .host()
            .ok_or(SignerError::parse_uri_error(
                format!("failed to extract the S3 bucket name. Format the URI as `s3://<bucket_name>/<key>`. Received: {}", uri)
            ))?;

        let key = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::parse_uri_error(
                format!("failed to extract the S3 object key. Format the URI as `s3://<bucket_name>/<key>`. Received: {}", uri)
            ))?;

        if key.is_empty() {
            return Err(SignerError::parse_uri_error(
                format!("S3 object key is empty. Format the URI as `s3://<bucket_name>/<key>`. Received: {}", uri)
            ));
        }

        Ok(Self::new(bucket.to_string(), key.to_string()))
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl FromStr for S3Uri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri: Uri = s.parse().map_err(|e| {
            SignerError::parse_uri_error(format!("Invalid URI. Cause: {e}. Received URI: `{s}`."))
        })?;

        match uri.scheme_str() {
            Some("s3" | "s3a" | "s3n") => Ok(Self::from_s3_uri(&uri)?),
            Some(unsupported_scheme) => Err(
                SignerError::parse_uri_error(
                format!("The URI scheme should be `S3`, `S3a` or `S3n`. Received URI scheme: `{unsupported_scheme}`.")
            )),     
            None => Err(SignerError::parse_uri_error(
                format!("The URI scheme shoud be `S3`, `S3a` or `S3n`. Received URI: `{s}`.")
                
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::signer::SignerErrorKind;

    use super::*;

    #[test]
    fn parse_s3_scheme() {
        let uri = "s3://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3a_scheme() {
        let uri = "s3a://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3n_scheme() {
        let uri = "s3n://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3_nested_key() {
        let uri = "s3://bucket/key/nested";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key/nested");
    }

    #[test]
    fn parse_invalid_empty_key() {
        let uri = "s3://bucket/";
        let uri_err = S3Uri::from_str(uri).unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::ParseUriError);
        assert_eq!(
            uri_err.message(),
            "S3 object key is empty. Format the URI as `s3://<bucket_name>/<key>`. Received: s3://bucket/"
        );
    }

    #[test]
    fn parse_invalid_uri() {
        let uri = "";
        let uri_err = S3Uri::from_str(uri).unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::ParseUriError);
        assert_eq!(
            uri_err.message(),
            "Invalid URI. Cause: empty string. Received URI: ``."
        );
    }

    #[test]
    fn parse_uri_without_scheme() {
        let uri = "bucket";
        let uri_err = S3Uri::from_str(uri).unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::ParseUriError);
        assert_eq!(
            uri_err.message(),
            "The URI scheme shoud be `S3`, `S3a` or `S3n`. Received URI: `bucket`."
        );
    }

    #[test]
    fn parse_unsupported_scheme() {
        let uri = "abfss://bucket.s3.us-east-1.amazonaws.com/key";
        let uri_err = S3Uri::from_str(uri).unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::ParseUriError);
        assert_eq!(
            uri_err.message(),
            "The URI scheme should be `S3`, `S3a` or `S3n`. Received URI scheme: `abfss`."
        );
    }
}
