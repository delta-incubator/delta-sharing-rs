//! Token management module.
//!
//! This module provides a simple token management system that can be used to
//! internally manage tokens for recipient profiles. More spoeifically, we need
//! to solve two problems:
//!
//! 1. Authorize a user to access a specific resource.
//! 2. Distribute a token to a user to access a specific resource.
//!
//! The first problem is solved via the [profiles] defined in the delta sharing protocol.
//! There are several solutions out there that can be used to solve the second problem.
//! For now we implement a simple mechanism to create pre-signed urls that can be
//! shared with a recipient to load a profile. This approach is simple and can be
//! used as well for server administrators to locally create a signed url to bootstrap
//! access to the server on initial setup. I.e. fetch an admin profile.
//! This requires some shared secret accessible to admins and the running server.
//!
//! [profiles]: https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md#profile-file-format
use std::collections::HashMap;

use chrono::{DateTime, SecondsFormat, Utc};
use http::{HeaderMap, Method};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::error::{Error, Result};
use crate::{Permission, Resource};

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

pub(crate) fn hmac_sha256(secret: impl AsRef<[u8]>, bytes: impl AsRef<[u8]>) -> ring::hmac::Tag {
    let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_ref());
    ring::hmac::sign(&key, bytes.as_ref())
}

pub fn string_to_sign(
    url: &Url,
    permission: &Permission,
    resource: &Resource,
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> (String, HashMap<&'static str, String>) {
    let signed_start = start.to_rfc3339_opts(SecondsFormat::Secs, true);
    let signed_expiry = end.to_rfc3339_opts(SecondsFormat::Secs, true);
    let signed_permissions = permission.as_ref().to_string();
    let signed_resource: String = resource.into();
    let signed_host = url.host_str().unwrap_or_default();

    let string_to_sign = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        signed_host, signed_permissions, signed_start, signed_expiry, signed_resource,
    );

    let mut pairs = HashMap::new();
    pairs.insert("sp", signed_permissions);
    pairs.insert("st", signed_start);
    pairs.insert("se", signed_expiry);
    pairs.insert("sr", signed_resource);

    (string_to_sign, pairs)
}
