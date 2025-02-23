use delta_sharing_common::{AssociationLabel, Object, ObjectLabel, ResourceIdent, ResourceRef};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod sharing;
mod store;

pub use store::Store as GraphStore;

pub struct ObjectRelations {
    pub owner: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
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
