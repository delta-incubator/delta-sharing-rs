use anyhow::Result;
use rusoto_credential::ProfileProvider;

pub fn new(path: &str, profile: &str) -> Result<ProfileProvider> {
    tracing::info!("creating AWS profile provider");
    let pp = ProfileProvider::with_configuration(path, profile);
    tracing::info!("created AWS profile provider");
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
