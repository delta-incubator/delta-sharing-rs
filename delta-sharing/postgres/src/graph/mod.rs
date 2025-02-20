use delta_sharing_common::models::internal::resource::ObjectLabel;
use delta_sharing_common::{AssociationLabel, ResourceIdent, ResourceName, ResourceRef};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod sharing;
mod store;

pub use store::Store as GraphStore;

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::FromRow)]
pub struct Object {
    /// The globally unique identifier of the object.
    pub id: Uuid,

    /// The label / tyoe of the object.
    pub label: ObjectLabel,

    /// The namespaced name of the object.
    pub name: ResourceName,

    /// The properties of the object.
    pub properties: Option<serde_json::Value>,

    /// The time when the object was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time when the object was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct ObjectRelations {
    pub owner: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Object {
    pub fn resource_ident(&self) -> ResourceIdent {
        let id = ResourceRef::Uuid(self.id);
        match self.label {
            ObjectLabel::ShareInfo => ResourceIdent::Share(id),
            ObjectLabel::SharingSchemaInfo => ResourceIdent::Schema(id),
            ObjectLabel::SharingTable => ResourceIdent::SharingTable(id),
            ObjectLabel::Credential => ResourceIdent::Credential(id),
            ObjectLabel::StorageLocation => ResourceIdent::StorageLocation(id),
            ObjectLabel::CatalogInfo => ResourceIdent::Catalog(id),
            ObjectLabel::SchemaInfo => ResourceIdent::Schema(id),
            ObjectLabel::TableInfo => ResourceIdent::Table(id),
        }
    }
}

/// Associations describe relationships between two objects.
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, PartialEq)]
pub struct Association {
    id: Uuid,

    /// Source object identifier.
    pub from_id: Uuid,

    /// The label / type of the association.
    pub label: AssociationLabel,

    /// Target object identifier.
    pub to_id: Uuid,

    /// The label / type of the target object.
    pub to_label: ObjectLabel,

    /// The properties of the association.
    pub properties: Option<serde_json::Value>,

    /// The time when the association was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time when the association was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Association {
    pub fn target_ident(&self) -> ResourceIdent {
        self.to_label.to_ident(ResourceRef::Uuid(self.to_id))
    }

    pub fn target_ref(&self) -> ResourceRef {
        ResourceRef::Uuid(self.to_id)
    }
}
