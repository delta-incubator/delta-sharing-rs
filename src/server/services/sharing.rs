use crate::config::JWT_SECRET;
use crate::utils::jwt::Claims;
use crate::utils::jwt::Role;
use anyhow::Context;
use anyhow::Result;
use jsonwebtoken::encode;
use jsonwebtoken::Header;

pub const VERSION: i64 = 1;

pub struct Service;

impl Service {
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
