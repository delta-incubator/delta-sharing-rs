use crate::impl_string_property;
use crate::impl_uuid_property;
use crate::server::repositories::account::AccountRepository;
use crate::server::repositories::account::PgAccountRepository;
use crate::utils::argon2;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
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
    #[serde(skip_serializing)]
    #[getset(get = "pub", set = "pub")]
    password: AccountPassword,
    #[getset(get = "pub", set = "pub")]
    namespace: AccountNamespace,
}

impl Account {
    pub fn new(
        id: impl Into<Option<String>>,
        name: String,
        email: String,
        password: String,
        namespace: String,
    ) -> Result<Self> {
        Ok(Self {
            id: AccountId::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: AccountName::new(name)?,
            email: AccountEmail::new(email)?,
            password: AccountPassword::new(argon2::hash(password.as_bytes()).unwrap())?,
            namespace: AccountNamespace::new(namespace)?,
        })
    }

    pub async fn list(
        limit: impl Into<Option<&i64>> + Send,
        offset: impl Into<Option<&i64>> + Send,
        pg_pool: &PgPool,
    ) -> Result<Vec<Self>> {
        let repo = PgAccountRepository;
        let rows = repo.select(limit.into(), offset.into(), pg_pool).await?;
        rows.into_iter()
            .map(|row| {
                Self::new(
                    row.id.to_string(),
                    row.name,
                    row.email,
                    row.password,
                    row.namespace,
                )
            })
            .collect()
    }

    pub async fn find_by_id(id: &AccountId, pg_pool: &PgPool) -> Result<Option<Self>> {
        let repo = PgAccountRepository;
        match repo.select_by_id(&id, pg_pool).await? {
            Some(row) => Ok(Self {
                id: AccountId::new(row.id),
                name: AccountName::new(row.name)?,
                email: AccountEmail::new(row.email)?,
                password: AccountPassword::new(row.password)?,
                namespace: AccountNamespace::new(row.namespace)?,
            }
            .into()),
            _ => Ok(None),
        }
    }

    pub async fn find_by_name(name: &AccountName, pg_pool: &PgPool) -> Result<Option<Self>> {
        let repo = PgAccountRepository;
        match repo.select_by_name(&name, pg_pool).await? {
            Some(row) => Ok(Self {
                id: AccountId::new(row.id),
                name: AccountName::new(row.name)?,
                email: AccountEmail::new(row.email)?,
                password: AccountPassword::new(row.password)?,
                namespace: AccountNamespace::new(row.namespace)?,
            }
            .into()),
            _ => Ok(None),
        }
    }

    pub async fn register(&self, pg_pool: &PgPool) -> Result<PgQueryResult> {
        let repo = PgAccountRepository;
        repo.upsert(&self, pg_pool).await
    }

    pub fn verify(&self, password: &[u8]) -> Result<()> {
        argon2::verify(password.into(), self.password().as_str())
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
