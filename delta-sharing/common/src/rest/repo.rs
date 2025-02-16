use axum::extract::{Extension, State};
use axum::{routing::get, Json, Router};

use crate::models::catalog::v1::*;
use crate::models::v1::*;
use crate::{Policy, Recipient, Result, SharingRepository};

/// Create a new [Router] for the Delta Sharing REST API.
pub fn get_router<T>(handler: T) -> Router
where
    T: SharingRepository + Policy + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/shares", get(create_share::<T>))
        .with_state(handler)
}

async fn create_share<T: SharingRepository + Policy>(
    State(handler): State<T>,
    Extension(recipient): Extension<Recipient>,
    request: CreateShareRequest,
) -> Result<Json<Share>> {
    handler.check_required(&request, &recipient).await?;
    let share = handler
        .add_share(&request.share.unwrap().name, None, None)
        .await?;
    Ok(Json(share))
}
