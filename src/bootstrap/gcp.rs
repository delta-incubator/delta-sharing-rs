use anyhow::Context;
use anyhow::Result;
use tame_gcs::signing::ServiceAccount;

pub fn new(path: &str) -> Result<ServiceAccount> {
    tracing::info!("creating GCP service account");
    let sa = ServiceAccount::load_json_file(path)
        .context("failed to load GCP service account private key JSON")?;
    tracing::info!("created GCP service account");
    Ok(sa)
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_new() {
        let path = format!(
            "{}",
            shellexpand::tilde(
                std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                    .ok()
                    .unwrap_or("~/.gcp/service-account-file.json".into())
                    .as_str()
            )
        );
        assert!(new(&path).is_ok());
    }
}
