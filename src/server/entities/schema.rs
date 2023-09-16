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
use crate::server::entities::share::Id as ShareId;
use crate::server::repositories::schema::Repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

impl_uuid_property!(Id);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Name {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(Name);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    name: Name,
    #[getset(get = "pub", set = "pub")]
    share_id: ShareId,
    #[getset(get = "pub")]
    created_by: AccountId,
}

impl Entity {
    pub fn new(
        id: impl Into<Option<String>>,
        name: String,
        share_id: String,
        created_by: String,
    ) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: Name::new(name)?,
            share_id: ShareId::try_from(share_id)?,
            created_by: AccountId::try_from(created_by)?,
        })
    }

    pub async fn load(share_id: &ShareId, name: &Name, pg_pool: &PgPool) -> Result<Option<Self>> {
        match Repository::select_by_name(share_id, name, pg_pool).await? {
            Some(row) => Ok(Self {
                id: Id::new(row.id),
                name: Name::new(row.name)?,
                share_id: ShareId::new(row.share_id),
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
        assert!(Name::new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_name() {
        assert!(Name::new("").is_err());
    }
}
