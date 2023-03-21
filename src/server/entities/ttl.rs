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
pub struct Seconds {
    #[validate(range(min = 0))]
    value: i32,
}

impl_i32_property!(Seconds);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    seconds: Seconds,
    #[getset(get = "pub")]
    account_id: AccountId,
}

impl Entity {
    pub fn new(id: impl Into<Option<String>>, seconds: i32, account_id: String) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            seconds: Seconds::new(seconds)?,
            account_id: AccountId::try_from(account_id)?,
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
    fn test_valid_seconds() {
        assert!(matches!(
            Seconds::new(testutils::rand::i32(0, 100000)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_seconds() {
        assert!(matches!(
            Seconds::new(testutils::rand::i32(-100000, -1)),
            Err(_)
        ));
    }
}
