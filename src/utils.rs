pub mod argon2;
pub mod postgres;
pub mod redis;
use crate::config;
use anyhow::Result;
use sqlx::PgPool;

pub async fn new_pg_pool() -> Result<PgPool> {
    postgres::connect(&config::fetch::<String>("db_url")).await
}

pub fn new_redis_client() -> Result<redis::Client> {
    redis::connect(&config::fetch::<String>("kvs_url"))
}
