use ring::digest;
use serde::{Deserialize, Serialize};

use super::ProfileClaims;

/// Default recipient for delta sharing.
#[derive(Debug, Clone, PartialEq)]
pub enum DefaultRecipient<C: ProfileClaims> {
    Anonymous,
    Profile(C),
}

impl<C: ProfileClaims> DefaultRecipient<C> {
    /// Get the profile id from the recipient.
    pub fn profile_id(&self) -> Option<String> {
        match self {
            DefaultRecipient::Anonymous => None,
            DefaultRecipient::Profile(profile) => Some(profile.fingerprint()),
        }
    }
}

/// Default claims for delta sharing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefaultClaims {
    /// Email associated with the profile.
    pub email: String,

    /// Timestamp the profile was created.
    pub issued_at: i64,

    /// List of shares this profile has access to.
    pub shares: Vec<String>,

    /// Expiration time for the profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,

    /// Server admin flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
}

impl ProfileClaims for DefaultClaims {
    fn fingerprint(&self) -> String {
        let seed = format!(
            "{}-{}-{}",
            self.email,
            self.issued_at,
            self.shares.join(",")
        );
        let digest = digest::digest(&digest::SHA256, seed.as_bytes());
        hex::encode(digest.as_ref())
    }
}
