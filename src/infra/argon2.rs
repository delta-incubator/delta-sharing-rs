use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::Argon2;

pub fn hash(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    if let Ok(hashed) = Argon2::default().hash_password(password, &salt) {
        Ok(hashed.to_string())
    } else {
        Err(anyhow!("falield to hash password"))
    }
}

pub fn verify(password: &[u8], hash: &str) -> Result<()> {
    let parsed = if let Ok(parsed) = PasswordHash::new(hash) {
        parsed
    } else {
        return Err(anyhow!("falield to parse hashed password"));
    };
    if let Ok(_) = Argon2::default().verify_password(password, &parsed) {
        Ok(())
    } else {
        Err(anyhow!("falield to verify password"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash() {
        let password = testutils::rand::string(20);
        let hashed = hash(password.as_bytes());
        assert!(matches!(hashed, Ok(_)));
    }

    #[tokio::test]
    async fn test_verify() {
        let password = testutils::rand::string(20);
        let hashed = hash(password.as_bytes()).expect("password should be hashed properly");
        let verified = verify(password.as_bytes(), &hashed);
        assert!(matches!(verified, Ok(_)));
    }
}
