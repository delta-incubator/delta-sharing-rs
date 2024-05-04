#[allow(dead_code)]
pub mod types {
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorResponse {
        pub error_code: String,
        pub message: String,
    }

    pub struct TableRef {
        pub share: String,
        pub schema: String,
        pub table: String,
    }

    include!("gen/delta_sharing.v1.rs");
}
pub mod discovery;
pub mod error;
pub mod policies;
pub mod query;

pub use discovery::*;
pub use error::*;
pub use query::*;
pub use types::*;
