use anyhow::Result;
use rusoto_credential::ProfileProvider;
use tracing::info;

pub fn new(path: &str, profile: &str) -> Result<ProfileProvider> {
    info!("creating AWS profile provider");
    let pp = ProfileProvider::with_configuration(path, profile);
    info!("created AWS profile provider");
    Ok(pp)
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
}
