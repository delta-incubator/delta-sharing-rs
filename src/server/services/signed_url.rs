use crate::utils::aws;
use crate::utils::gcp;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use rusoto_credential::ProfileProvider as AWS;
use std::str::FromStr;
use tame_gcs::signing::ServiceAccount as GCP;
use url::Url;

#[derive(Debug)]
pub enum SignedUrl {
    AWS { bucket: String, path: String },
    GCP { bucket: String, path: String },
}

impl FromStr for SignedUrl {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(input).context("failed to parse signed url")?;
        match url.scheme() {
            "s3" => Ok(Self::AWS {
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path()),
            }),
            "s3a" => Ok(Self::AWS {
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path()),
            }),
            "gs" => Ok(Self::GCP {
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path()),
            }),
            _ => Err(anyhow!("failed to parse signed url")),
        }
    }
}

#[async_trait]
pub trait SignedUrlService {
    async fn signup(&self, bucket: &str, path: &str, duration: &u64) -> Result<Url>;
}

#[async_trait]
impl SignedUrlService for AWS {
    async fn signup(&self, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
        aws::signed_url(&self, bucket, path, duration).await
    }
}

#[async_trait]
impl SignedUrlService for GCP {
    async fn signup(&self, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
        gcp::signed_url(&self, bucket, path, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::utils::aws;
    use std::str::FromStr;

    #[test]
    fn test_aws_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("s3://{}/{}", bucket, path);
        let signed_url = SignedUrl::from_str(&url).expect("should parse s3 url properly");
        if let SignedUrl::AWS {
            bucket: parsed_bucket,
            path: parsed_path,
        } = signed_url
        {
            let mut with_slash: String = "/".to_owned();
            with_slash.push_str(&path);
            assert_eq!(parsed_bucket, bucket);
            assert_eq!(parsed_path, with_slash);
        } else {
            panic!("should be parsed as S3 url");
        }
    }

    #[test]
    fn test_gcp_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("gs://{}/{}", bucket, path);
        let signed_url = SignedUrl::from_str(&url).expect("should parse s3 url properly");
        if let SignedUrl::GCP {
            bucket: parsed_bucket,
            path: parsed_path,
        } = signed_url
        {
            let mut with_slash: String = "/".to_owned();
            with_slash.push_str(&path);
            assert_eq!(parsed_bucket, bucket);
            assert_eq!(parsed_path, with_slash);
        } else {
            panic!("should be parsed as GS url");
        }
    }

    //#[tokio::test]
    async fn test_aws_signup_local() {
        let pp = if let Ok(pp) = aws::new(
            &config::fetch::<String>("aws_credentials"),
            &config::fetch::<String>("aws_profile"),
        ) {
            pp
        } else {
            panic!("failed to create AWS profile provider");
        };
        let (bucket, path) = if let Ok(SignedUrl::AWS { bucket, path }) =
            SignedUrl::from_str("s3://kotosiro-sharing-test/sample.txt")
        {
            (bucket, path)
        } else {
            panic!("failed to parse S3 url");
        };
        if let Ok(url) = SignedUrlService::signup(&pp, &bucket, &path, &300).await {
            println!("{:?}", url);
        }
    }

    //#[tokio::test]
    async fn test_gcp_signup_local() {
        let sa = if let Ok(sa) = gcp::new(&config::fetch::<String>("gcp_sa_private_key")) {
            sa
        } else {
            panic!("failed to create GCP service account");
        };
        let (bucket, path) = if let Ok(SignedUrl::GCP { bucket, path }) =
            SignedUrl::from_str("gs://kotosiro-sharing-test/sample.txt")
        {
            (bucket, path)
        } else {
            panic!("failed to parse GS url");
        };
        if let Ok(url) = SignedUrlService::signup(&sa, &bucket, &path, &300).await {
            println!("{:?}", url);
        }
    }
}
