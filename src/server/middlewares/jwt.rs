use crate::config::JWT_SECRET;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::routers::SharedState;
use crate::server::services::error::Error;
use anyhow::anyhow;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::headers::HeaderMapExt;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Validation;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub role: Role,
    pub exp: i64,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum_macros::EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "admin")]
    Admin,
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "guest")]
    Guest,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

#[tracing::instrument(skip(next))]
pub async fn as_admin<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> std::result::Result<Response, Error>
where
    T: std::fmt::Debug,
{
    let Some(auth) = request.headers().typed_get::<Authorization<Bearer>>() else {
        tracing::error!("bearer token is missing");
	return Err(Error::BadRequest);
    };
    let token = auth.token().to_owned();
    let Ok(jwt) = decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default()) else {
        tracing::error!("bearer token is incorrect");
        return Err(Error::Unauthorized);
    };
    let Some(state) = request.extensions().get::<SharedState>() else {
        tracing::error!("request is not handled correctly due to a server error");
        return Err(anyhow!("failed to acquire shared state").into());
    };
    let Ok(name) = AccountName::new(jwt.claims.name.clone()) else {
        tracing::error!("request is malformed");
	return Err(Error::ValidationFailed);
    };
    let Ok(account) = AccountEntity::load(&name, &state.pg_pool).await else {
        tracing::error!("request is not handled correctly due to a server error");
        return Err(anyhow!("error occured while selecting account from database").into());
    };
    let Some(account) = account else {
        tracing::error!("bearer token is incorrect");
	return Err(Error::Unauthorized);
    };
    if jwt.claims.role != Role::Admin {
        tracing::error!("request is forbidden from being fulfilled");
        return Err(Error::Forbidden);
    }
    request.extensions_mut().insert(account);
    Ok(next.run(request).await)
}

#[tracing::instrument(skip(next))]
pub async fn as_guest<T>(request: Request<T>, next: Next<T>) -> std::result::Result<Response, Error>
where
    T: std::fmt::Debug,
{
    let Some(auth) = request.headers().typed_get::<Authorization<Bearer>>() else {
        tracing::error!("bearer token is missing");
	return Err(Error::BadRequest);
    };
    let token = auth.token().to_owned();
    let Ok(_) = decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default()) else {
        tracing::error!("bearer token is incorrect");
        return Err(Error::Unauthorized)?;
    };
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
