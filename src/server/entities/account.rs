use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId {
    value: Uuid,
}

impl_uuid_property!(AccountId);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct AccountName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(AccountName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct AccountEmail {
    #[validate(email)]
    value: String,
}

impl_string_property!(AccountEmail);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct AccountPassword {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(AccountPassword);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct AccountNamespace {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(AccountNamespace);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Account {
    #[getset(get = "pub")]
    id: AccountId,
    #[getset(get = "pub", set = "pub")]
    name: AccountName,
    #[getset(get = "pub", set = "pub")]
    email: AccountEmail,
    #[getset(get = "pub", set = "pub")]
    password: AccountPassword,
    #[getset(get = "pub", set = "pub")]
    namespace: AccountNamespace,
}

impl Account {
    pub fn new(
        id: String,
        name: String,
        email: String,
        password: String,
        namespace: String,
    ) -> Result<Self> {
        Ok(Self {
            id: AccountId::try_from(id)?,
            name: AccountName::new(name)?,
            email: AccountEmail::new(email)?,
            password: AccountPassword::new(password)?,
            namespace: AccountNamespace::new(namespace)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_account_id() {
        assert!(matches!(
            AccountId::try_from(testutils::rand::uuid()),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_account_id() {
        assert!(matches!(
            AccountId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_account_name() {
        assert!(matches!(
            AccountName::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_account_name() {
        assert!(matches!(AccountName::new(""), Err(_)));
    }

    #[test]
    fn test_valid_account_email() {
        assert!(matches!(AccountEmail::new(testutils::rand::email()), Ok(_)));
    }

    #[test]
    fn test_invalid_account_email() {
        assert!(matches!(
            AccountEmail::new(testutils::rand::string(20)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_account_password() {
        assert!(matches!(
            AccountPassword::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_account_password() {
        assert!(matches!(AccountPassword::new(""), Err(_)));
    }

    #[test]
    fn test_valid_account_namespace() {
        assert!(matches!(
            AccountNamespace::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_account_namespace() {
        assert!(matches!(AccountNamespace::new(""), Err(_)));
    }
}
