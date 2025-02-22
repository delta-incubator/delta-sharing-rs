use tonic::{Request, Response, Status};

use crate::api::{RequestContext, SharingDiscoveryHandler, SharingQueryHandler};
use crate::models::sharing::v1::{delta_sharing_service_server::DeltaSharingService, *};
use crate::{Error, Recipient, Result};

fn extract_context<T>(request: &Request<T>) -> Result<RequestContext> {
    let recipient = request
        .extensions()
        .get::<Recipient>()
        .cloned()
        .ok_or(Error::MissingRecipient)?;
    Ok(RequestContext { recipient })
}

#[async_trait::async_trait]
impl<T> DeltaSharingService for T
where
    T: SharingDiscoveryHandler + SharingQueryHandler,
{
    async fn list_shares(
        &self,
        request: Request<ListSharesRequest>,
    ) -> Result<Response<ListSharesResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::list_shares(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn get_share(
        &self,
        request: Request<GetShareRequest>,
    ) -> Result<Response<Share>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::get_share(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn list_share_tables(
        &self,
        request: Request<ListShareTablesRequest>,
    ) -> Result<Response<ListShareTablesResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::list_share_tables(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn list_sharing_schemas(
        &self,
        request: Request<ListSharingSchemasRequest>,
    ) -> Result<Response<ListSharingSchemasResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::list_sharing_schemas(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn list_schema_tables(
        &self,
        request: Request<ListSchemaTablesRequest>,
    ) -> Result<Response<ListSchemaTablesResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::list_schema_tables(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn get_table_version(
        &self,
        request: Request<GetTableVersionRequest>,
    ) -> Result<Response<GetTableVersionResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::get_table_version(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }

    async fn get_table_metadata(
        &self,
        request: Request<GetTableMetadataRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let ctx = extract_context(&request)?;
        let result = T::get_table_metadata(self, request.into_inner(), ctx).await?;
        Ok(Response::new(result))
    }
}
