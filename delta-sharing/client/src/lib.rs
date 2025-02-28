pub(crate) mod error;
pub(crate) mod service;
// pub(crate) mod sharing;

use cloud_client::CloudClient;
use delta_sharing_common::{
    models::catalogs::v1::{CreateCatalogRequest, DeleteCatalogRequest},
    CatalogInfo,
};

pub use self::error::*;

#[derive(Clone)]
pub struct CatalogClient {
    client: CloudClient,
    base_url: url::Url,
}

impl CatalogClient {
    pub async fn create_catalog(&self, req: &CreateCatalogRequest) -> Result<()> {
        let url = self.base_url.join("catalogs")?;
        let result = self.client.post(url).json(req).send().await.unwrap();
        Ok(())
    }
}
