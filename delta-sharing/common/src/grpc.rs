use tonic::{Request, Response, Status};

use crate::models::v1::{delta_sharing_service_server::DeltaSharingService, *};
use crate::{DiscoveryManager, Error, Recipient, Result, TableQueryManager};

fn extract_recipient<T>(request: &Request<T>) -> Result<Recipient> {
    request
        .extensions()
        .get::<Recipient>()
        .cloned()
        .ok_or(Error::MissingRecipient)
}

#[async_trait::async_trait]
impl<T> DeltaSharingService for T
where
    T: DiscoveryManager + TableQueryManager,
{
    async fn list_shares(
        &self,
        request: Request<ListSharesRequest>,
    ) -> Result<Response<ListSharesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::list_shares(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn get_share(
        &self,
        request: Request<GetShareRequest>,
    ) -> Result<Response<Share>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::get_share(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn list_share_tables(
        &self,
        request: Request<ListShareTablesRequest>,
    ) -> Result<Response<ListShareTablesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::list_share_tables(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn list_schemas(
        &self,
        request: Request<ListSchemasRequest>,
    ) -> Result<Response<ListSchemasResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::list_schemas(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn list_schema_tables(
        &self,
        request: Request<ListSchemaTablesRequest>,
    ) -> Result<Response<ListSchemaTablesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::list_schema_tables(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn get_table_version(
        &self,
        request: Request<GetTableVersionRequest>,
    ) -> Result<Response<GetTableVersionResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::get_table_version(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }

    async fn get_table_metadata(
        &self,
        request: Request<GetTableMetadataRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let result = T::get_table_metadata(self, request.into_inner(), &recipient).await?;
        Ok(Response::new(result))
    }
}
