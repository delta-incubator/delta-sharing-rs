use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareId {
    value: Uuid,
}

impl_uuid_property!(ShareId);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ShareName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(ShareName);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareCreatedBy {
    value: Uuid,
}

impl_uuid_property!(ShareCreatedBy);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Share {
    #[getset(get = "pub")]
    id: ShareId,
    #[getset(get = "pub", set = "pub")]
    name: ShareName,
    #[getset(get = "pub")]
    created_by: ShareCreatedBy,
}

impl Share {
    pub fn new(id: impl Into<Option<String>>, name: String, created_by: String) -> Result<Self> {
        Ok(Self {
            id: ShareId::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: ShareName::new(name)?,
            created_by: ShareCreatedBy::try_from(created_by)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_share_id() {
        assert!(matches!(ShareId::try_from(testutils::rand::uuid()), Ok(_)));
    }

    #[test]
    fn test_invalid_share_id() {
        assert!(matches!(
            ShareId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_share_name() {
        assert!(matches!(
            ShareName::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_share_name() {
        assert!(matches!(ShareName::new(""), Err(_)));
    }

    #[test]
    fn test_valid_share_created_by() {
        assert!(matches!(
            ShareCreatedBy::try_from(testutils::rand::uuid()),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_share_created_by() {
        assert!(matches!(
            ShareCreatedBy::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }
}
