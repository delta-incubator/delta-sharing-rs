use anyhow::Context;
use anyhow::Result;
use std::time::Duration;
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

pub fn signed_url(
    service_account: &ServiceAccount,
    bucket: &str,
    object: &str,
    duration: &u64,
) -> Result<Url> {
    let bucket = BucketName::try_from(bucket).context("failed to parse bucket name")?;
    let object = ObjectName::try_from(object).context("failed to parse object name")?;
    let options = SignedUrlOptional {
        duration: Duration::from_secs(*duration),
        ..Default::default()
    };
    let signer = UrlSigner::with_ring();
    let url = signer
        .generate(service_account, &(&bucket, &object), options)
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
}
