use std::sync::Arc;

use delta_sharing_common::{DiscoveryHandler, Policy, TableQueryHandler};

#[derive(Clone)]
pub struct DeltaSharingHandler {
    pub discovery: Arc<dyn DiscoveryHandler>,
    pub query: Arc<dyn TableQueryHandler>,
    pub policy: Arc<dyn Policy>,
}
