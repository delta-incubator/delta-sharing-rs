use anyhow::Context;
use anyhow::Result;
use rusoto_core::Region;
use rusoto_credential::AwsCredentials as AWS;
use rusoto_s3::util::PreSignedRequest;
use rusoto_s3::util::PreSignedRequestOption;
use rusoto_s3::GetObjectRequest;
use std::str::FromStr;
use std::time::Duration;
use tame_gcs::signed_url::SignedUrlOptional;
use tame_gcs::signed_url::UrlSigner;
use tame_gcs::signing::ServiceAccount as GCP;
use tame_gcs::BucketName;
use tame_gcs::ObjectName;
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum Platform {
    AWS {
        url: String,
        bucket: String,
        path: String,
    },
    GCP {
        url: String,
        bucket: String,
        path: String,
    },
    NONE {
        url: String,
    },
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(input).context("failed to parse URL")?;
        match url.scheme() {
            "s3" => Ok(Self::AWS {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix("/").unwrap_or("")),
            }),
            "s3a" => Ok(Self::AWS {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix("/").unwrap_or("")),
            }),
            "gs" => Ok(Self::GCP {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix("/").unwrap_or("")),
            }),
            _ => Ok(Self::NONE {
                url: String::from(url.as_str()),
            }),
        }
    }
}

pub struct Utility;

impl Utility {
    pub fn sign_aws(aws: &AWS, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
        let region = Region::default();
        let options = PreSignedRequestOption {
            expires_in: Duration::from_secs(*duration),
        };
        let request = GetObjectRequest {
            bucket: bucket.to_string(),
            key: path.to_string(),
            ..Default::default()
        };
        let url = request.get_presigned_url(&region, aws, &options);
        let url = Url::parse(&url).context("failed to parse AWS signed URL")?;
        Ok(url)
    }

    pub fn sign_gcp(gcp: &GCP, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
        let bucket = BucketName::try_from(bucket).context("failed to parse bucket name")?;
        let object = ObjectName::try_from(path).context("failed to parse object name")?;
        let options = SignedUrlOptional {
            duration: Duration::from_secs(*duration),
            ..Default::default()
        };
        let signer = UrlSigner::with_ring();
        let url = signer
            .generate(gcp, &(&bucket, &object), options)
            .context("failed to generate signed url")?;
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap;
    use rusoto_credential::ProvideAwsCredentials;
    use std::str::FromStr;

    #[test]
    fn test_aws_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("s3://{}/{}", bucket, path);
        let provider = Platform::from_str(&url).expect("should parse s3 url properly");
        if let Platform::AWS {
            url: parsed_url,
            bucket: parsed_bucket,
            path: parsed_path,
        } = provider
        {
            assert_eq!(parsed_url, url);
            assert_eq!(parsed_bucket, bucket);
            assert_eq!(parsed_path, path);
        } else {
            panic!("should be parsed as S3 url");
        }
    }

    #[test]
    fn test_gcp_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("gs://{}/{}", bucket, path);
        let provider = Platform::from_str(&url).expect("should parse gcs url properly");
        if let Platform::GCP {
            url: parsed_url,
            bucket: parsed_bucket,
            path: parsed_path,
        } = provider
        {
            assert_eq!(parsed_url, url);
            assert_eq!(parsed_bucket, bucket);
            assert_eq!(parsed_path, path);
        } else {
            panic!("should be parsed as GS url");
        }
    }

    //#[tokio::test]
    async fn test_aws_sign_local() {
        let aws_profile = std::env::var("AWS_PROFILE").expect("AWS profile should be specified");
        let pp = bootstrap::aws::new(&aws_profile)
            .expect("AWS profile provider should be created properly");
        let creds = pp
            .credentials()
            .await
            .expect("AWS credentials should be acquired properly");
        if let Ok(Platform::AWS { bucket, path, .. }) =
            Platform::from_str("s3://delta-sharing-test/covid")
        {
            if let Ok(url) = Utility::sign_aws(&creds, &bucket, &path, &300) {
                println!("{:?}", url);
            }
        } else {
            panic!("failed to parse S3 url");
        };
    }

    //#[tokio::test]
    async fn test_gcp_sign_local() {
        let path = format!(
            "{}",
            shellexpand::tilde(
                std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                    .ok()
                    .unwrap_or("~/.gcp/service-account-file.json".into())
                    .as_str()
            )
        );
        let sa =
            bootstrap::gcp::new(&path).expect("GCP service account should be created properly");
        if let Ok(Platform::GCP { bucket, path, .. }) =
            Platform::from_str("gs://delta-sharing-test/covid")
        {
            if let Ok(url) = Utility::sign_gcp(&sa, &bucket, &path, &300) {
                println!("{:?}", url);
            }
        } else {
            panic!("failed to parse GS url");
        };
    }
}
