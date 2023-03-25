mod aws;
mod gcp;
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
    gcp::new(&config::fetch::<String>("gcp_sa_private_key"))
}

pub(crate) fn new_aws_profile_provider() -> Result<ProfileProvider> {
    aws::new(
        &config::fetch::<String>("aws_credentials"),
        &config::fetch::<String>("aws_profile"),
    )
}
