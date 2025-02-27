pub(crate) mod catalogs;
pub(crate) mod credentials;
pub(crate) mod external_locations;
pub(crate) mod recipients;
pub(crate) mod schemas;
pub(crate) mod shares;
pub(crate) mod sharing;
pub(crate) mod tables;

pub use catalogs::CatalogHandler;
pub use credentials::CredentialsHandler;
pub use external_locations::ExternalLocationsHandler;
pub use recipients::RecipientsHandler;
pub use schemas::SchemasHandler;
pub use shares::SharesHandler;
pub use sharing::{SharingDiscoveryHandler, SharingQueryHandler};
pub use tables::TablesHandler;

use crate::{Permission, Recipient, ResourceIdent};

#[derive(Debug, Clone)]
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

pub trait SecuredAction: Send + Sync {
    /// The resource that the action is performed on.
    fn resource(&self) -> ResourceIdent;

    /// The permission required to perform the action.
    fn permission(&self) -> &'static Permission;
}
