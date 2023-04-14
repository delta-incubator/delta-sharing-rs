pub(crate) mod aws;
pub(crate) mod gcp;
mod postgres;
use crate::config;
use anyhow::Result;
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

pub(crate) async fn new_pg_pool() -> Result<PgPool> {
    postgres::connect(&config::fetch::<String>("db_url")).await
}

pub(crate) fn new_gcp_service_account() -> Result<ServiceAccount> {
    let path = format!(
        "{}",
        shellexpand::tilde(
            std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                .ok()
                .unwrap_or("~/.gcp/service-account-file.json".into())
                .as_str()
        )
    );
    gcp::new(&path)
}

pub(crate) fn new_aws_profile_provider() -> Result<ProfileProvider> {
    aws::new(&config::fetch::<String>("aws_profile"))
}
