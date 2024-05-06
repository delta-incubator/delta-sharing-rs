use std::str::FromStr;

use http::header::HeaderMap;

const DELTA_SHARING_CAPABILITIES: &str = "delta-sharing-capabilities";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFormat {
    Parquet,
    Delta,
}

impl FromStr for ResponseFormat {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parquet" => Ok(Self::Parquet),
            "delta" => Ok(Self::Delta),
            _ => Err("Invalid response format".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    response_formats: Vec<ResponseFormat>,
    reader_features: Vec<String>,
}

impl Capabilities {
    pub fn new(response_formats: Vec<ResponseFormat>, reader_features: Vec<String>) -> Self {
        Self {
            response_formats,
            reader_features,
        }
    }

    pub fn response_formats(&self) -> &[ResponseFormat] {
        &self.response_formats
    }

    pub fn reader_features(&self) -> &[String] {
        &self.reader_features
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
    type Error = Box<dyn std::error::Error>;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let mut capabilities = Capabilities::default();
        if let Some(header) = headers.get(DELTA_SHARING_CAPABILITIES) {
            let capability_headers = header.to_str()?;
            for capability in capability_headers.split(';') {
                let (capability_key, capability_value) = capability.split_once('=').unwrap();
                match capability_key {
                    "responseformat" => {
                        capabilities.response_formats = capability_value
                            .split(',')
                            .flat_map(|s| s.trim().to_lowercase().parse::<ResponseFormat>().ok())
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
        assert_eq!(capabilities.response_formats, vec![ResponseFormat::Parquet]);
        assert_eq!(capabilities.reader_features, Vec::<String>::new());
    }

    #[test]
    fn test_capabilities_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            DELTA_SHARING_CAPABILITIES,
            "responseformat=delta,parquet;readerfeatures=feature1,feature2"
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
