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
    Aws,
    Gcp,
    Azure,
    None,
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(input).context("failed to parse URL")?;
        match url.scheme() {
            "s3" | "s3a" => Ok(Self::Aws),
            "abfss" | "abfs" => Ok(Self::Azure),
            "gs" => Ok(Self::Gcp),
            _ => Ok(Self::None),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusoto_credential::AwsCredentials;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_aws_sign_local() {
        let creds = AwsCredentials::new("test", "test", None, None);
        if let Ok(Platform::Aws) = Platform::from_str("s3://delta-sharing-test/covid") {
            let signer = AwsSigner {
                aws: creds,
                expiration: Duration::from_secs(300),
            };
            if let Ok(url) = signer.sign("s3://delta-sharing-test/covid").await {
                println!("{:?}", url);
            } else {
                panic!("failed to sign S3 url")
            }
        } else {
            panic!("failed to parse S3 url");
        };
    }

    #[tokio::test]
    async fn test_azure_sign_local() {
        let creds = StorageCredentials::access_key("mystorageaccount", "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");
        if let Ok(Platform::Azure) =
            Platform::from_str("abfss://mycontainer@mystorageaccount.dfs.core.windows.net/myblob")
        {
            let signer = AzureSigner {
                azure: creds,
                expiration: Duration::from_secs(300),
            };
            if let Ok(url) = signer
                .sign("abfss://mycontainer@mystorageaccount.dfs.core.windows.net/myblob")
                .await
            {
                println!("{:?}", url);
            } else {
                panic!("failed to sign Azure url")
            }
        } else {
            panic!("failed to parse Azure url");
        };
    }

    // TODO: tame-gcs does not allow you to hard code some credentials :(
    // #[tokio::test]
    // async fn test_gcp_sign_local() {
    //     let path = format!(
    //         "{}",
    //         shellexpand::tilde(
    //             std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
    //                 .ok()
    //                 .unwrap_or("~/.gcp/service-account-file.json".into())
    //                 .as_str()
    //         )
    //     );
    //     let sa =
    //         bootstrap::gcp::new(&path).expect("GCP service account should be created properly");
    //     if let Ok(Platform::Gcp { bucket, path, .. }) =
    //         Platform::from_str("gs://delta-sharing-test/covid")
    //     {
    //         if let Ok(url) = Utility::sign_gcp(&sa, &bucket, &path, &300) {
    //             println!("{:?}", url);
    //         }
    //     } else {
    //         panic!("failed to parse GS url");
    //     };
    // }
}
