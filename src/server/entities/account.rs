use crate::impl_i64_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use crate::server::repositories::account::PgRepository;
use crate::server::repositories::account::Repository;
use crate::utils::argon2;
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

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl_string_property!(Email);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(Password);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Namespace {
    #[validate(length(min = 1))]
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Ttl {
    #[validate(range(min = 0))]
    value: i64,
}

impl_i64_property!(Ttl);

impl_string_property!(Namespace);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    name: Name,
    #[getset(get = "pub", set = "pub")]
    email: Email,
    #[serde(skip_serializing)]
    #[getset(get = "pub", set = "pub")]
    password: Password,
    #[getset(get = "pub", set = "pub")]
    namespace: Namespace,
    #[getset(get = "pub", set = "pub")]
    ttl: Ttl,
}

impl Entity {
    pub fn new(
        id: impl Into<Option<String>>,
        name: String,
        email: String,
        password: String,
        namespace: String,
        ttl: i64,
    ) -> Result<Self> {
        Ok(Self {
            id: Id::try_from(id.into().unwrap_or(uuid::Uuid::new_v4().to_string()))?,
            name: Name::new(name)?,
            email: Email::new(email)?,
            password: Password::new(argon2::hash(password.as_bytes()).unwrap())?,
            namespace: Namespace::new(namespace)?,
            ttl: Ttl::new(ttl)?,
        })
    }

    pub async fn list(
        limit: impl Into<Option<&i64>> + Send,
        offset: impl Into<Option<&i64>> + Send,
        pg_pool: &PgPool,
    ) -> Result<Vec<Self>> {
        let repo = PgRepository;
        let rows = repo.select(limit.into(), offset.into(), pg_pool).await?;
        rows.into_iter()
            .map(|row| {
                Self::new(
                    row.id.to_string(),
                    row.name,
                    row.email,
                    row.password,
                    row.namespace,
                    row.ttl,
                )
            })
            .collect()
    }

    pub async fn find_by_id(id: &Id, pg_pool: &PgPool) -> Result<Option<Self>> {
        let repo = PgRepository;
        match repo.select_by_id(&id, pg_pool).await? {
            Some(row) => Ok(Self {
                id: Id::new(row.id),
                name: Name::new(row.name)?,
                email: Email::new(row.email)?,
                password: Password::new(row.password)?,
                namespace: Namespace::new(row.namespace)?,
                ttl: Ttl::new(row.ttl)?,
            }
            .into()),
            _ => Ok(None),
        }
    }

    pub async fn find_by_name(name: &Name, pg_pool: &PgPool) -> Result<Option<Self>> {
        let repo = PgRepository;
        match repo.select_by_name(&name, pg_pool).await? {
            Some(row) => Ok(Self {
                id: Id::new(row.id),
                name: Name::new(row.name)?,
                email: Email::new(row.email)?,
                password: Password::new(row.password)?,
                namespace: Namespace::new(row.namespace)?,
                ttl: Ttl::new(row.ttl)?,
            }
            .into()),
            _ => Ok(None),
        }
    }

    pub async fn register(&self, pg_pool: &PgPool) -> Result<PgQueryResult> {
        let repo = PgRepository;
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

    #[test]
    fn test_valid_email() {
        assert!(matches!(Email::new(testutils::rand::email()), Ok(_)));
    }

    #[test]
    fn test_invalid_email() {
        assert!(matches!(Email::new(testutils::rand::string(20)), Err(_)));
    }

    #[test]
    fn test_valid_password() {
        assert!(matches!(Password::new(testutils::rand::string(255)), Ok(_)));
    }

    #[test]
    fn test_invalid_password() {
        assert!(matches!(Password::new(""), Err(_)));
    }

    #[test]
    fn test_valid_namespace() {
        assert!(matches!(
            Namespace::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_namespace() {
        assert!(matches!(Namespace::new(""), Err(_)));
    }

    #[test]
    fn test_valid_ttl() {
        assert!(matches!(Ttl::new(testutils::rand::i64(0, 100000)), Ok(_)));
    }

    #[test]
    fn test_invalid_ttl() {
        assert!(matches!(
            Ttl::new(testutils::rand::i64(-100000, -1)),
            Err(_)
        ));
    }
}
