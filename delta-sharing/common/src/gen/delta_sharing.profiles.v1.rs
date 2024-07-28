// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProfileRequest {
    /// serialized profile claims
    #[prost(bytes="bytes", tag="1")]
    pub claims: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProfileResponse {
    /// the profile file
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<super::super::v1::Profile>,
}
include!("delta_sharing.profiles.v1.serde.rs");
// @@protoc_insertion_point(module)