use crate::config::JWT_SECRET;
use crate::server::entities::account::Entity as AccountEntity;
use crate::server::entities::account::Name as AccountName;
use crate::server::error::Error;
use crate::server::routers::SharedState;
use crate::server::schemas::claims::Claims;
use crate::server::schemas::claims::Role;
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
    let name = AccountName::new(jwt.claims.name.clone()).map_err(|_| Error::ValidationFailed)?;
    let account = AccountEntity::load(&name, &state.pg_pool)
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
