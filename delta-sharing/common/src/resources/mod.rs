use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use name::*;
pub use store::*;

use crate::models::ObjectLabel;

mod name;
mod store;

/// Unique identifier for a resource.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ResourceRef {
    Uuid(Uuid),
    Name(ResourceName),
    /// Not referencing a specific resource.
    ///
    /// This is used to represent a wildcard in a policy
    /// which can be useful to check if a user can create
    /// or manage resources at a specific level.
    Undefined,
}

impl ResourceRef {
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    pub fn name(name: impl Into<ResourceName>) -> Self {
        Self::Name(name.into())
    }
}

impl std::fmt::Display for ResourceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uuid(u) => write!(f, "{}", u.hyphenated()),
            Self::Name(name) => {
                write!(f, "{}", name)
            }
            Self::Undefined => write!(f, "*"),
        }
    }
}

impl From<Uuid> for ResourceRef {
    fn from(val: Uuid) -> Self {
        Self::Uuid(val)
    }
}

impl From<&Uuid> for ResourceRef {
    fn from(val: &Uuid) -> Self {
        Self::Uuid(*val)
    }
}

impl From<ResourceName> for ResourceRef {
    fn from(val: ResourceName) -> Self {
        Self::Name(val)
    }
}

/// Resource that a policy can authorize.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceIdent {
    Share(ResourceRef),
    SharingSchema(ResourceRef),
    SharingTable(ResourceRef),
    Credential(ResourceRef),
    ExternalLocation(ResourceRef),
    Catalog(ResourceRef),
    Schema(ResourceRef),
    Table(ResourceRef),
    Recipient(ResourceRef),
    Column(ResourceRef),
}

impl ResourceIdent {
    pub fn label(&self) -> &ObjectLabel {
        self.as_ref()
    }

    pub fn reference(&self) -> &ResourceRef {
        self.as_ref()
    }

    pub fn share(name: impl Into<ResourceRef>) -> Self {
        Self::Share(name.into())
    }

    pub fn sharing_schema(name: impl Into<ResourceRef>) -> Self {
        Self::SharingSchema(name.into())
    }

    pub fn sharing_table(name: impl Into<ResourceRef>) -> Self {
        Self::SharingTable(name.into())
    }

    pub fn credential(name: impl Into<ResourceRef>) -> Self {
        Self::Credential(name.into())
    }

    pub fn catalog(name: impl Into<ResourceRef>) -> Self {
        Self::Catalog(name.into())
    }

    pub fn schema(name: impl Into<ResourceRef>) -> Self {
        Self::Schema(name.into())
    }

    pub fn table(name: impl Into<ResourceRef>) -> Self {
        Self::Table(name.into())
    }

    pub fn column(name: impl Into<ResourceRef>) -> Self {
        Self::Column(name.into())
    }

    pub fn external_location(name: impl Into<ResourceRef>) -> Self {
        Self::ExternalLocation(name.into())
    }

    pub fn recipient(name: impl Into<ResourceRef>) -> Self {
        Self::Recipient(name.into())
    }
}

impl std::fmt::Display for ResourceIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceIdent::Share(r) => write!(f, "share:{}", r),
            ResourceIdent::SharingSchema(r) => write!(f, "schema:{}", r),
            ResourceIdent::SharingTable(r) => write!(f, "table:{}", r),
            ResourceIdent::Credential(r) => write!(f, "credential:{}", r),
            ResourceIdent::ExternalLocation(r) => write!(f, "external_location:{}", r),
            ResourceIdent::Catalog(r) => write!(f, "catalog:{}", r),
            ResourceIdent::Schema(r) => write!(f, "schema:{}", r),
            ResourceIdent::Table(r) => write!(f, "table:{}", r),
            ResourceIdent::Recipient(r) => write!(f, "recipient:{}", r),
            ResourceIdent::Column(r) => write!(f, "column:{}", r),
        }
    }
}

impl AsRef<ResourceRef> for ResourceIdent {
    fn as_ref(&self) -> &ResourceRef {
        match self {
            ResourceIdent::Share(r) => r,
            ResourceIdent::SharingSchema(r) => r,
            ResourceIdent::SharingTable(r) => r,
            ResourceIdent::Credential(r) => r,
            ResourceIdent::ExternalLocation(r) => r,
            ResourceIdent::Catalog(r) => r,
            ResourceIdent::Schema(r) => r,
            ResourceIdent::Table(r) => r,
            ResourceIdent::Recipient(r) => r,
            ResourceIdent::Column(r) => r,
        }
    }
}

impl AsRef<ObjectLabel> for ResourceIdent {
    fn as_ref(&self) -> &ObjectLabel {
        match self {
            ResourceIdent::Share(_) => &ObjectLabel::ShareInfo,
            ResourceIdent::SharingSchema(_) => &ObjectLabel::SharingSchemaInfo,
            ResourceIdent::SharingTable(_) => &ObjectLabel::SharingTable,
            ResourceIdent::Credential(_) => &ObjectLabel::CredentialInfo,
            ResourceIdent::ExternalLocation(_) => &ObjectLabel::ExternalLocationInfo,
            ResourceIdent::Catalog(_) => &ObjectLabel::CatalogInfo,
            ResourceIdent::Schema(_) => &ObjectLabel::SchemaInfo,
            ResourceIdent::Table(_) => &ObjectLabel::TableInfo,
            ResourceIdent::Recipient(_) => &ObjectLabel::RecipientInfo,
            ResourceIdent::Column(_) => &ObjectLabel::ColumnInfo,
        }
    }
}

impl From<ResourceIdent> for ResourceRef {
    fn from(ident: ResourceIdent) -> Self {
        match ident {
            ResourceIdent::Share(r) => r,
            ResourceIdent::SharingSchema(r) => r,
            ResourceIdent::SharingTable(r) => r,
            ResourceIdent::Credential(r) => r,
            ResourceIdent::ExternalLocation(r) => r,
            ResourceIdent::Catalog(r) => r,
            ResourceIdent::Schema(r) => r,
            ResourceIdent::Table(r) => r,
            ResourceIdent::Recipient(r) => r,
            ResourceIdent::Column(r) => r,
        }
    }
}

impl From<&ResourceIdent> for ResourceRef {
    fn from(ident: &ResourceIdent) -> Self {
        (ident as &dyn AsRef<ResourceRef>).as_ref().clone()
    }
}

impl From<&ResourceIdent> for ObjectLabel {
    fn from(ident: &ResourceIdent) -> Self {
        *(ident as &dyn AsRef<ObjectLabel>).as_ref()
    }
}

impl From<ResourceIdent> for ObjectLabel {
    fn from(ident: ResourceIdent) -> Self {
        (&ident).into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "association_label", rename_all = "snake_case")
)]
pub enum AssociationLabel {
    OwnedBy,
    OwnerOf,
    DependsOn,
    DependencyOf,
    ParentOf,
    ChildOf,
    HasPart,
    PartOf,
    References,
    ReferencedBy,
}

impl AssociationLabel {
    /// Get the inverse of the association label.
    ///
    /// Associations may be bidirectional, either symmetric or asymmetric.
    /// Symmetric types are their own inverse. Asymmetric types have a distinct inverse.
    pub fn inverse(&self) -> Option<Self> {
        match self {
            AssociationLabel::HasPart => Some(AssociationLabel::PartOf),
            AssociationLabel::PartOf => Some(AssociationLabel::HasPart),
            AssociationLabel::DependsOn => Some(AssociationLabel::DependencyOf),
            AssociationLabel::DependencyOf => Some(AssociationLabel::DependsOn),
            AssociationLabel::ParentOf => Some(AssociationLabel::ChildOf),
            AssociationLabel::ChildOf => Some(AssociationLabel::ParentOf),
            AssociationLabel::References => Some(AssociationLabel::ReferencedBy),
            AssociationLabel::ReferencedBy => Some(AssociationLabel::References),
            AssociationLabel::OwnedBy => Some(AssociationLabel::OwnerOf),
            AssociationLabel::OwnerOf => Some(AssociationLabel::OwnedBy),
        }
    }
}
