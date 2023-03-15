mod entities;
mod interactors;
mod repositories;
mod services;
use crate::wrappers;
use anyhow::Context;
use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Server {
    pub id: Uuid,
    pub db_pool: PgPool,
}

impl Server {
    pub async fn new() -> Result<Arc<Self>> {
        let db_pool = wrappers::new_pg_pool()
            .await
            .context("failed to create postgres connection pool")?;
        Ok(Arc::new(Server {
            id: Uuid::new_v4(),
            db_pool,
        }))
    }

    pub async fn start(self: Arc<Self>) -> Result<()> {
        interactors::bind(self)
            .await
            .context("failed to start API server")
    }
}
