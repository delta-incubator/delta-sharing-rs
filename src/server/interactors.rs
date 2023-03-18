pub mod api;
pub mod internal;
use crate::config;
use anyhow::Context;
use anyhow::Result;
use axum::extract::Extension;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;
use tame_gcs::signing::ServiceAccount;
use tracing::debug;

pub struct State {
    pub pg_pool: PgPool,
    pub redis_client: Client,
    pub gcp_service_account: ServiceAccount,
}

type SharedState = Arc<State>;

async fn route(
    pg_pool: PgPool,
    redis_client: Client,
    gcp_service_account: ServiceAccount,
) -> Result<Router> {
    let state = Arc::new(State {
        pg_pool,
        redis_client,
        gcp_service_account,
    });
    let app = Router::new()
        .route(
            "/api/register",
            post(self::api::account::register).put(self::api::account::register),
        )
        .route(
            "/api/login",
            post(self::api::account::login).put(self::api::account::login),
        )
        .route("/api/profile", get(self::api::account::profile))
        .layer(Extension(state));
    Ok(app)
}

pub async fn bind(
    pg_pool: PgPool,
    redis_client: Client,
    gcp_service_account: ServiceAccount,
) -> Result<()> {
    let app = route(pg_pool, redis_client, gcp_service_account)
        .await
        .context("failed to create axum router")?;
    let server_bind = config::fetch::<String>("server_bind");
    let addr = server_bind.as_str().parse().context(format!(
        r#"failed to parse "{}" to SocketAddr"#,
        server_bind
    ))?;
    debug!("kotosiro sharing server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context(format!(
            r#"failed to bind "{}" to hyper::Server"#,
            server_bind,
        ))?;
    Ok(())
}
