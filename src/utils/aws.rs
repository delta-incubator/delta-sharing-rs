use anyhow::Context;
use anyhow::Result;
use rusoto_core::Region;
use rusoto_credential::ProfileProvider;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_s3::util::PreSignedRequest;
use rusoto_s3::util::PreSignedRequestOption;
use rusoto_s3::GetObjectRequest;
use tracing::info;
use url::Url;

pub fn new(path: &str, profile: &str) -> Result<ProfileProvider> {
    info!("creating AWS profile provider");
    let pp = ProfileProvider::with_configuration(path, profile);
    info!("connected GCP service account");
    Ok(pp)
}

pub async fn signed_url(
    profile_provider: &ProfileProvider,
    bucket: &str,
    object: &str,
) -> Result<Url> {
    let credentials = profile_provider
        .credentials()
        .await
        .context("failed to acquire AWS credentials")?;
    let region = Region::default();
    let request = GetObjectRequest {
        bucket: bucket.to_string(),
        key: object.to_string(),
        ..Default::default()
    };
    let url = request.get_presigned_url(&region, &credentials, &PreSignedRequestOption::default());
    let url = Url::parse(&url).context("failed to parse AWS signed URL")?;
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn test_new() {
        assert!(matches!(
            new(
                &config::fetch::<String>("aws_credentials"),
                &config::fetch::<String>("aws_profile")
            ),
            Ok(_)
        ));
    }

    //#[tokio::test]
    async fn test_signed_url_local() {
        if let Ok(pp) = new(
            &config::fetch::<String>("aws_credentials"),
            &config::fetch::<String>("aws_profile"),
        ) {
            if let Ok(url) = signed_url(&pp, "kotosiro-sharing-test", "sample.txt").await {
                println!("{:?}", url);
            }
        }
    }
}
