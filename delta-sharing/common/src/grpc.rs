use tonic::{Request, Response, Status};

use crate::models::v1::{delta_sharing_service_server::DeltaSharingService, *};
use crate::{
    process_resources, DiscoveryHandler, Error, Permission, Policy, Recipient, Result,
    TableQueryHandler,
};

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
    T: DiscoveryHandler + TableQueryHandler + Policy,
{
    async fn list_shares(
        &self,
        request: Request<ListSharesRequest>,
    ) -> Result<Response<ListSharesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let mut result = T::list_shares(self, request.into_inner(), &recipient).await?;
        process_resources(self, &recipient, &Permission::Read, &mut result.items).await?;
        Ok(Response::new(result))
    }

    async fn get_share(
        &self,
        request: Request<GetShareRequest>,
    ) -> Result<Response<Share>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::get_share(self, req).await?;
        Ok(Response::new(result))
    }

    async fn list_share_tables(
        &self,
        request: Request<ListShareTablesRequest>,
    ) -> Result<Response<ListShareTablesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::list_share_tables(self, req).await?;
        Ok(Response::new(result))
    }

    async fn list_schemas(
        &self,
        request: Request<ListSchemasRequest>,
    ) -> Result<Response<ListSchemasResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::list_schemas(self, req).await?;
        Ok(Response::new(result))
    }

    async fn list_schema_tables(
        &self,
        request: Request<ListSchemaTablesRequest>,
    ) -> Result<Response<ListSchemaTablesResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::list_schema_tables(self, req).await?;
        Ok(Response::new(result))
    }

    async fn get_table_version(
        &self,
        request: Request<GetTableVersionRequest>,
    ) -> Result<Response<GetTableVersionResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::get_table_version(self, req).await?;
        Ok(Response::new(result))
    }

    async fn get_table_metadata(
        &self,
        request: Request<GetTableMetadataRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let recipient = extract_recipient(&request)?;
        let req = request.into_inner();
        self.check_required(&req, &recipient).await?;
        let result = T::get_table_metadata(self, req).await?;
        Ok(Response::new(result))
    }
}
