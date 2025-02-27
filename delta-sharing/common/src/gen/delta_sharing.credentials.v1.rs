// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureServicePrincipal {
    /// The directory ID corresponding to the Azure Active Directory (AAD) tenant of the application.
    #[prost(string, tag="1")]
    pub directory_id: ::prost::alloc::string::String,
    /// The application ID of the application registration within the referenced AAD tenant.
    #[prost(string, tag="2")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(oneof="azure_service_principal::Credential", tags="3, 5")]
    pub credential: ::core::option::Option<azure_service_principal::Credential>,
}
/// Nested message and enum types in `AzureServicePrincipal`.
pub mod azure_service_principal {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// The client secret generated for the above app ID in AAD.
        #[prost(string, tag="3")]
        ClientSecret(::prost::alloc::string::String),
        /// Location of the file containing a federated token.
        ///
        /// Specifically useful for workload identity federation.
        #[prost(string, tag="5")]
        FederatedTokenFile(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureManagedIdentity {
    #[prost(oneof="azure_managed_identity::Identifier", tags="1, 2, 3")]
    pub identifier: ::core::option::Option<azure_managed_identity::Identifier>,
}
/// Nested message and enum types in `AzureManagedIdentity`.
pub mod azure_managed_identity {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// Object id for use with managed identity authentication
        #[prost(string, tag="1")]
        ObjectId(::prost::alloc::string::String),
        /// The application ID of the application registration within the referenced AAD tenant.
        #[prost(string, tag="2")]
        ApplicationId(::prost::alloc::string::String),
        /// Msi resource id for use with managed identity authentication
        #[prost(string, tag="3")]
        MsiResourceId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CredentialInfo {
    /// The unique identifier of the credential.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The credential name.
    ///
    /// The name must be unique among storage and service credentials within the metastore.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Indicates the purpose of the credential.
    #[prost(enumeration="Purpose", tag="3")]
    pub purpose: i32,
    /// Whether the credential is usable only for read operations.
    ///
    /// Only applicable when purpose is STORAGE.
    #[prost(bool, tag="4")]
    pub read_only: bool,
    /// User-provided free-form text description.
    #[prost(string, optional, tag="5")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// Username of current owner of credential.
    #[prost(string, optional, tag="6")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this credential was created, in epoch milliseconds.
    #[prost(int64, optional, tag="7")]
    pub created_at: ::core::option::Option<i64>,
    /// Username of credential creator.
    #[prost(string, optional, tag="8")]
    pub created_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which this credential was last updated, in epoch milliseconds.
    #[prost(int64, optional, tag="9")]
    pub updated_at: ::core::option::Option<i64>,
    /// Username of user who last modified credential.
    #[prost(string, optional, tag="10")]
    pub updated_by: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether this credential is the current metastore's root storage credential.
    ///
    /// Only applicable when purpose is STORAGE.
    #[prost(bool, tag="11")]
    pub used_for_managed_storage: bool,
    /// The full name of the credential.
    #[prost(string, optional, tag="12")]
    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="credential_info::Credential", tags="100, 101")]
    pub credential: ::core::option::Option<credential_info::Credential>,
}
/// Nested message and enum types in `CredentialInfo`.
pub mod credential_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        #[prost(message, tag="100")]
        AzureServicePrincipal(super::AzureServicePrincipal),
        #[prost(message, tag="101")]
        AzureManagedIdentity(super::AzureManagedIdentity),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Purpose {
    Unspecified = 0,
    Storage = 1,
    Service = 2,
}
impl Purpose {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Purpose::Unspecified => "PURPOSE_UNSPECIFIED",
            Purpose::Storage => "STORAGE",
            Purpose::Service => "SERVICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PURPOSE_UNSPECIFIED" => Some(Self::Unspecified),
            "STORAGE" => Some(Self::Storage),
            "SERVICE" => Some(Self::Service),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCredentialsRequest {
    /// The maximum number of results per page that should be returned.
    #[prost(int32, optional, tag="2")]
    pub max_results: ::core::option::Option<i32>,
    /// Opaque pagination token to go to next page based on previous query.
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Return only credentials for the specified purpose.
    #[prost(enumeration="Purpose", optional, tag="4")]
    pub purpose: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCredentialsResponse {
    /// The credentials returned.
    #[prost(message, repeated, tag="1")]
    pub credentials: ::prost::alloc::vec::Vec<CredentialInfo>,
    /// The next_page_token value to include in the next List request.
    #[prost(string, optional, tag="2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Create a new credential
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCredentialRequest {
    /// The credential name. The name must be unique among storage and service credentials within the metastore.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The credential purpose.
    #[prost(enumeration="Purpose", tag="2")]
    pub purpose: i32,
    /// Comment associated with the credential.
    #[prost(string, optional, tag="3")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether the credential is usable only for read operations. Only applicable when purpose is STORAGE.
    #[prost(bool, optional, tag="4")]
    pub read_only: ::core::option::Option<bool>,
    /// Supplying true to this argument skips validation of the created set of credentials.
    #[prost(bool, tag="5")]
    pub skip_validation: bool,
    #[prost(oneof="create_credential_request::Credential", tags="100, 101")]
    pub credential: ::core::option::Option<create_credential_request::Credential>,
}
/// Nested message and enum types in `CreateCredentialRequest`.
pub mod create_credential_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        #[prost(message, tag="100")]
        AzureServicePrincipal(super::AzureServicePrincipal),
        #[prost(message, tag="101")]
        AzureManagedIdentity(super::AzureManagedIdentity),
    }
}
/// Get a credential
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialRequest {
    /// Name of credential.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Update a credential
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCredentialRequest {
    /// Name of credential.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Name of credential.
    #[prost(string, optional, tag="2")]
    pub new_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Comment associated with the credential.
    #[prost(string, optional, tag="3")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether the credential is usable only for read operations. Only applicable when purpose is STORAGE.
    #[prost(bool, optional, tag="4")]
    pub read_only: ::core::option::Option<bool>,
    /// Username of current owner of credential.
    #[prost(string, optional, tag="5")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    /// Supply true to this argument to skip validation of the updated credential.
    #[prost(bool, optional, tag="6")]
    pub skip_validation: ::core::option::Option<bool>,
    /// Force an update even if there are dependent services (when purpose is SERVICE)
    /// or dependent external locations and external tables (when purpose is STORAGE).
    #[prost(bool, optional, tag="7")]
    pub force: ::core::option::Option<bool>,
    #[prost(oneof="update_credential_request::Credential", tags="100, 101")]
    pub credential: ::core::option::Option<update_credential_request::Credential>,
}
/// Nested message and enum types in `UpdateCredentialRequest`.
pub mod update_credential_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        #[prost(message, tag="100")]
        AzureServicePrincipal(super::AzureServicePrincipal),
        #[prost(message, tag="101")]
        AzureManagedIdentity(super::AzureManagedIdentity),
    }
}
/// Delete a credential
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCredentialRequest {
    /// Name of credential.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
include!("delta_sharing.credentials.v1.serde.rs");
// @@protoc_insertion_point(module)