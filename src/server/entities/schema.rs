use crate::impl_string_property;
use crate::impl_uuid_property;
use crate::server::entities::account::Id as AccountId;
use crate::server::entities::share::Id as ShareId;
use crate::server::entities::table::Id as TableId;
use crate::server::repositories::schema::PgRepository;
use crate::server::repositories::schema::Repository;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

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
    table_id: TableId,
    #[getset(get = "pub", set = "pub")]
    share_id: ShareId,
    #[getset(get = "pub")]
    created_by: AccountId,
}

impl Entity {
    pub fn new(
        id: impl Into<Option<String>>,
        name: String,
        table_id: String,
        share_id: String,
        created_by: String,
    ) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: Name::new(name)?,
            table_id: TableId::try_from(table_id)?,
            share_id: ShareId::try_from(share_id)?,
            created_by: AccountId::try_from(created_by)?,
        })
    }

    pub async fn register(&self, pg_pool: &PgPool) -> Result<PgQueryResult> {
        let repo = PgRepository;
        repo.upsert(&self, pg_pool).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(matches!(Id::try_from(testutils::rand::uuid()), Ok(_)));
    }

    #[test]
    fn test_invalid_id() {
        assert!(matches!(Id::try_from(testutils::rand::string(255)), Err(_)));
    }

    #[test]
    fn test_valid_name() {
        assert!(matches!(Name::new(testutils::rand::string(255)), Ok(_)));
    }

    #[test]
    fn test_invalid_name() {
        assert!(matches!(Name::new(""), Err(_)));
    }
}
