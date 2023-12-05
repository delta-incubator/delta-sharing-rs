use anyhow::Context;
use anyhow::Result;
use rusoto_credential::ProfileProvider;

pub fn new(profile: &str) -> Result<ProfileProvider> {
    tracing::info!("creating AWS profile provider");
    let mut pp = ProfileProvider::new().context("failed to create AWS profile provider")?;
    pp.set_profile(profile);
    tracing::info!("created AWS profile provider");
    Ok(pp)
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_new() {
        let aws_profile = std::env::var("AWS_PROFILE").expect("AWS profile should be specified");
        assert!(new(&aws_profile).is_ok());
    }
}
