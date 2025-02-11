//! Authentication middleware
//!
//! Depending on the use case, there are vastly different requirements on how to
//! implement authentication and authorization for the Delta Sharing server.
//!
//! Example use cases include:
//! - Public access: No authentication is required, all data is public.
//! - Share based access: Recipients can access data based on a shared secret.
//! - OAuth: Recipients can access data based on an OAuth token.
//!
//! In order to support these different use cases, the Delta Sharing server is
//! designed to be extensible. The authentication middleware is implemented as a
//! [tower::Layer](https://docs.rs/tower/0.4.4/tower/trait.Layer.html) that
//! wraps the application service. The authentication middleware is responsible
//! for authenticating the client and setting the recipient identifier in the
//! request extensions. the recipient identifier is used by the Delta Sharing
//! server to determine the access control policy for the request.

use std::fmt::Display;

pub mod public;

/// Recipient identifier.
///
/// The recipient identifier is used to identify the client that is making the
/// request. The recipient identifier is used by the Delta Sharing server to
/// determine the access control policy for the request.
#[derive(Debug, Clone, PartialEq)]
pub enum RecipientId {
    /// Anonymous recipient identifier.
    Unknown,
    /// Known recipient identifier.
    Known(String),
}

impl RecipientId {
    /// Create a new [`RecipientId`] for an anonymous recipient.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::unknown();
    /// assert_eq!(recipient_id, RecipientId::Unknown);
    /// ```
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// Create a new [`RecipientId`] for an anonymous recipient.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::anonymous();
    /// assert_eq!(recipient_id, RecipientId::Unknown);
    /// ```
    pub fn anonymous() -> Self {
        Self::Unknown
    }

    /// Create a new [`RecipientId`] for an authenticated recipient.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::known("foo");
    /// assert_eq!(recipient_id, RecipientId::Known("foo".to_owned()));
    /// ```
    pub fn known<S: Into<String>>(recipient_id: S) -> Self {
        Self::Known(recipient_id.into())
    }

    /// Check if the recipient identifier is unknown.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::anonymous();
    /// assert!(recipient_id.is_unknown());
    /// ```
    pub fn is_unknown(&self) -> bool {
        matches!(self, RecipientId::Unknown)
    }

    /// Check if the recipient identifier is anonymous.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::anonymous();
    /// assert!(recipient_id.is_anonymous());
    /// ```
    pub fn is_anonymous(&self) -> bool {
        matches!(self, RecipientId::Unknown)
    }

    /// Check if the recipient identifier is known.
    ///
    /// # Example
    /// ```
    /// use delta_sharing::auth::RecipientId;
    ///
    /// let recipient_id = RecipientId::known("foo");
    /// assert!(recipient_id.is_known());
    /// ```
    pub fn is_known(&self) -> bool {
        matches!(self, RecipientId::Known(_))
    }
}

impl Default for RecipientId {
    fn default() -> Self {
        Self::Unknown
    }
}

impl AsRef<str> for RecipientId {
    fn as_ref(&self) -> &str {
        match self {
            RecipientId::Unknown => "ANONYMOUS",
            RecipientId::Known(id) => id.as_str(),
        }
    }
}

impl Display for RecipientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_recipient_id() {
        assert_eq!(RecipientId::default(), RecipientId::Unknown);
    }

    #[test]
    fn display_recipient_id() {
        assert_eq!(
            RecipientId::anonymous().to_string(),
            String::from("ANONYMOUS")
        );
        assert_eq!(
            RecipientId::known("my_recipient_id").to_string(),
            String::from("my_recipient_id")
        );
    }
}
