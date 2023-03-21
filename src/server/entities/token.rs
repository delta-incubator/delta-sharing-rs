use crate::impl_i32_property;
use crate::impl_uuid_property;
use crate::server::entities::account::Id as AccountId;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
    value: Uuid,
}

impl_uuid_property!(Id);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Expiry {
    #[validate(range(min = 0))]
    value: i32,
}

impl_i32_property!(Expiry);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    expiry: Expiry,
    #[getset(get = "pub")]
    created_by: AccountId,
}

impl Entity {
    pub fn new(id: impl Into<Option<String>>, expiry: i32, created_by: String) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            expiry: Expiry::new(expiry)?,
            created_by: AccountId::try_from(created_by)?,
        })
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
    fn test_valid_expiry() {
        assert!(matches!(
            Expiry::new(testutils::rand::i32(0, 100000)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_expiry() {
        assert!(matches!(
            Expiry::new(testutils::rand::i32(-100000, -1)),
            Err(_)
        ));
    }
}
