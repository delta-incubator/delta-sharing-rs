use anyhow::anyhow;
use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use getset::Getters;
use getset::Setters;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::impl_i64_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use crate::server::repositories::account::Repository;

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
pub struct Email {
    #[validate(email)]
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 1))]
    value: String,
}

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

impl_uuid_property!(Id);
impl_string_property!(Name);
impl_string_property!(Email);
impl_string_property!(Password);
impl_string_property!(Namespace);
impl_i64_property!(Ttl);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: Id,
    #[getset(get = "pub", set = "pub")]
    name: Name,
    #[getset(get = "pub", set = "pub")]
    email: Email,
    #[getset(get = "pub", set = "pub")]
    password: Password,
    #[getset(get = "pub", set = "pub")]
    namespace: Namespace,
    #[getset(get = "pub", set = "pub")]
    ttl: Ttl,
}

fn hash(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(password, &salt)
        .map_err(|_| anyhow!("falield to hash password"))?;
    Ok(hashed.to_string())
}

fn verify(password: &[u8], hash: &str) -> Result<()> {
    let parsed =
        PasswordHash::new(hash).map_err(|_| anyhow!("falield to parse hashed password"))?;
    Argon2::default()
        .verify_password(password, &parsed)
        .map_err(|_| anyhow!("falield to verify password"))
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
            password: Password::new(self::hash(password.as_bytes()).unwrap())?,
            namespace: Namespace::new(namespace)?,
            ttl: Ttl::new(ttl)?,
        })
    }

    pub async fn load(name: &Name, pg_pool: &PgPool) -> Result<Option<Self>> {
        match Repository::select_by_name(name, pg_pool).await? {
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

    pub async fn save(&self, pg_pool: &PgPool) -> Result<PgQueryResult> {
        Repository::upsert(self, pg_pool).await
    }

    pub fn verify(&self, password: &[u8]) -> Result<()> {
        self::verify(password, self.password().as_str())
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

    #[test]
    fn test_valid_email() {
        assert!(Email::new(testutils::rand::email()).is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new(testutils::rand::string(20)).is_err());
    }

    #[test]
    fn test_valid_password() {
        assert!(Password::new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_password() {
        assert!(Password::new("").is_err());
    }

    #[test]
    fn test_valid_namespace() {
        assert!(Namespace::new(testutils::rand::string(255)).is_ok());
    }

    #[test]
    fn test_invalid_namespace() {
        assert!(Namespace::new("").is_err());
    }

    #[test]
    fn test_valid_ttl() {
        assert!(Ttl::new(testutils::rand::i64(0, 100000)).is_ok());
    }

    #[test]
    fn test_invalid_ttl() {
        assert!(Ttl::new(testutils::rand::i64(-100000, -1)).is_err());
    }
}
