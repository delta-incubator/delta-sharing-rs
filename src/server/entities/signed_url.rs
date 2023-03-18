use crate::impl_string_property;
use crate::impl_u64_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct SignedUrlBucket {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(SignedUrlBucket);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct SignedUrlPath {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(SignedUrlPath);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct SignedUrlExpiry {
    #[validate(range(min = 0))]
    value: u64,
}

impl_u64_property!(SignedUrlExpiry);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct SignedUrl {
    #[getset(get = "pub", set = "pub")]
    bucket: SignedUrlBucket,
    #[getset(get = "pub", set = "pub")]
    path: SignedUrlPath,
    #[getset(get = "pub", set = "pub")]
    expiry: SignedUrlExpiry,
}

impl SignedUrl {
    pub fn new(bucket: String, path: String, expiry: u64) -> Result<Self> {
        Ok(Self {
            bucket: SignedUrlBucket::new(bucket)?,
            path: SignedUrlPath::new(path)?,
            expiry: SignedUrlExpiry::new(expiry)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signed_url_bucket() {
        assert!(matches!(
            SignedUrlBucket::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_signed_url_bucket() {
        assert!(matches!(SignedUrlBucket::new(""), Err(_)));
    }

    #[test]
    fn test_valid_signed_url_path() {
        assert!(matches!(
            SignedUrlPath::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_signed_url_path() {
        assert!(matches!(SignedUrlPath::new(""), Err(_)));
    }

    #[test]
    fn test_valid_signed_url_expiry() {
        assert!(matches!(
            SignedUrlExpiry::new(testutils::rand::u64(0, 10000)),
            Ok(_)
        ));
    }
}
