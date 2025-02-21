mod catalog;
mod credentials;

pub use catalog::*;
pub use credentials::*;

use crate::Recipient;

pub struct RequestContext {
    pub recipient: Recipient,
}

impl RequestContext {
    pub fn recipient(&self) -> &Recipient {
        &self.recipient
    }
}
