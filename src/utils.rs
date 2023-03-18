pub mod argon2;
pub mod aws;
pub mod gcp;
pub mod jwt;
pub mod postgres;
pub mod redis;
use crate::config;
use anyhow::Result;
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

pub async fn new_pg_pool() -> Result<PgPool> {
    postgres::connect(&config::fetch::<String>("db_url")).await
}

pub fn new_redis_client() -> Result<redis::Client> {
    redis::connect(&config::fetch::<String>("kvs_url"))
}

pub fn new_gcp_service_account() -> Result<ServiceAccount> {
    gcp::new(&config::fetch::<String>("gcp_sa_private_key"))
}

pub fn new_aws_profile_provider() -> Result<ProfileProvider> {
    aws::new(
        &config::fetch::<String>("aws_credentials"),
        &config::fetch::<String>("aws_profile"),
    )
}
