pub mod api;
pub mod internal;
use crate::server::Server;
use anyhow::Context;
use anyhow::Result;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Json;
use axum::Router;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;

pub struct State {
    server: Arc<Server>,
}

type SharedState = Arc<State>;

pub enum InteractorError {
    InternalServerProblem(anyhow::Error),
    BadRequest,
    Unauthorized,
    ValidationFailed,
    Conflict,
}

impl From<anyhow::Error> for InteractorError {
    fn from(e: anyhow::Error) -> Self {
        InteractorError::InternalServerProblem(e)
    }
}

impl IntoResponse for InteractorError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            InteractorError::InternalServerProblem(e) => {
                debug!("stacktrace: {}", e.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
            InteractorError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            InteractorError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            InteractorError::ValidationFailed => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Validation errors")
            }
            InteractorError::Conflict => (StatusCode::CONFLICT, "Confliction occured"),
        };
        let body = Json(json!({
            "error": message,
        }));
        (status, body).into_response()
    }
}

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
    let addr = server.config.server_bind.as_str().parse().context(format!(
        r#"failed to parse "{}" to SocketAddr"#,
        server.config.server_bind
    ))?;
    debug!("kotosiro sharing server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context(format!(
            r#"failed to bind "{}" to hyper::Server"#,
            server.config.server_bind,
        ))?;
    Ok(())
}
