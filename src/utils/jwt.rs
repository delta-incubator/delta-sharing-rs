use crate::config::JWT_SECRET;
use crate::error::Error;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::interactors::SharedState;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::headers::HeaderMapExt;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use axum::TypedHeader;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Validation;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub role: Role,
    pub exp: i64,
}

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, strum_macros::EnumString,
)]
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
                return Ok(jwt.claims);
            }
            _ => Err(Error::Unauthorized),
        }
    }
}

pub async fn as_admin<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> std::result::Result<Response, Error> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(Error::BadRequest)?
        .token()
        .to_owned();
    let jwt = decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default())
        .map_err(|_| Error::Unauthorized)?;
    let state = request
        .extensions()
        .get::<SharedState>()
        .ok_or(anyhow!("failed to acquire shared state"))?;
    let name = AccountName::new(jwt.claims.name).map_err(|_| Error::ValidationFailed)?;
    let account = AccountEntity::find_by_name(&name, &state.pg_pool)
        .await
        .map_err(|_| anyhow!("error occured while selecting account from database"))?;
    let Some(account) = account else {
	return Err(Error::Unauthorized);
    };
    if jwt.claims.role != Role::Admin {
        return Err(Error::Unauthorized);
    }
    request.extensions_mut().insert(account);
    Ok(next.run(request).await)
}

pub async fn as_guest<T>(
    request: Request<T>,
    next: Next<T>,
) -> std::result::Result<Response, Error> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(Error::BadRequest)?
        .token()
        .to_owned();
    decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default())
        .map_err(|_| Error::Unauthorized)?;
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("TEST JWT!!!");
    }
}
