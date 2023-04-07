mod api_doc;
mod entities;
mod middlewares;
mod repositories;
mod routers;
mod services;
pub(crate) mod utilities;
use crate::bootstrap;
use anyhow::Context;
use anyhow::Result;
use rusoto_credential::AwsCredentials;
use rusoto_credential::ProvideAwsCredentials;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

pub struct Server {
    pg_pool: PgPool,
    gcp_service_account: ServiceAccount,
    aws_credentials: AwsCredentials,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let pg_pool = bootstrap::new_pg_pool()
            .await
            .context("failed to create postgres connection pool")?;
        let gcp_service_account =
            bootstrap::new_gcp_service_account().context("failed to create gcp service account")?;
        let aws_profile_provider = bootstrap::new_aws_profile_provider()
            .context("failed to create aws profile provider")?;
        let aws_credentials = aws_profile_provider
            .credentials()
            .await
            .context("failed to create aws credentials")?;
        Ok(Server {
            pg_pool,
            gcp_service_account,
            aws_credentials,
        })
    }

    pub async fn start(self: Self) -> Result<()> {
        routers::bind(self.pg_pool, self.gcp_service_account, self.aws_credentials)
            .await
            .context("failed to start API server")
    }
}
