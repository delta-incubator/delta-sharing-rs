pub mod admin;
pub mod api;
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
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use std::sync::Arc;
use tame_gcs::signing::ServiceAccount;
use tracing::debug;

pub struct State {
    pub pg_pool: PgPool,
    pub redis_client: Client,
    pub gcp_service_account: ServiceAccount,
    pub aws_profile_provider: ProfileProvider,
}

type SharedState = Arc<State>;

async fn route(
    pg_pool: PgPool,
    redis_client: Client,
    gcp_service_account: ServiceAccount,
    aws_profile_provider: ProfileProvider,
) -> Result<Router> {
    let state = Arc::new(State {
        pg_pool,
        redis_client,
        gcp_service_account,
        aws_profile_provider,
    });
    let app = Router::new()
        .route("/admin/login", post(self::admin::login))
        .route("/admin/register", post(self::admin::register))
        //        .route("/admin/accounts", get(self::admin::accounts))
        .route("/api/user/profile", get(self::api::user::profile))
        .layer(Extension(state));
    Ok(app)
}

pub async fn bind(
    pg_pool: PgPool,
    redis_client: Client,
    gcp_service_account: ServiceAccount,
    aws_profile_provider: ProfileProvider,
) -> Result<()> {
    let app = route(
        pg_pool,
        redis_client,
        gcp_service_account,
        aws_profile_provider,
    )
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
