use delta_sharing_common::{ResourceRef, Result, TableLocationResolver};

use super::GraphStore;

#[async_trait::async_trait]
impl TableLocationResolver for GraphStore {
    async fn resolve(&self, _table_ref: &ResourceRef) -> Result<url::Url> {
        todo!();
    }
}
