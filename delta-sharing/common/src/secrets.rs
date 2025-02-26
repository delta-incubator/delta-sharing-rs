use bytes::Bytes;
use uuid::Uuid;

use crate::Result;

/// A trait for managing secrets.
///
/// All sensitive data that needs to be stored in the system should be stored as a secret.
///
/// The secret manager is responsible for fetching the secret value from the secret store.
/// The secret store can be a key-value store, a secret manager service, or any other secret store.
#[async_trait::async_trait]
pub trait SecretManager: Send + Sync {
    /// Get the secret value for the given secret name.
    ///
    /// Secrets are identified by a unique name. The secret manager should return the secret value
    /// and the version of the secret. The version is used to ensure that the secret value has not
    /// changed since the last time it was fetched.
    ///
    /// The secret manager should return an error if the secret does not exist
    async fn get_secret(&self, secret_name: &str) -> Result<(Uuid, Bytes)>;

    /// Get the secret value for the given secret name and version.
    /// This method is used to fetch a specific version of the secret.
    /// The secret manager should return an error if the secret does not exist
    async fn get_secret_version(&self, secret_name: &str, version: Uuid) -> Result<Bytes>;

    /// Create a new secret with the given name and value.
    /// The secret manager should return an error if the secret already exists
    /// or if the secret value is invalid.
    async fn create_secret(&self, secret_name: &str, secret_value: Bytes) -> Result<Uuid>;

    /// Update the secret value for the given secret name.
    /// The secret manager should return an error if the secret does not exist
    /// or if the secret value is invalid.
    async fn update_secret(&self, secret_name: &str, secret_value: Bytes) -> Result<Uuid>;
}
