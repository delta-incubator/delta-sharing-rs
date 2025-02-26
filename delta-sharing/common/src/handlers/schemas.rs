use itertools::Itertools;

use crate::api::schemas::SchemasHandler;
use crate::models::schemas::v1::*;
use crate::policy::{process_resources, Permission, Policy};
use crate::{ObjectLabel, RequestContext, ResourceName, ResourceStore, Result, SecuredAction};

#[async_trait::async_trait]
impl<T: ResourceStore + Policy> SchemasHandler for T {
    async fn create_schema(
        &self,
        request: CreateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let resource = SchemaInfo {
            name: request.name,
            catalog_name: request.catalog_name,
            comment: request.comment,
            properties: request.properties,
            ..Default::default()
        };
        // TODO:
        // - update the schema with the current actor as owner
        // - create updated_* relations
        self.create(resource.into()).await?.0.try_into()
    }

    async fn delete_schema(
        &self,
        request: DeleteSchemaRequest,
        context: RequestContext,
    ) -> Result<()> {
        self.check_required(&request, context.as_ref()).await?;
        self.delete(&request.resource()).await
    }

    async fn list_schemas(
        &self,
        request: ListSchemasRequest,
        context: RequestContext,
    ) -> Result<ListSchemasResponse> {
        self.check_required(&request, context.as_ref()).await?;
        let (mut resources, next_page_token) = self
            .list(
                &ObjectLabel::SchemaInfo,
                Some(&ResourceName::new([&request.catalog_name])),
                request.max_results.map(|v| v as usize),
                request.page_token,
            )
            .await?;
        process_resources(self, context.as_ref(), &Permission::Read, &mut resources).await?;
        Ok(ListSchemasResponse {
            schemas: resources.into_iter().map(|r| r.try_into()).try_collect()?,
            next_page_token,
        })
    }

    async fn get_schema(
        &self,
        request: GetSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.check_required(&request, context.as_ref()).await?;
        self.get(&request.resource()).await?.0.try_into()
    }

    async fn update_schema(
        &self,
        request: UpdateSchemaRequest,
        context: RequestContext,
    ) -> Result<SchemaInfo> {
        self.check_required(&request, context.as_ref()).await?;
        let ident = request.resource();
        let name = ResourceName::from_naive_str_split(request.full_name);
        let [catalog_name, _schema_name] = name.as_ref() else {
            return Err(crate::Error::invalid_argument(
                "Invalid schema name - expected <catalog_name>.<schema_name>",
            ));
        };
        let resource = SchemaInfo {
            name: request.new_name.clone(),
            comment: request.comment,
            properties: request.properties,
            catalog_name: catalog_name.to_owned(),
            full_name: Some(format!("{}.{}", catalog_name, request.new_name)),
            ..Default::default()
        };
        // TODO:
        // - add update_* relations
        // - update owner if necessary
        self.update(&ident, resource.into()).await?.0.try_into()
    }
}
