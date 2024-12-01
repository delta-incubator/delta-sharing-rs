use tonic::codegen::{Body, Bytes, StdError};
use tonic::transport::{Channel, Endpoint};

use crate::error::{Error, Result};
use crate::gen::v1::open_fga_service_client::OpenFgaServiceClient;
use crate::gen::v1::{AuthorizationModel, ListStoresRequest, ReadAuthorizationModelsRequest};

impl<T> OpenFgaServiceClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    /// Read the latest authorization model for the given store
    pub async fn read_latest_authoriztation_model(
        &mut self,
        store_id: impl Into<String>,
    ) -> Result<Option<AuthorizationModel>> {
        let request = ReadAuthorizationModelsRequest {
            store_id: store_id.into(),
            page_size: Some(1),
            continuation_token: "".into(),
        };
        // NOTE: request will return authorization models sorted in descending order of creation.
        // https://openfga.dev/api/service#/Authorization%20Models/ReadAuthorizationModels
        let mut response = self.read_authorization_models(request).await?.into_inner();
        Ok(response.authorization_models.pop())
    }
}

#[derive(Clone)]
pub struct ApiTokenConfig {
    /// API Token Value
    pub token: String,
    /// API Token Header Name (default = Authorization)
    pub header_name: Option<String>,
    /// API Token Value Prefix (default = Bearer)
    pub header_value_prefix: Option<String>,
}

#[derive(Clone)]
pub enum CredentialMethod {
    ApiToken(ApiTokenConfig),
    ClientCredentials,
}

#[derive(Clone)]
pub struct ClientConfig {
    pub credentials: Option<CredentialMethod>,
    pub store_id: Option<String>,
    pub store_name: Option<String>,
    pub authorization_model_id: Option<String>,
}

#[derive(Clone)]
pub struct OpenFgaClient {
    inner: OpenFgaServiceClient<Channel>,
    store_id: String,
    authorization_model_id: String,
}

impl OpenFgaClient {
    /// Create a new instance of [`OpenFgaClient`].
    pub fn new(
        client: OpenFgaServiceClient<Channel>,
        store_id: impl Into<String>,
        authorization_model_id: impl Into<String>,
    ) -> Self {
        Self {
            inner: client,
            store_id: store_id.into(),
            authorization_model_id: authorization_model_id.into(),
        }
    }

    pub async fn connect<D>(address: D, config: ClientConfig) -> Result<OpenFgaClient>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut client = OpenFgaServiceClient::connect(address).await?;

        let store_id = if let Some(id) = config.store_id {
            id
        } else if let Some(store_name) = config.store_name {
            let stores = client
                .list_stores(ListStoresRequest {
                    page_size: None,
                    continuation_token: "".into(),
                })
                .await?
                .into_inner();
            stores
                .stores
                .into_iter()
                .find(|s| s.name == store_name.as_ref())
                .ok_or_else(|| Error::store_not_found(store_name))?
                .id
        } else {
            return Err(Error::MissingStoreConfig);
        };

        let authorization_model_id = if let Some(id) = config.authorization_model_id {
            id
        } else {
            client
                .read_latest_authoriztation_model(&store_id)
                .await?
                .ok_or_else(|| Error::authorization_model_missing(&store_id))?
                .id
        };

        Ok(OpenFgaClient {
            inner: client,
            store_id,
            authorization_model_id,
        })
    }

    /// Read the latest authorization model for the current store
    pub async fn read_latest_authoriztation_model(&self) -> Result<Option<AuthorizationModel>> {
        self.inner
            .clone()
            .read_latest_authoriztation_model(&self.store_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::maybe_skip_fga;

    #[tokio::test]
    async fn test_read_latest_authoriztation_model() {
        maybe_skip_fga!();

        let store_id = std::env::var("FGA_STORE_ID").unwrap();

        let client = OpenFgaClient::connect(
            "http://[::1]:8081",
            ClientConfig {
                store_id: Some(store_id),
                store_name: None,
                authorization_model_id: None,
                credentials: None,
            },
        )
        .await
        .unwrap();

        let model = client.read_latest_authoriztation_model().await.unwrap();
        assert!(model.is_some())
    }
}
