pub mod admin;
pub mod root;
pub mod shares;
use crate::config;
use crate::server::schemas::ApiDoc;
use crate::utils::jwt;
use anyhow::Context;
use anyhow::Result;
use axum::extract::Extension;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use redis::Client;
use rusoto_credential::ProfileProvider;
use sqlx::PgPool;
use std::sync::Arc;
use tame_gcs::signing::ServiceAccount;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct State {
    pub pg_pool: PgPool,
    pub redis_client: Client,
    pub gcp_service_account: ServiceAccount,
    pub aws_profile_provider: ProfileProvider,
}

pub type SharedState = Arc<State>;

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

    let swagger = SwaggerUi::new(config::fetch::<String>("swagger_ui_path")).url(
        config::fetch::<String>("open_api_doc_path"),
        ApiDoc::openapi(),
    );

    let admin = Router::new()
        .route("/admin/accounts", post(self::admin::accounts::post))
        .route("/admin/accounts", get(self::admin::accounts::list))
        .route("/admin/accounts/:name", get(self::admin::accounts::get))
        .route("/admin/shares", post(self::admin::shares::post))
        .route_layer(middleware::from_fn(jwt::as_admin))
        .route("/admin/login", post(self::admin::login))
        .layer(Extension(state.clone()));

    let guest = Router::new()
        .route("/", get(self::root::get))
        .route("/shares", get(self::shares::list))
        .route("/shares/:name", get(self::shares::get))
        .route_layer(middleware::from_fn(jwt::as_guest))
        .layer(Extension(state.clone()));

    let app = Router::new().merge(swagger).merge(admin).merge(guest);

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
