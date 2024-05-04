mod traits;
#[allow(dead_code)]
mod types {
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorResponse {
        pub error_code: String,
        pub message: String,
    }

    include!("gen/delta_sharing.v1.rs");
}
mod error;
pub mod handlers;
pub mod policies;

pub use error::*;
pub use traits::*;
pub use types::*;
