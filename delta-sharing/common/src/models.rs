use serde::Serialize;

#[allow(clippy::empty_docs, clippy::large_enum_variant)]
pub mod v1 {
    include!("gen/delta_sharing.v1.rs");
    #[cfg(feature = "grpc")]
    include!("gen/delta_sharing.v1.tonic.rs");
}

pub mod catalog {
    pub mod v1 {
        include!("gen/delta_sharing.catalog.v1.rs");
        #[cfg(feature = "grpc")]
        include!("gen/delta_sharing.catalog.v1.tonic.rs");
    }
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
