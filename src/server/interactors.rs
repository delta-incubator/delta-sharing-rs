pub mod api;
pub mod internal;
use crate::config;
use crate::server::Server;
use anyhow::Context;
use anyhow::Result;
use axum::extract::Extension;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;
use std::sync::Arc;
use tracing::debug;

pub struct State {
    server: Arc<Server>,
}

type SharedState = Arc<State>;

async fn route(server: Arc<Server>) -> Result<Router> {
    let state = Arc::new(State { server });
    let app = Router::new()
        .route(
            "/api/account",
            post(self::api::account::create).put(self::api::account::create),
        )
        //        .route(
        //            "/api/account/:id",
        //            get(self::api::account::get_by_id).delete(self::api::account::delete),
        //        )
        .layer(Extension(state));
    Ok(app)
}

pub async fn bind(server: Arc<Server>) -> Result<()> {
    let app = route(server.clone())
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
