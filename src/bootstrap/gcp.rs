use anyhow::Context;
use anyhow::Result;
use tame_gcs::signing::ServiceAccount;
use tracing::info;

pub fn new(path: &str) -> Result<ServiceAccount> {
    info!("creating GCP service account");
    let sa = ServiceAccount::load_json_file(path)
        .context("failed to load GCP service account private key JSON")?;
    info!("created GCP service account");
    Ok(sa)
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
