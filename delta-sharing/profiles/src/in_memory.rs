use bytes::Bytes;
use chrono::{DateTime, SecondsFormat, Utc};
use dashmap::DashSet;
use delta_sharing_common::{Error, Recipient, Result};
use ring::digest;
use serde::{Deserialize, Serialize};

pub use crate::tokens::*;
use crate::{Profile, ProfileClaims, ProfileManager};

pub type DeltaRecipient = DefaultRecipient<DefaultClaims>;
pub type DeltaProfileManager = InMemoryProfileManager<DefaultClaims>;

/// Default recipient for delta sharing.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DefaultRecipient<C: ProfileClaims> {
    Anonymous,
    Profile(C),
}

impl<C: ProfileClaims> From<DefaultRecipient<C>> for Recipient {
    fn from(recipient: DefaultRecipient<C>) -> Self {
        Recipient::custom(Bytes::from(serde_json::to_vec(&recipient).unwrap()))
    }
}

impl<C: ProfileClaims> DefaultRecipient<C> {
    /// Get the profile fingerprint for the recipient.
    pub fn fingerprint(&self) -> Option<String> {
        match self {
            DefaultRecipient::Anonymous => None,
            DefaultRecipient::Profile(profile) => Some(profile.fingerprint()),
        }
    }
}

/// Default claims for delta sharing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefaultClaims {
    /// The subject of the profile.
    ///
    /// This is typically the email address of the user, but may be any identiifier for the recipient.
    pub sub: String,

    /// Expiration time for the profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,

    /// Timestamp the profile was created.
    pub issued_at: i64,

    /// List of shares this profile has access to.
    pub shares: Vec<String>,

    /// Server admin flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
}

impl ProfileClaims for DefaultClaims {
    fn fingerprint(&self) -> String {
        let seed = format!("{}-{}-{}", self.sub, self.issued_at, self.shares.join(","));
        let digest = digest::digest(&digest::SHA256, seed.as_bytes());
        hex::encode(digest.as_ref())
    }
}

pub struct InMemoryProfileManager<T: ProfileClaims + Send> {
    token_manager: TokenManager,
    server_endpoint: String,
    share_credentials_version: i32,
    revoked_profiles: DashSet<String>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ProfileClaims + Send> InMemoryProfileManager<T> {
    pub fn new(
        server_endpoint: String,
        share_credentials_version: i32,
        token_manager: TokenManager,
    ) -> Self {
        Self {
            server_endpoint,
            share_credentials_version,
            token_manager,
            revoked_profiles: DashSet::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: ProfileClaims + Send> ProfileManager for InMemoryProfileManager<T> {
    type Claims = T;

    async fn issue_profile(
        &self,
        claims: &Self::Claims,
        expiration_time: Option<DateTime<Utc>>,
    ) -> Result<Profile> {
        let token = self.token_manager.encode(claims)?;
        let profile = Profile {
            share_credentials_version: self.share_credentials_version,
            endpoint: self.server_endpoint.clone(),
            bearer_token: token,
            expiration_time: expiration_time
                .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true)),
        };
        Ok(profile)
    }

    async fn revoke_profile(&self, fingerprint: &str) -> Result<()> {
        self.revoked_profiles.insert(fingerprint.to_string());
        Ok(())
    }

    async fn validate_profile(&self, token: &str) -> Result<Self::Claims> {
        let claims = self.token_manager.decode::<Self::Claims>(token)?;
        if self.revoked_profiles.contains(&claims.fingerprint()) {
            return Err(Error::Generic(
                "Profile has previously been revoked".to_string(),
            ));
        }
        Ok(claims)
    }
}
