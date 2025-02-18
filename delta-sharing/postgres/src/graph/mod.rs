use delta_sharing_common::{AssociationLabel, ResourceIdent, ResourceRef};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod sharing;
mod store;

pub use store::Store as GraphStore;

// IMPORTANT: Any changes to the schema must be reflected in the migrations.
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "object_label", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ObjectLabel {
    DeltaShare,
    DeltaSchema,
    Table,
    Principal,
    Credential,
    StorageLocation,
}

impl ObjectLabel {
    pub fn to_ident(&self, id: impl Into<ResourceRef>) -> ResourceIdent {
        let id = id.into();
        match self {
            ObjectLabel::DeltaShare => ResourceIdent::Share(id),
            ObjectLabel::DeltaSchema => ResourceIdent::Schema(id),
            ObjectLabel::Table => ResourceIdent::Table(id),
            ObjectLabel::Principal => todo!(),
            ObjectLabel::Credential => ResourceIdent::Credential(id),
            ObjectLabel::StorageLocation => ResourceIdent::StorageLocation(id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::FromRow)]
pub struct Object {
    /// The globally unique identifier of the object.
    pub id: Uuid,

    /// The label / tyoe of the object.
    pub label: ObjectLabel,

    /// The namespace of the object.
    pub namespace: Vec<String>,

    /// The name of the object.
    ///
    /// The name of the object is unique within the namespace.
    pub name: String,

    /// The properties of the object.
    pub properties: Option<serde_json::Value>,

    /// The time when the object was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time when the object was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Object {
    pub fn resource_ident(&self) -> ResourceIdent {
        let id = ResourceRef::Uuid(self.id.clone());
        match self.label {
            ObjectLabel::DeltaShare => ResourceIdent::Share(id),
            ObjectLabel::DeltaSchema => ResourceIdent::Schema(id),
            ObjectLabel::Table => ResourceIdent::Table(id),
            ObjectLabel::Credential => ResourceIdent::Credential(id),
            ObjectLabel::StorageLocation => ResourceIdent::StorageLocation(id),
            ObjectLabel::Principal => todo!(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, PartialEq)]
pub struct Association {
    id: Uuid,

    /// Source object identifier.
    pub from_id: uuid::Uuid,

    /// The label / type of the association.
    pub label: AssociationLabel,

    /// Target object identifier.
    pub to_id: uuid::Uuid,

    /// The label / type of the target object.
    pub to_label: ObjectLabel,

    /// The properties of the association.
    pub properties: Option<serde_json::Value>,

    /// The time when the association was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time when the association was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
