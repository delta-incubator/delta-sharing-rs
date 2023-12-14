use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use azure_storage::shared_access_signature::service_sas::BlobSasPermissions;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use rusoto_core::Region;
use rusoto_credential::AwsCredentials as AWS;
use rusoto_s3::util::PreSignedRequest;
use rusoto_s3::util::PreSignedRequestOption;
use rusoto_s3::GetObjectRequest;
use tame_gcs::signed_url::SignedUrlOptional;
use tame_gcs::signed_url::UrlSigner;
use tame_gcs::signing::ServiceAccount as GCP;
use tame_gcs::BucketName;
use tame_gcs::ObjectName;
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum Platform {
    Aws {
        url: String,
        bucket: String,
        path: String,
    },
    Gcp {
        url: String,
        bucket: String,
        path: String,
    },
    Azure {
        url: String,
        storage_account: String,
        container: String,
        blob_name: String,
    },
    None {
        url: String,
    },
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(input).context("failed to parse URL")?;
        match url.scheme() {
            "s3" => Ok(Self::Aws {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix('/').unwrap_or("")),
            }),
            "s3a" => Ok(Self::Aws {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix('/').unwrap_or("")),
            }),
            "abfss" | "abfs" => {
                let storage_account = url
                    .domain()
                    .and_then(|d| d.split_once("."))
                    .map(|m| m.0)
                    .unwrap_or("");
                let container = url.username();
                let blob = url.path().strip_prefix("/").unwrap_or("");

                Ok(Self::Azure {
                    url: String::from(url.as_str()),
                    storage_account: String::from(storage_account),
                    container: String::from(container),
                    blob_name: String::from(blob),
                })
            }
            "gs" => Ok(Self::Gcp {
                url: String::from(url.as_str()),
                bucket: String::from(url.domain().unwrap_or("")),
                path: String::from(url.path().strip_prefix('/').unwrap_or("")),
            }),
            _ => Ok(Self::None {
                url: String::from(url.as_str()),
            }),
        }
    }
}

#[async_trait::async_trait]
pub trait Signer: Send + Sync {
    async fn sign(&self, path: &str) -> Result<String>;
}

#[async_trait::async_trait]
impl<S: Signer + ?Sized> Signer for Box<S> {
    async fn sign(&self, path: &str) -> Result<String> {
        (**self).sign(path).await
    }
}

pub struct NoopSigner;

#[async_trait::async_trait]
impl Signer for NoopSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        Ok(path.to_string())
    }
}

pub struct AwsSigner {
    pub aws: AWS,
    pub expiration: Duration,
}

#[async_trait::async_trait]
impl Signer for AwsSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        let url = Url::parse(path).context("failed to parse URL")?;
        let bucket = String::from(url.domain().unwrap_or(""));
        let path = String::from(url.path().strip_prefix('/').unwrap_or(""));

        let region = Region::default();
        let options = PreSignedRequestOption {
            expires_in: self.expiration,
        };
        let request = GetObjectRequest {
            bucket: bucket.to_string(),
            key: path.to_string(),
            ..Default::default()
        };
        let url = request.get_presigned_url(&region, &self.aws, &options);
        let url = Url::parse(&url).context("failed to parse AWS signed URL")?;
        Ok(url.into())
    }
}

pub struct AzureSigner {
    pub azure: StorageCredentials,
    pub expiration: Duration,
}

#[async_trait::async_trait]
impl Signer for AzureSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        let url = Url::parse(path).context("failed to parse URL")?;

        let storage_account = url
            .domain()
            .and_then(|d| d.split_once("."))
            .map(|m| m.0)
            .unwrap_or("");
        let container = url.username();
        let blob = url.path().strip_prefix("/").unwrap_or("");

        // build azure blob client
        let cb = ClientBuilder::new(storage_account.to_string(), self.azure.clone());
        let blob_client = cb.blob_client(container, blob);

        // generate SAS token for signing URLs
        let mut permissions = BlobSasPermissions::default();
        permissions.read = true;
        let dt = time::OffsetDateTime::now_utc() + self.expiration;
        let sas_token = blob_client
            .shared_access_signature(permissions, dt)
            .await
            .unwrap();

        // sign blobs with SAS token
        let url = blob_client.generate_signed_blob_url(&sas_token)?;
        Ok(url.into())
    }
}

pub struct GcpSigner {
    pub gcp: GCP,
    pub expiration: Duration,
}

#[async_trait::async_trait]
impl Signer for GcpSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        let url = Url::parse(path).context("failed to parse URL")?;
        let bucket = String::from(url.domain().unwrap_or(""));
        let path = String::from(url.path().strip_prefix('/').unwrap_or(""));

        let bucket = BucketName::try_from(bucket).context("failed to parse bucket name")?;
        let object = ObjectName::try_from(path).context("failed to parse object name")?;
        let options = SignedUrlOptional {
            duration: self.expiration,
            ..Default::default()
        };
        let signer = UrlSigner::with_ring();
        let url = signer
            .generate(&self.gcp, &(&bucket, &object), options)
            .context("failed to generate signed url")?;
        Ok(url.into())
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

    pub async fn sign_azure(
        azure: &StorageCredentials,
        storage_account: &str,
        container: &str,
        blob: &str,
        duration: u64,
    ) -> Result<Url> {
        // build azure blob client
        let cb = ClientBuilder::new(storage_account.to_string(), azure.clone());
        let blob_client = cb.blob_client(container, blob);

        // generate SAS token for signing URLs
        let mut permissions = BlobSasPermissions::default();
        permissions.read = true;
        let dt = time::OffsetDateTime::now_utc() + time::Duration::seconds(duration as i64);
        let sas_token = blob_client
            .shared_access_signature(permissions, dt)
            .await
            .unwrap();

        // sign blobs with SAS token
        let url = blob_client.generate_signed_blob_url(&sas_token)?;
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
        if let Platform::Aws {
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
    fn test_abfss_url() {
        let adls_url =
            "abfss://my_container@my_storage_account.dfs.core.windows.net/my_prefix/my_file";
        let provider = Platform::from_str(adls_url).expect("should correctly parse ADLS url");
        if let Platform::Azure {
            url,
            storage_account,
            container,
            blob_name,
        } = provider
        {
            assert_eq!(
                &url,
                "abfss://my_container@my_storage_account.dfs.core.windows.net/my_prefix/my_file"
            );
            assert_eq!(&storage_account, "my_storage_account");
            assert_eq!(&container, "my_container");
            assert_eq!(&blob_name, "my_prefix/my_file");
        } else {
            panic!("expected Azure platform")
        }
    }

    #[test]
    fn test_gcp_url() {
        let bucket = testutils::rand::string(10);
        let path = testutils::rand::string(10);
        let url = format!("gs://{}/{}", bucket, path);
        let provider = Platform::from_str(&url).expect("should parse gcs url properly");
        if let Platform::Gcp {
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
        if let Ok(Platform::Aws { bucket, path, .. }) =
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
        if let Ok(Platform::Gcp { bucket, path, .. }) =
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
