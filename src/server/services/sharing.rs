use crate::config;
use crate::config::JWT_SECRET;
use crate::utils::jwt::Claims;
use crate::utils::jwt::Role;
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub share_credentials_version: i64,
    pub endpoint: String,
    pub bearer_token: String,
    pub expiration_time: DateTime<Utc>,
}

pub struct Service;

impl Service {
    pub fn profile_v1(
        name: String,
        email: String,
        namespace: String,
        role: Role,
        ttl: i64,
    ) -> Result<Profile> {
        let ttl = u64::try_from(ttl).context("failed to convert i64 ttl to u64")?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("failed to create JWT token expiry")?;
        let expiry = now + Duration::from_secs(ttl);
        let expiry = expiry.as_millis();
        let expiry = i64::try_from(expiry).context("failed to convert u128 expiry to i64")?;
        let expiration_time = NaiveDateTime::from_timestamp_millis(expiry)
            .context("faield to parse expiry millis to datetime")?;
        let expiration_time = DateTime::<Utc>::from_utc(expiration_time, Utc);
        let claims = Claims {
            name: name,
            email: email,
            namespace: namespace,
            role: role,
            exp: expiry,
        };
        let token = encode(&Header::default(), &claims, &JWT_SECRET.encoding)
            .context("failed to create JWT token")?;
        Ok(Profile {
            share_credentials_version: 1,
            endpoint: config::fetch::<String>("server_bind"),
            bearer_token: token,
            expiration_time: expiration_time,
        })
    }
}
