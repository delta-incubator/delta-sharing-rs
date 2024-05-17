use std::ops::Deref;

use axum::extract::FromRequestParts;

use delta_sharing_core::capabilities::Capabilities as CoreCapabilities;
use http::request::Parts;

use crate::error::Error;

#[derive(Debug)]
pub struct Capabilities(pub CoreCapabilities);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Capabilities {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let capabilities: CoreCapabilities = (&parts.headers).try_into()?;
        Ok(Capabilities(capabilities))
    }
}

impl Deref for Capabilities {
    type Target = CoreCapabilities;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use delta_sharing_core::capabilities::ResponseFormat;

    use super::*;

    #[tokio::test]
    async fn test_capabilities() {
        let request = http::Request::builder()
            .header(
                "delta-sharing-capabilities",
                "responseformat=delta,Parquet;readerfeatures=feature1,feature2",
            )
            .body(())
            .unwrap();

        let (mut parts, _) = request.into_parts();
        let capabilities = Capabilities::from_request_parts(&mut parts, &())
            .await
            .unwrap();

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
