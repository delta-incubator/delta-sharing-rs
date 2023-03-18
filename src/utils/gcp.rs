use anyhow::Context;
use anyhow::Result;
use tame_gcs::signed_url::SignedUrlOptional;
use tame_gcs::signed_url::UrlSigner;
use tame_gcs::signing::ServiceAccount;
use tame_gcs::BucketName;
use tame_gcs::ObjectName;
use tracing::info;
use url::Url;

pub fn new(path: &str) -> Result<ServiceAccount> {
    info!("creating GCP service account");
    let sa = ServiceAccount::load_json_file(path)
        .context("failed to load GCP service account private key JSON")?;
    info!("created GCP service account");
    Ok(sa)
}

pub fn signed_url(service_account: &ServiceAccount, bucket: &str, object: &str) -> Result<Url> {
    let bucket = BucketName::try_from(bucket).context("failed to parse bucket name")?;
    let object = ObjectName::try_from(object).context("failed to parse object name")?;
    let signer = UrlSigner::with_ring();
    let url = signer
        .generate(
            service_account,
            &(&bucket, &object),
            SignedUrlOptional::default(),
        )
        .context("failed to generate signed url")?;
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn test_new() {
        assert!(matches!(
            new(&config::fetch::<String>("gcp_sa_private_key")),
            Ok(_)
        ));
    }

    //#[test]
    fn test_signed_url_local() {
        if let Ok(sa) = new(&config::fetch::<String>("gcp_sa_private_key")) {
            if let Ok(url) = signed_url(&sa, "kotosiro-sharing-test", "sample.txt") {
                println!("{:?}", url);
            }
        }
    }
}
