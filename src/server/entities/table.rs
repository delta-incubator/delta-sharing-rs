use anyhow::Result;
use getset::Getters;
use getset::Setters;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::impl_string_property;
use crate::impl_uuid_property;
use crate::server::entities::account::Id as AccountId;
use crate::server::entities::schema::Id as SchemaId;
use crate::server::repositories::table::Repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Name {
    #[validate(length(min = 1))]
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Location {
    #[validate(length(min = 1))]
    value: String,
}

impl_uuid_property!(Id);
impl_string_property!(Name);
impl_string_property!(Location);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    name: Name,
    #[getset(get = "pub", set = "pub")]
    schema_id: SchemaId,
    #[getset(get = "pub", set = "pub")]
    location: Location,
    #[getset(get = "pub")]
    created_by: AccountId,
}

impl Entity {
    pub fn new(
        id: impl Into<Option<String>>,
        name: String,
        schema_id: String,
        location: String,
        created_by: String,
    ) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: Name::try_new(name)?,
            schema_id: SchemaId::try_from(schema_id)?,
            location: Location::try_new(location)?,
            created_by: AccountId::try_from(created_by)?,
        })
    }

    pub async fn load(schema_id: &SchemaId, name: &Name, pg_pool: &PgPool) -> Result<Option<Self>> {
        match Repository::select_by_name(schema_id, name, pg_pool).await? {
            Some(row) => Ok(Self {
                id: Id::new(row.id),
                name: Name::try_new(row.name)?,
                schema_id: SchemaId::new(row.schema_id),
                location: Location::try_new(row.location)?,
                created_by: AccountId::new(row.created_by),
            }
            .into()),
            _ => Ok(None),
        }
    }

    pub async fn save(&self, pg_pool: &PgPool) -> Result<PgQueryResult> {
        Repository::upsert(self, pg_pool).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(Id::try_from(testutils::rand::uuid()).is_ok());
    }

    #[test]
    fn test_invalid_id() {
        assert!(Id::try_from(testutils::rand::string(255)).is_err());
    }

    #[test]
    fn test_valid_name() {
        assert!(Name::try_new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_name() {
        assert!(Name::try_new("").is_err());
    }

    #[test]
    fn test_valid_location() {
        assert!(Location::try_new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_location() {
        assert!(Location::try_new("").is_err());
    }
}
