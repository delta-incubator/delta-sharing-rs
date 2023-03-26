use crate::config;
use crate::server::entities::account::Entity as Account;
pub use crate::server::middlewares::jwt::Keys as JwtKeys;
use crate::server::utilities::postgres::Utility as PostgresUtility;
use anyhow::anyhow;
use anyhow::Result;
use sqlx::PgPool;
use tracing::error;
use tracing::trace;
use tracing::warn;

pub struct Utility;

impl Utility {
    pub async fn init_postgres(pool: &PgPool) -> Result<Account> {
        let admin = if let Ok(admin) = Account::new(
            None,
            config::fetch::<String>("admin_name"),
            config::fetch::<String>("admin_email"),
            config::fetch::<String>("admin_password"),
            config::fetch::<String>("admin_namespace"),
            config::fetch::<i64>("admin_ttl"),
        ) {
            admin
        } else {
            error!("failed to validate admin account");
            return Err(anyhow!("failed to validate admin account"));
        };
        match PostgresUtility::error(admin.save(&pool).await)? {
            Ok(_) => {
                trace!(
                    r#"updated admin account id: "{}" name: "{}""#,
                    admin.id().as_uuid(),
                    admin.name().as_str()
                );
                Ok(admin)
            }
            Err(e) if PostgresUtility::is_conflict(&e) => {
                warn!("confliction occured while creating admin account: {}", e);
                Ok(admin)
            }
            Err(e) => {
                error!("unknown error occured while creating admin account: {}", e);
                Err(anyhow!("error occured while updating account"))
            }
        }
    }
}
