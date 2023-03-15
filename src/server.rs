mod entities;
mod interactors;
mod repositories;
mod services;
use crate::utils;
use anyhow::Context;
use anyhow::Result;
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Server {
    pub id: Uuid,
    pub db_pool: PgPool,
    pub kvs_client: Client,
}

impl Server {
    pub async fn new() -> Result<Arc<Self>> {
        let db_pool = utils::new_pg_pool()
            .await
            .context("failed to create postgres connection pool")?;
        let kvs_client = utils::new_redis_client().context("failed to create redis client")?;
        Ok(Arc::new(Server {
            id: Uuid::new_v4(),
            db_pool,
            kvs_client,
        }))
    }

    pub async fn start(self: Arc<Self>) -> Result<()> {
        interactors::bind(self)
            .await
            .context("failed to start API server")
    }
}
