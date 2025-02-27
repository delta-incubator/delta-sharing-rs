// @generated
// This file is @generated by prost-build.
/// Dummy message to hold all resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(oneof="resource::Resource", tags="1, 2, 3, 4, 6, 7, 8, 9, 10")]
    pub resource: ::core::option::Option<resource::Resource>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(::strum::EnumDiscriminants)]
    #[strum_discriminants(name(ObjectLabel))]
    #[strum_discriminants(derive(::strum::AsRefStr, ::strum::Display, ::strum::EnumIter, ::strum::EnumString), strum(serialize_all = "snake_case"))]
    #[strum_discriminants(derive(::serde::Serialize, ::serde::Deserialize, Hash))]
    #[strum_discriminants(serde(rename_all = "snake_case"))]
    #[strum_discriminants(strum(ascii_case_insensitive))]
    #[strum_discriminants(cfg_attr(feature = "sqlx", derive(::sqlx::Type)))]
    #[strum_discriminants(cfg_attr(feature = "sqlx", sqlx(type_name = "object_label", rename_all = "snake_case")))]
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(message, tag="1")]
        ShareInfo(super::super::shares::v1::ShareInfo),
        #[prost(message, tag="2")]
        SharingSchemaInfo(super::super::sharing::v1::SharingSchemaInfo),
        #[prost(message, tag="3")]
        SharingTable(super::super::sharing::v1::SharingTable),
        #[prost(message, tag="4")]
        CredentialInfo(super::super::credentials::v1::CredentialInfo),
        #[prost(message, tag="6")]
        CatalogInfo(super::super::catalogs::v1::CatalogInfo),
        #[prost(message, tag="7")]
        SchemaInfo(super::super::schemas::v1::SchemaInfo),
        #[prost(message, tag="8")]
        TableInfo(super::super::tables::v1::TableInfo),
        #[prost(message, tag="9")]
        ExternalLocationInfo(super::super::external_locations::v1::ExternalLocationInfo),
        #[prost(message, tag="10")]
        RecipientInfo(super::super::recipients::v1::RecipientInfo),
    }
}
include!("delta_sharing.internal.serde.rs");
// @@protoc_insertion_point(module)