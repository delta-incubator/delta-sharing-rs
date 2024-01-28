use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::http::Method;
use object_store::azure::MicrosoftAzureBuilder;
use object_store::path::Path;
use object_store::signer::Signer as ObjectStoreSigner;
use rusoto_core::Region;
use rusoto_credential::AwsCredentials as AWS;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::GetObjectRequest;
use tame_gcs::signed_url::{SignedUrlOptional, UrlSigner};
use tame_gcs::signing::ServiceAccount as GCP;
use tame_gcs::{BucketName, ObjectName};
use url::Url;

use crate::server::{routers::AzureCredential, AzureLocation};

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
        self.as_ref().sign(path).await
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

#[async_trait::async_trait]
impl Signer for dyn ObjectStoreSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        let url = Url::parse(path).context("failed to parse URL")?;
        let path = Path::from(url.path());
        let expires_in = Duration::from_secs(crate::config::fetch::<u64>("signed_url_ttl"));
        let signed = self
            .signed_url(Method::GET, &path, expires_in)
            .await
            .context("failed to sign URL")?;
        Ok(signed.to_string())
    }
}

pub struct AzureSigner {
    pub azure: AzureLocation,
    pub expiration: Duration,
}

#[async_trait::async_trait]
impl Signer for AzureSigner {
    async fn sign(&self, path: &str) -> Result<String> {
        let url = Url::parse(path).context("failed to parse URL")?;

        let storage_account = url
            .domain()
            .and_then(|d| d.split_once('.'))
            .map(|m| m.0)
            .unwrap_or("");
        let container = url.username();
        let AzureCredential::AccessKey(access_key) = &self.azure.credential;

        let store = MicrosoftAzureBuilder::new()
            .with_account(storage_account)
            .with_container_name(container)
            .with_access_key(access_key)
            .build()
            .context("failed to build Azure blob store")?;

        let path = Path::parse(url.path().strip_prefix('/').unwrap_or(""))
            .context("failed to parse blob path")?;
        let signed = store
            .signed_url(Method::GET, &path, self.expiration)
            .await
            .context("failed to sign URL")?;

        Ok(signed.into())
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
    pub fn aws_signer(aws: AWS, expiration: Duration) -> Box<dyn Signer> {
        Box::new(AwsSigner { aws, expiration })
    }

    pub fn azure_signer(azure: AzureLocation, expiration: Duration) -> Box<dyn Signer> {
        Box::new(AzureSigner { azure, expiration })
    }

    pub fn gcp_signer(gcp: GCP, expiration: Duration) -> Box<dyn Signer> {
        Box::new(GcpSigner { gcp, expiration })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rusoto_credential::AwsCredentials;
    use serde_json::json;
    use std::str::FromStr;
    use tame_gcs::signing::ServiceAccount;

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
        let creds = AzureLocation {
            account: "mystorageaccount".into(),
            credential: AzureCredential::AccessKey("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==".into())
        };
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

    #[tokio::test]
    async fn test_gcp_sign_local() {
        let creds = json!({
            "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCestzikbw/l1ga\ntLSJYoiQCtX5EM+Dfh3Xn4lefol5uhX0anmrTWIidZ1CSn512tQc6WUKQOHJhDUW\naxDFEr0bwmN+vvguVHXHIRV4ilVbNykt21g3eWGHBOnY0cCjLbSICEQGcqqSCDHe\nhVkKXH4vkpahtX8P0+DiAMZ1Ou+4AaOK6BTUWDIR63+LRGd3wrx8VhzDPJHK2R/q\nO23bM0E8ktNl0Rs0Q/a2krniFwx7f7e/RZrkJ+kq+f6hZrgYwufij1P87XmNKl1T\nz+7xJXsPc72wQkeqr7AGqJe8KsiX9Ajlms7rWQRvx0sJrJYWnT+t++MqOzSXcwB0\nS1tsCOnjAgMBAAECggEAAcwTE0pvitAlx8ZPGxUvaGOEW880bQ/zD5DiHIdV+uJq\nwuX9Hb2rxFR3pPC0sOn6/Ul+rZiMK1zgwFxoXWBNHZuHG+rWuFOYMoVDOdEQjrli\nW9TAPCnsOvwJF+hRhFI4A/2v+NtjPMcfUB6gzQISIxYdDZTBbRugUCmOax/GkjCx\n+GtlxszPfhqgQCsDTXaoceJZlLGg6aZwW7gyHkXOt1P+DDom0cS6GBxR5aIf13Fr\n+I3/318IVCz7JATLvSMETQD2KMt7KAvC6sdLsHqgPBCNJ3u1WSeXwnwxHdCfTkPn\nWLiRCznz4CeVCiY18z+DopmBYPXUMJKap8SaCNsFNQKBgQDf84Yvu6flUBdhIjVL\nfQd1d/TtXa0D0IpnqqkeX51P5cFfjNCzzwHfIv8NWvoQ2LdVeLE0eiNxgHJYj1M+\nv5/FgmLMZGJVysCNI/g+nkjxr7dVtLtpMI55Vm5/cIlsOk85qGPefmaqrHxO8ryh\niN1Hb6QCsccG+Dh3ZoAgwyi1pwKBgQC1aM3Zh8kXfne54AdNBqU7rVqkt6QKoiuI\niPd9fC+sTh9zbVARa0dQQm55x+V0+PvAZNRLueci6ZqRjAhnm3MCLLAfvTsS594D\nDRpjMXdljMQ064jhvmPQ2Q+h+uPzBtH1I5q08CN4BdKXsN1oz7PSfcNU9KyUu/yn\n2tO/iEKpZQKBgQCijg8ugpXB2zq9JKlum9hYKbQ8vywggrSTvsp244w6PFj6VCoA\n+hcvsiVTul+c7tFUVwC5SJaFgmh9Y7tW5pzALn4sQgkmoL7XM+6y9Q2ZcKQwr7kB\nB1/DLzuRgUwepMxw24tyKmm3JPAuFf9ZeRC1E5IG6qe+pVnHQT1rinz4LQKBgFHb\nTKePgcm8I0IYOLMlAIIBIxmYU8kIjCQ7yZEx7EEPr1liRfLWOYOZtkf1TzCM+OxD\nkxfodsdmKXzrdw9pMWgVyhNIS9OoFKHD09hWhc2oyxAmB8n1Iw0mJMuubhVHSo4W\n1sQ2Z4rM9c3E3ONidX3RicZX8Vfby5HiSBHw5kORAoGAaLRFJmgLq092NNtf5O1u\n+9hs/xVxIlUk4yHq7stpHipiUsy4qPHNr2KNP74Y3Ki3jVqULf0oiq72W/1dssTR\n0HmFe832OfH0gjMJyFtH9t0Wi6WyZpRYYvPA8Hl2tTBuSLsQY3p5A3E5okT8TbId\nx1Ee1vUKIFIVfK+pb/l6/lg=\n-----END PRIVATE KEY-----\n",
            "client_email": "real-address@very-good-project-id.iam.gserviceaccount.com",
        });
        let sa = ServiceAccount::load_json(creds.to_string().as_bytes())
            .expect("GCP service account should be created properly");

        if let Ok(Platform::Gcp) = Platform::from_str("gs://delta-sharing-test/covid") {
            let signer = GcpSigner {
                gcp: sa,
                expiration: Duration::from_secs(300),
            };
            if let Ok(url) = signer.sign("gs://delta-sharing-test/covid").await {
                println!("{:?}", url);
            } else {
                panic!("failed to sign GCS url")
            }
        } else {
            panic!("failed to parse GS url");
        };
    }
}
