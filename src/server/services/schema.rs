use crate::server::entities::schema::Entity as SchemaEntity;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
}

impl Schema {
    pub fn from(entity: SchemaEntity) -> Self {
        Self {
            id: entity.id().to_uuid(),
            name: entity.name().to_string(),
        }
    }
}
