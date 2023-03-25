use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use rusoto_core::Region;
use rusoto_credential::ProfileProvider as AWS;
use rusoto_credential::ProvideAwsCredentials;
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

#[derive(Debug)]
pub enum Platform {
    AWS { bucket: String, path: String },
    GCP { bucket: String, path: String },
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(input).context("failed to parse signed url provider")?;
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
            _ => Err(anyhow!("failed to parse signed url provider")),
        }
    }
}

pub async fn aws(aws: &AWS, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
    let credentials = aws
        .credentials()
        .await
        .context("failed to acquire AWS credentials")?;
    let region = Region::default();
    let options = PreSignedRequestOption {
        expires_in: Duration::from_secs(*duration),
    };
    let request = GetObjectRequest {
        bucket: bucket.to_string(),
        key: path.to_string(),
        ..Default::default()
    };
    let url = request.get_presigned_url(&region, &credentials, &options);
    let url = Url::parse(&url).context("failed to parse AWS signed URL")?;
    Ok(url)
}

pub async fn gcp(gcp: &GCP, bucket: &str, path: &str, duration: &u64) -> Result<Url> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap;
    use crate::config;
    use std::str::FromStr;

    #[test]
    fn test_aws_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("s3://{}/{}", bucket, path);
        let provider = Platform::from_str(&url).expect("should parse s3 url properly");
        if let Platform::AWS {
            bucket: parsed_bucket,
            path: parsed_path,
        } = provider
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
        let provider = Platform::from_str(&url).expect("should parse s3 url properly");
        if let Platform::GCP {
            bucket: parsed_bucket,
            path: parsed_path,
        } = provider
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
    async fn test_aws_sign_local() {
        let pp = if let Ok(pp) = bootstrap::aws::new(
            &config::fetch::<String>("aws_credentials"),
            &config::fetch::<String>("aws_profile"),
        ) {
            pp
        } else {
            panic!("failed to create AWS profile provider");
        };
        if let Ok(Platform::AWS { bucket, path }) =
            Platform::from_str("s3://kotosiro-sharing-test/sample.txt")
        {
            if let Ok(url) = aws(&pp, &bucket, &path, &300).await {
                println!("{:?}", url);
            }
        } else {
            panic!("failed to parse S3 url");
        };
    }

    //#[tokio::test]
    async fn test_gcp_sign_local() {
        let sa = if let Ok(sa) = bootstrap::gcp::new(&config::fetch::<String>("gcp_sa_private_key"))
        {
            sa
        } else {
            panic!("failed to create GCP service account");
        };
        if let Ok(Platform::GCP { bucket, path }) =
            Platform::from_str("gs://kotosiro-sharing-test/sample.txt")
        {
            if let Ok(url) = gcp(&sa, &bucket, &path, &300).await {
                println!("{:?}", url);
            }
        } else {
            panic!("failed to parse GS url");
        };
    }
}
