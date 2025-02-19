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
    /// A share in the delta shaing service.
    DeltaShare,

    /// A schema in the delta sharing service.
    DeltaSchema,

    /// A data table.
    Table,

    /// A credential for accessing an external resource or storage location.
    Credential,

    /// A storage location where data is stored.
    ///
    /// THe stored data may represent a table, a file, a model, etc.
    StorageLocation,

    Principal,
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
        let id = ResourceRef::Uuid(self.id);
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

/// Associations describe relationships between two objects.
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

impl Association {
    pub fn target_ident(&self) -> ResourceIdent {
        self.to_label
            .to_ident(ResourceRef::Uuid(self.to_id))
    }

    pub fn target_ref(&self) -> ResourceRef {
        ResourceRef::Uuid(self.to_id)
    }
}
