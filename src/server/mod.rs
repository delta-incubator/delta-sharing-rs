mod api_doc;
mod catalog;
mod entities;
mod middlewares;
mod repositories;
pub(crate) mod routers;
mod services;
pub(crate) mod utilities;

use anyhow::{Context, Result};
use rusoto_credential::AwsCredentials;
use rusoto_credential::ProvideAwsCredentials;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;

pub use entities::account::{Entity as AccountEntity, Id as AccountId};
pub use entities::schema::{Entity as SchemaEntity, Id as SchemaId};
pub use entities::share::{Entity as ShareEntity, Id as ShareId};
pub use entities::table::{Entity as TableEntity, Id as TableId};
pub use entities::token::{Entity as TokenEntity, Id as TokenId};
pub use repositories::account::Repository as AccountRepository;
pub use repositories::schema::Repository as SchemaRepository;
pub use repositories::share::Repository as ShareRepository;
pub use repositories::table::Repository as TableRepository;
pub use repositories::token::Repository as TokenRepository;
pub use services::account::Service as AccountService;
pub use services::schema::Service as SchemaService;
pub use services::share::Service as ShareService;
pub use services::table::Service as TableService;

use crate::bootstrap;
pub use crate::server::middlewares::jwt::Role;
use crate::server::routers::AzureLocation;

pub struct Server {
    pg_pool: PgPool,
    gcp_service_account: Option<ServiceAccount>,
    aws_credentials: Option<AwsCredentials>,
    azure_storage_credentials: Option<AzureLocation>,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let pg_pool = bootstrap::new_pg_pool()
            .await
            .context("failed to create postgres connection pool")?;
        let gcp_service_account = bootstrap::new_gcp_service_account().ok();
        if gcp_service_account.is_none() {
            tracing::warn!("failed to load GCP service account");
        }
        let aws_credentials =
            if let Ok(aws_profile_provider) = bootstrap::new_aws_profile_provider() {
                let aws_credentials = aws_profile_provider.credentials().await;
                if aws_credentials.is_ok() {
                    aws_credentials.ok()
                } else {
                    None
                }
            } else {
                None
            };
        if aws_credentials.is_none() {
            tracing::warn!("failed to load AWS credentials");
        }

        let azure_storage_credentials = bootstrap::new_azure_storage_account().ok();
        if azure_storage_credentials.is_none() {
            tracing::warn!("failed to load Azure Storage credentials");
        }

        Ok(Server {
            pg_pool,
            gcp_service_account,
            aws_credentials,
            azure_storage_credentials,
        })
    }

    pub async fn start(self) -> Result<()> {
        routers::bind(
            self.pg_pool,
            self.gcp_service_account,
            self.aws_credentials,
            self.azure_storage_credentials,
        )
        .await
        .context("failed to start API server")
    }
}
