mod catalog;
mod credentials;
mod sharing;

pub use catalog::*;
pub use credentials::*;
pub use sharing::*;

use crate::Recipient;

pub struct RequestContext {
    pub recipient: Recipient,
}

impl RequestContext {
    pub fn recipient(&self) -> &Recipient {
        &self.recipient
    }
}

impl AsRef<Recipient> for RequestContext {
    fn as_ref(&self) -> &Recipient {
        &self.recipient
    }
}
