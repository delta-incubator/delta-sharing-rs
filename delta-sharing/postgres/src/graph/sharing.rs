use delta_sharing_common::{ResourceRef, Result, TableLocationResover};

use super::GraphStore;

#[async_trait::async_trait]
impl TableLocationResover for GraphStore {
    async fn resolve(&self, _table_ref: &ResourceRef) -> Result<url::Url> {
        todo!();
    }
}
