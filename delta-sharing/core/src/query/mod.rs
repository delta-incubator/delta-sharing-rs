use std::sync::Arc;

use delta_kernel::{Engine, Table};

use crate::error::Result;
use crate::types as t;
pub use kernel::*;

pub mod kernel;

#[async_trait::async_trait]
pub trait TableQueryHandler: Send + Sync {
    async fn get_table_version(
        &self,
        request: t::GetTableVersionRequest,
    ) -> Result<t::GetTableVersionResponse>;
}

#[async_trait::async_trait]
pub trait KernelEngineFactroy: Send + Sync {
    async fn create(&self, table: &Table) -> Result<Arc<dyn Engine>>;
}
