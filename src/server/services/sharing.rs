use crate::config::JWT_SECRET;
use crate::server::schemas::claims::Claims;
use crate::server::schemas::claims::Role;
use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub const VERSION: i64 = 1;

pub struct Service;

impl Service {
    pub fn expires_in(ttl: i64) -> Result<(i64, DateTime<Utc>)> {
        let ttl = u64::try_from(ttl).context("failed to convert i64 ttl to u64")?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("failed to create current system time")?;
        let expiration_secs = now + Duration::from_secs(ttl);
        let expiration_secs = expiration_secs.as_secs();
        let expiration_secs = i64::try_from(expiration_secs)
            .context("failed to convert u128 expiration seconds to i64")?;
        let expiration_time = NaiveDateTime::from_timestamp_opt(expiration_secs, 0)
            .context("faield to parse expiration seconds to datetime")?;
        let expiration_time = DateTime::<Utc>::from_utc(expiration_time, Utc);
        Ok((expiration_secs, expiration_time))
    }

    pub fn token(
        name: String,
        email: String,
        namespace: String,
        role: Role,
        expiry: i64,
    ) -> Result<String> {
        let claims = Claims {
            name: name,
            email: email,
            namespace: namespace,
            role: role,
            exp: expiry,
        };
        let token = encode(&Header::default(), &claims, &JWT_SECRET.encoding)
            .context("failed to create JWT token")?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("TEST SHARING!!!");
    }
}
