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
use crate::server::middlewares::jwt::Role;
use crate::server::repositories::token::Repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Value {
    #[validate(length(min = 1))]
    value: String,
}

impl_uuid_property!(Id);
impl_string_property!(Email);
impl_string_property!(Value);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    email: Email,
    #[getset(get = "pub", set = "pub")]
    role: Role,
    #[getset(get = "pub", set = "pub")]
    value: Value,
    #[getset(get = "pub")]
    created_by: AccountId,
}

impl Entity {
    pub fn new(
        id: impl Into<Option<String>>,
        email: String,
        role: Role,
        value: String,
        created_by: String,
    ) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            email: Email::new(email)?,
            role,
            value: Value::new(value)?,
            created_by: AccountId::try_from(created_by)?,
        })
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
    fn test_valid_email() {
        assert!(Email::new(testutils::rand::email()).is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new(testutils::rand::string(20)).is_err());
    }

    #[test]
    fn test_valid_value() {
        assert!(Value::new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_value() {
        assert!(Value::new("").is_err());
    }
}
