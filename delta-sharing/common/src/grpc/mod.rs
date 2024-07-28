use tonic::{Request, Response, Status};

use crate::error::{Error, Result};
use crate::types::{delta_sharing_service_server::DeltaSharingService, *};
use crate::{DeltaSharingHandler, Recipient};

#[async_trait::async_trait]
impl DeltaSharingService for DeltaSharingHandler {
    async fn list_shares(
        &self,
        request: Request<ListSharesRequest>,
    ) -> Result<Response<ListSharesResponse>, Status> {
        let recipient = request
            .extensions()
            .get::<Recipient>()
            .ok_or(Error::MissingRecipient)?
            .clone();
        let result = self
            .discovery
            .list_shares(request.into_inner(), &recipient)
            .await?;
        Ok(Response::new(result))
    }

    async fn get_share(
        &self,
        request: Request<GetShareRequest>,
    ) -> Result<Response<GetShareResponse>, Status> {
        let result = self.discovery.get_share(request.into_inner()).await?;
        Ok(Response::new(result))
    }

    async fn list_share_tables(
        &self,
        request: Request<ListShareTablesRequest>,
    ) -> Result<Response<ListShareTablesResponse>, Status> {
        let result = self
            .discovery
            .list_share_tables(request.into_inner())
            .await?;
        Ok(Response::new(result))
    }

    async fn list_schemas(
        &self,
        request: Request<ListSchemasRequest>,
    ) -> Result<Response<ListSchemasResponse>, Status> {
        let result = self.discovery.list_schemas(request.into_inner()).await?;
        Ok(Response::new(result))
    }

    async fn list_schema_tables(
        &self,
        request: Request<ListSchemaTablesRequest>,
    ) -> Result<Response<ListSchemaTablesResponse>, Status> {
        let result = self
            .discovery
            .list_schema_tables(request.into_inner())
            .await?;
        Ok(Response::new(result))
    }
}
