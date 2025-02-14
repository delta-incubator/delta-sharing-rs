use chrono::{DateTime, Utc};
use delta_sharing_common::{Profile, Result};
use jsonwebtoken::Validation;
use serde::{de::DeserializeOwned, Serialize};

pub use in_memory::*;
pub use tokens::*;

mod in_memory;
mod tokens;

/// Claims that are encoded in a profile.
pub trait ProfileClaims: Serialize + DeserializeOwned + Send + Sync {
    /// Get the profile fingerprint from the claims.
    fn fingerprint(&self) -> String;

    fn validation() -> Validation {
        Validation::default()
    }
}

#[async_trait::async_trait]
pub trait ProfileManager: Send + Sync {
    /// Claims that are encoded in the profile.
    type Claims: ProfileClaims;

    /// Issue a profile for a set of claims that can be shared with a recipient.
    async fn issue_profile(
        &self,
        claims: &Self::Claims,
        expiration_time: Option<DateTime<Utc>>,
    ) -> Result<Profile>;

    /// Revoke a profile by its fingerprint.
    ///
    /// This should invalidate the profile and prevent it from being used.
    async fn revoke_profile(&self, fingerprint: &str) -> Result<()>;

    /// Validate a profile token and return the claims.
    /// This should return an error if the profile is invalid or has been revoked.
    async fn validate_profile(&self, token: &str) -> Result<Self::Claims>;
}
