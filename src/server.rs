pub mod entities;
pub mod interactors;
mod repositories;
mod schemas;
mod services;
use crate::utils;
use anyhow::Context;
use anyhow::Result;
use redis::Client;
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

pub struct Server {
    pg_pool: PgPool,
    redis_client: Client,
    gcp_service_account: ServiceAccount,
    aws_profile_provider: ProfileProvider,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let pg_pool = utils::new_pg_pool()
            .await
            .context("failed to create postgres connection pool")?;
        let redis_client = utils::new_redis_client().context("failed to create redis client")?;
        let gcp_service_account =
            utils::new_gcp_service_account().context("failed to create gcp service account")?;
        let aws_profile_provider =
            utils::new_aws_profile_provider().context("failed to create aws profile provider")?;
        Ok(Server {
            pg_pool,
            redis_client,
            gcp_service_account,
            aws_profile_provider,
        })
    }

    pub async fn start(self: Self) -> Result<()> {
        interactors::bind(
            self.pg_pool,
            self.redis_client,
            self.gcp_service_account,
            self.aws_profile_provider,
        )
        .await
        .context("failed to start API server")
    }
}
