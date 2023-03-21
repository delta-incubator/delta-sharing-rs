use crate::config::JWT_SECRET;
use crate::error::Error;
use anyhow::Context;
use anyhow::Result;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::TypedHeader;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Validation;
use std::convert::TryFrom;
use std::str::FromStr;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub role: Role,
    pub exp: u64,
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize, strum_macros::EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "admin")]
    Admin,
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "guest")]
    Guest,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn expires_in(ttl: i32) -> Result<u64> {
    let ttl = u64::try_from(ttl).context("failed to convert i32 ttl to u64")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to create JWT token expiry")?;
    let expiry = now + Duration::from_secs(ttl.into());
    Ok(expiry.as_secs())
}

fn required_role_of(path: &str) -> Role {
    let path = path.trim_start_matches("/");
    let path = path.split("/").next().unwrap_or("");
    if let Ok(role) = Role::from_str(path) {
        role
    } else {
        Role::Guest
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &B,
    ) -> std::result::Result<Self, Self::Rejection> {
        let maybe = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok();
        match maybe {
            Some(TypedHeader(Authorization(bearer))) => {
                let jwt =
                    decode::<Claims>(bearer.token(), &JWT_SECRET.decoding, &Validation::default())
                        .map_err(|_| Error::Unauthorized)?;
                let required_role = required_role_of(&parts.uri.path());
                if required_role == Role::Guest {
                    return Ok(jwt.claims);
                }
                if jwt.claims.role == Role::Guest {
                    return Err(Error::Unauthorized);
                }
                return Ok(jwt.claims);
            }
            _ => Err(Error::Unauthorized),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("TEST JWT!!!");
    }
}
