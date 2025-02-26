use axum::routing::{delete, get, post, Router};

use crate::api::credentials::*;
use crate::api::CredentialsHandler;

pub fn get_router<T: CredentialsHandler + Clone>(handler: T) -> Router {
    Router::new()
        .route("/credentials", post(create_credential::<T>))
        .route("/credentials/{name}", get(get_credential::<T>))
        //.route("/credentials/{name}", patch(update_credential::<T>))
        .route("/credentials/{name}", delete(delete_credential::<T>))
        .with_state(handler)
}
