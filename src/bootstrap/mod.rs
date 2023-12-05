pub(crate) mod aws;
pub(crate) mod gcp;

mod postgres;
use anyhow::Context;
use anyhow::Result;
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

use crate::config;

pub(crate) async fn new_pg_pool() -> Result<PgPool> {
    postgres::connect(&config::fetch::<String>("db_url")).await
}

pub(crate) fn new_gcp_service_account() -> Result<ServiceAccount> {
    let google_applicayion_credentials_path = format!(
        "{}",
        shellexpand::tilde(
            std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                .ok()
                .unwrap_or("~/.gcp/service-account-file.json".into())
                .as_str()
        )
    );
    gcp::new(&google_applicayion_credentials_path)
}

pub(crate) fn new_aws_profile_provider() -> Result<ProfileProvider> {
    let aws_profile =
        std::env::var("AWS_PROFILE").context("failed to get `AWS_PROFILE` environment variable")?;
    aws::new(&aws_profile)
}
