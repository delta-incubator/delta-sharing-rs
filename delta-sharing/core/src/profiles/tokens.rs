//! Token management module.
//!
//! This module provides a simple token management system that can be used to
//! internally manage tokens recipient profiles.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

/// A pair of encoding and decoding keys.
#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Create a new instnace of [`Keys`].
    pub fn new(encoding: EncodingKey, decoding: DecodingKey) -> Self {
        Self { encoding, decoding }
    }

    /// Create a new instance of [`Keys`] from a secret.
    pub fn new_from_secret(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl From<&[u8]> for Keys {
    fn from(secret: &[u8]) -> Self {
        Self::new_from_secret(secret)
    }
}

/// Token manager.
///
/// This struct provides simple token management capabilties that can be used to
/// build a profile management system for delta-sharing servers.
pub struct TokenManager {
    keys: Keys,
}

impl TokenManager {
    /// Create a new instance of [`TokenManager`].
    pub fn new(keys: Keys) -> Self {
        Self { keys }
    }

    /// Create a new instance of [`TokenManager`] from a secret.
    pub fn new_from_secret(secret: &[u8]) -> Self {
        Self {
            keys: Keys::new_from_secret(secret),
        }
    }

    /// Encode a set of claims into a token.
    pub fn encode<C: Serialize>(&self, claims: &C) -> Result<String> {
        encode(&Header::default(), claims, &self.keys.encoding).map_err(Error::from)
    }

    /// Decode a token into a set of claims.
    pub fn decode<C: DeserializeOwned>(&self, token: impl AsRef<str>) -> Result<C> {
        decode::<C>(token.as_ref(), &self.keys.decoding, &Validation::default())
            .map(|data| data.claims)
            .map_err(Error::from)
    }
}
