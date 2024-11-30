use tonic::transport::Channel;

use crate::error::{Error, Result};
use crate::gen::v1::open_fga_service_client::OpenFgaServiceClient;
use crate::gen::v1::{ConsistencyPreference, ListObjectsRequest, ListStoresRequest};

pub struct Client {
    inner: OpenFgaServiceClient<Channel>,
    store_id: String,
    authorization_model_id: String,
}

impl Client {
    pub async fn connect(address: String, store_name: impl AsRef<str>) -> Result<Client> {
        let mut client = OpenFgaServiceClient::connect(address).await?;
        let stores = client
            .list_stores(ListStoresRequest {
                page_size: None,
                continuation_token: "".into(),
            })
            .await?
            .into_inner();

        let store_id = stores
            .stores
            .into_iter()
            .find(|s| s.name == store_name.as_ref())
            .ok_or_else(|| Error::store_not_found(store_name.as_ref()))?
            .id;

        Ok(Client {
            inner: client,
            store_id,
            authorization_model_id: "".into(),
        })
    }

    pub async fn list_schemas(&mut self, user_id: impl AsRef<str>) -> Result<Vec<String>> {
        let user = format!("user:{}", user_id.as_ref());
        let request = ListObjectsRequest {
            store_id: self.store_id.clone(),
            authorization_model_id: self.authorization_model_id.clone(),
            user,
            relation: "can_read".into(),
            r#type: "share".into(),
            contextual_tuples: None,
            context: None,
            consistency: ConsistencyPreference::MinimizeLatency.into(),
        };
        let res = self.inner.list_objects(request).await?.into_inner();
        Ok(res.objects)
    }
}
