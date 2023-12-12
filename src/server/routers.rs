pub mod catalog;
pub mod sharing;

use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::Extension;
use axum::http::{header, Method, Uri};
use axum::middleware;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
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
}

pub type SharedState = Arc<State>;

async fn bad_request(_: Uri) -> std::result::Result<Response, Error> {
    Err(Error::BadRequest)
}

async fn route(
    pg_pool: PgPool,
    gcp_service_account: Option<ServiceAccount>,
    aws_credentials: Option<AwsCredentials>,
) -> Result<Router> {
    let state = Arc::new(State {
        pg_pool,
        gcp_service_account,
        aws_credentials,
    });

    let swagger = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());

    let catalog = Router::new()
        .route("/catalog/profile", get(self::catalog::profile))
        .route("/catalog/accounts", post(self::catalog::accounts::post))
        .route("/catalog/accounts", get(self::catalog::accounts::list))
        .route(
            "/catalog/accounts/:account",
            get(self::catalog::accounts::get),
        )
        .route("/catalog/shares", post(self::catalog::shares::post))
        .route(
            "/catalog/shares/:share/schemas",
            post(catalog::shares::schemas::post),
        )
        .route(
            "/catalog/shares/:share/schemas/:schema/tables",
            post(catalog::shares::schemas::tables::post),
        )
        .route_layer(middleware::from_fn(jwt::as_admin))
        .route("/catalog/login", post(self::catalog::login))
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

    let sharing = Router::new()
        .route("/sharing/shares", get(self::sharing::shares::list))
        .route("/sharing/shares/:share", get(self::sharing::shares::get))
        .route(
            "/sharing/shares/:share/all-tables",
            get(self::sharing::shares::all_tables::list),
        )
        .route(
            "/sharing/shares/:share/schemas",
            get(self::sharing::shares::schemas::list),
        )
        .route(
            "/sharing/shares/:share/schemas/:schema/tables",
            get(self::sharing::shares::schemas::tables::list),
        )
        .route(
            "/sharing/shares/:share/schemas/:schema/tables/:table/version",
            get(self::sharing::shares::schemas::tables::version::get),
        )
        .route(
            "/sharing/shares/:share/schemas/:schema/tables/:table/metadata",
            get(self::sharing::shares::schemas::tables::metadata::get),
        )
        .route(
            "/sharing/shares/:share/schemas/:schema/tables/:table/query",
            post(self::sharing::shares::schemas::tables::query::post),
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
        .merge(catalog)
        .merge(sharing)
        .fallback(bad_request);

    Ok(app)
}

pub async fn bind(
    pg_pool: PgPool,
    gcp_service_account: Option<ServiceAccount>,
    aws_credentials: Option<AwsCredentials>,
) -> Result<()> {
    let app = route(pg_pool, gcp_service_account, aws_credentials)
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
