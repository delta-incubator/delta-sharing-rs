//! Capabilities of the client.
//!
//! The capabilities are communicated between the client and the server using the `delta-sharing-capabilities` header.

use std::str::FromStr;

use http::header::HeaderMap;

use crate::Error;

const DELTA_SHARING_CAPABILITIES: &str = "delta-sharing-capabilities";

/// The format of the response that the client can accept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFormat {
    /// API response in Parquet format.
    Parquet,
    /// Api response in Delta format.
    Delta,
}

impl FromStr for ResponseFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "parquet" => Ok(Self::Parquet),
            "delta" => Ok(Self::Delta),
            _ => Err(Error::Generic(format!("Unknown response format: {}", s))),
        }
    }
}

/// Capabilities of the client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    response_formats: Vec<ResponseFormat>,
    reader_features: Vec<String>,
}

impl Capabilities {
    /// Create a new [`Capabilities`] instance.
    ///
    /// # Example
    /// ```
    /// # use delta_sharing_common::capabilities::{Capabilities, ResponseFormat};
    ///
    /// let capabilities = Capabilities::new(
    ///   vec![ResponseFormat::Delta],
    ///   vec!["deletionVectors".to_string()],
    /// );
    /// assert_eq!(capabilities.response_formats(), &[ResponseFormat::Delta]);
    /// ```
    pub fn new(response_formats: Vec<ResponseFormat>, reader_features: Vec<String>) -> Self {
        Self {
            response_formats,
            reader_features: reader_features
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect(),
        }
    }

    /// Returns the response formats that the client can accept.
    ///
    /// # Example
    /// ```
    /// # use delta_sharing_common::capabilities::{Capabilities, ResponseFormat};
    ///
    /// let capabilities = Capabilities::new(
    ///   vec![ResponseFormat::Delta],
    ///   vec!["deletionVectors".to_string()],
    /// );
    /// assert_eq!(capabilities.response_formats(), &[ResponseFormat::Delta]);
    /// ```
    pub fn response_formats(&self) -> &[ResponseFormat] {
        &self.response_formats
    }

    /// Returns the reader features that the client can accept.
    ///
    /// # Example
    /// ```
    /// # use delta_sharing_common::capabilities::{Capabilities, ResponseFormat};
    ///
    /// let capabilities = Capabilities::new(
    ///   vec![ResponseFormat::Delta],
    ///   vec!["deletionVectors".to_string()],
    /// );
    /// assert_eq!(capabilities.reader_features(), &["deletionvectors"]);
    pub fn reader_features(&self) -> &[String] {
        self.reader_features.as_slice()
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            response_formats: vec![ResponseFormat::Parquet],
            reader_features: vec![],
        }
    }
}

impl TryFrom<&HeaderMap> for Capabilities {
    type Error = Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let mut capabilities = Capabilities::default();
        if let Some(header) = headers.get(DELTA_SHARING_CAPABILITIES) {
            let capability_header = header.to_str().map_err(|e| {
                Error::Generic(format!("Failed to parse capabilities header: {}", e))
            })?;
            for capability in capability_header.split(';') {
                let (capability_key, capability_value) = capability.split_once('=').ok_or(
                    Error::Generic(format!("Failed to parse capability: {}", capability)),
                )?;
                match capability_key {
                    "responseformat" => {
                        capabilities.response_formats = capability_value
                            .split(',')
                            .flat_map(|s| s.trim().parse().ok())
                            .collect();
                    }
                    "readerfeatures" => {
                        capabilities.reader_features = capability_value
                            .split(',')
                            .map(|s| s.trim().to_lowercase())
                            .collect();
                    }
                    _ => {
                        tracing::warn!(
                            capability = capability_key,
                            "encountered unrecognized capability"
                        );
                    }
                }
            }
        }

        Ok(capabilities)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let capabilities = Capabilities::default();
        assert_eq!(
            capabilities.response_formats(),
            vec![ResponseFormat::Parquet]
        );
        assert_eq!(capabilities.reader_features(), Vec::<String>::new());
    }

    #[test]
    fn test_capabilities_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            DELTA_SHARING_CAPABILITIES,
            "responseformat=delta,Parquet;readerfeatures=feature1,feature2"
                .parse()
                .unwrap(),
        );

        let capabilities = Capabilities::try_from(&headers).unwrap();
        assert_eq!(
            capabilities.response_formats(),
            vec![ResponseFormat::Delta, ResponseFormat::Parquet]
        );
        assert_eq!(
            capabilities.reader_features(),
            vec!["feature1".to_string(), "feature2".to_string()]
        );
    }
}
