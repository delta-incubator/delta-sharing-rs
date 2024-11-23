use serde::Serialize;

pub mod v1 {
    include!("gen/delta_sharing.v1.rs");
}

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
