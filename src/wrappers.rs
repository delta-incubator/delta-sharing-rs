pub mod argon2;
pub mod postgres;
use crate::config;
use anyhow::Result;
use sqlx::PgPool;

pub async fn new_pg_pool() -> Result<PgPool> {
    postgres::connect(&config::fetch::<String>("db_url")).await
}
