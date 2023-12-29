pub mod admin;
pub mod shares;

use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::Extension;
use axum::http::{header, Method, Uri};
use axum::middleware;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use azure_storage::StorageCredentials;
use rusoto_credential::AwsCredentials;
use sqlx::PgPool;
use tame_gcs::signing::ServiceAccount;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config;
use crate::server::api_doc::ApiDoc;
use crate::server::middlewares::jwt;
use crate::server::services::error::Error;

pub struct State {
    pub pg_pool: PgPool,
    pub gcp_service_account: Option<ServiceAccount>,
    pub aws_credentials: Option<AwsCredentials>,
    pub azure_credentials: Option<StorageCredentials>,
}

pub type SharedState = Arc<State>;

async fn bad_request(_: Uri) -> std::result::Result<Response, Error> {
    Err(Error::BadRequest)
}

async fn route(
    pg_pool: PgPool,
    gcp_service_account: Option<ServiceAccount>,
    aws_credentials: Option<AwsCredentials>,
    azure_credentials: Option<StorageCredentials>,
) -> Result<Router> {
    let state = Arc::new(State {
        pg_pool,
        gcp_service_account,
        aws_credentials,
        azure_credentials,
    });

    let swagger = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());

    let admin = Router::new()
        .route("/admin/profile", get(self::admin::profile))
        .route("/admin/accounts", post(self::admin::accounts::post))
        .route("/admin/accounts", get(self::admin::accounts::list))
        .route("/admin/accounts/:account", get(self::admin::accounts::get))
        .route("/admin/shares", post(self::admin::shares::post))
        .route(
            "/admin/shares/:share/schemas",
            post(admin::shares::schemas::post),
        )
        .route(
            "/admin/shares/:share/schemas/:schema/tables",
            post(admin::shares::schemas::tables::post),
        )
        .route_layer(middleware::from_fn(jwt::as_admin))
        .route("/admin/login", post(self::admin::login))
        .layer(Extension(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "http://localhost:3000"
                        .parse::<header::HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::HEAD])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_credentials(true),
        );

    let guest = Router::new()
        .route("/shares", get(self::shares::list))
        .route("/shares/:share", get(self::shares::get))
        .route(
            "/shares/:share/all-tables",
            get(self::shares::all_tables::list),
        )
        .route("/shares/:share/schemas", get(self::shares::schemas::list))
        .route(
            "/shares/:share/schemas/:schema/tables",
            get(self::shares::schemas::tables::list),
        )
        .route(
            "/shares/:share/schemas/:schema/tables/:table/version",
            get(self::shares::schemas::tables::version::get),
        )
        .route(
            "/shares/:share/schemas/:schema/tables/:table/metadata",
            get(self::shares::schemas::tables::metadata::get),
        )
        .route(
            "/shares/:share/schemas/:schema/tables/:table/query",
            post(self::shares::schemas::tables::query::post),
        )
        .route_layer(middleware::from_fn(jwt::as_guest))
        .layer(Extension(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "http://localhost:3000"
                        .parse::<header::HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::HEAD])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_credentials(true),
        );

    let app = Router::new()
        .merge(swagger)
        .merge(admin)
        .merge(guest)
        .fallback(bad_request);

    Ok(app)
}

pub async fn bind(
    pg_pool: PgPool,
    gcp_service_account: Option<ServiceAccount>,
    aws_credentials: Option<AwsCredentials>,
    azure_credentials: Option<StorageCredentials>,
) -> Result<()> {
    let app = route(
        pg_pool,
        gcp_service_account,
        aws_credentials,
        azure_credentials,
    )
    .await
    .context("failed to create axum router")?;
    let server_bind = config::fetch::<String>("server_bind");
    let addr = server_bind.as_str().parse().context(format!(
        r#"failed to parse "{}" to SocketAddr"#,
        server_bind
    ))?;
    tracing::info!("delta sharing server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context(format!(
            r#"failed to bind "{}" to hyper::Server"#,
            server_bind,
        ))?;
    Ok(())
}
