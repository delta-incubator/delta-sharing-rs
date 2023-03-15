pub mod argon2;
pub mod postgres;
use crate::config::Config;
use anyhow::Result;
use sqlx::PgPool;

pub async fn new_pg_pool(config: &Config) -> Result<PgPool> {
    postgres::connect(&config.db_url).await
}
