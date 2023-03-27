use crate::config;
use crate::config::JWT_SECRET;
use crate::server::middlewares::jwt::Claims;
use crate::server::middlewares::jwt::Role;
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
use utoipa::ToSchema;

pub const VERSION: i64 = 1;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub share_credentials_version: i64,
    pub endpoint: String,
    pub bearer_token: String,
    pub expiration_time: String,
}

pub struct Service;

fn new_token(
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

fn new_expiration(ttl: i64) -> Result<(i64, DateTime<Utc>)> {
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

impl Service {
    pub fn issue(
        name: String,
        email: String,
        namespace: String,
        role: Role,
        ttl: i64,
    ) -> Result<Profile> {
        let (expiration_secs, expiration_time) =
            self::new_expiration(ttl).context("expiration time calculation failed")?;
        let token = self::new_token(name, email, namespace, role, expiration_secs)
            .context("profile creation failed")?;
        Ok(Profile {
            share_credentials_version: VERSION,
            endpoint: config::fetch::<String>("server_bind"),
            bearer_token: token,
            expiration_time: expiration_time.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("TEST PROFILE!!!");
    }
}
