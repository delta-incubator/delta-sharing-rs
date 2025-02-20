use itertools::Itertools;

use super::AssociationLabel;
use crate::models::sharing::v1::{
    Share, ShareInfo, SharingSchema, SharingSchemaInfo, SharingTable,
};
use crate::models::IntoProtoStruct;
use crate::models::PropertyMap;
use crate::models::Resource;
use crate::Error;
use crate::{ResourceIdent, ResourceRef, Result, SharingRepository};

/// Generic store that can be used to store and retrieve resources.
#[async_trait::async_trait]
pub trait ResourceStore: Send + Sync + 'static {
    /// Get a resource by its identifier.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to get.
    ///
    /// # Returns
    /// The resource with the given identifier.
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)>;

    async fn get_many(&self, ids: &[ResourceIdent]) -> Result<Vec<(Resource, ResourceRef)>> {
        let futures = ids.iter().map(|id| self.get(id)).collect_vec();
        Ok(futures_util::future::try_join_all(futures).await?)
    }

    /// List resources.
    ///
    /// List resources in the store that are children of the given resource.
    /// If the Reference inside the ResourceIdent is [Undefined](crate::ResourceRef::Undefined),
    /// the root of the store is used and resources of the specified type are listed.
    ///
    /// # Arguments
    /// - `root`: The root resource to list children of.
    /// - `max_results`: The maximum number of results to return.
    /// - `page_token`: The token to use to get the next page of results.
    async fn list(
        &self,
        root: &ResourceIdent,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Resource>, Option<String>)>;

    /// Create a new resource.
    ///
    /// # Arguments
    /// - `resource`: The resource to create.
    ///
    /// # Returns
    /// The created resource.
    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)>;

    /// Delete a resource and all connected associations by its identifier.
    ///
    /// The implementing store should delete all associations of the resource
    /// before deleting the resource itself.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to delete.
    async fn delete(&self, id: &ResourceIdent) -> Result<()>;

    /// Add an association between two resources.
    ///
    /// Assosications are directed edges between resources with a label and optional properties.
    /// Between two resources must be at most one association with a given label.
    /// Associations are bi-directional, meaning that if an association is added from A to B,
    /// there is also an association from B to A with the inverse label. Some labels are symmetric,
    /// meaning that the inverse label is the same as the label.
    ///
    /// # Arguments
    /// - `from`: The source resource of the association.
    /// - `to`: The target resource of the association.
    /// - `label`: The label of the association.
    /// - `properties`: Optional properties of the association.
    ///
    /// # Errors
    /// - [AlreadyExists](crate::Error::AlreadyExists) If the association already exists.
    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()>;

    /// Remove an association between two resources.
    ///
    /// Implementations must remove the inverse association as well.
    ///
    /// # Arguments
    /// - `from`: The source resource of the association.
    /// - `to`: The target resource of the association.
    /// - `label`: The label of the association.
    ///
    /// # Errors
    /// - [NotFound](crate::Error::NotFound) If the association does not exist.
    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()>;

    /// List associations of a resource.
    ///
    /// List associations of a resource with the given label.
    ///
    /// # Arguments
    /// - `resource`: The resource to list associations of.
    /// - `label`: The label of the associations to list.
    /// - `target_label`: The label of the target resource of the associations to list.
    /// - `max_results`: The maximum number of results to return.
    /// - `page_token`: The token to use to get the next page of results.
    ///
    /// # Returns
    /// The list of associations of the resource with the given label.
    /// The token to use to get the next page of results.
    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)>;
}

#[async_trait::async_trait]
impl<T: ResourceStore> SharingRepository for T {
    async fn add_share(
        &self,
        name: &str,
        comment: Option<String>,
        properties: Option<PropertyMap>,
    ) -> Result<ShareInfo> {
        let share_info = ShareInfo {
            id: "".to_string(),
            name: name.to_string(),
            properties: properties.map(|p| p.into_proto_struct()),
            description: comment,
        };
        self.create(share_info.into()).await?.0.try_into()
    }

    async fn get_share(&self, id: &ResourceRef) -> Result<ShareInfo> {
        self.get(&ResourceIdent::Share(id.clone()))
            .await?
            .0
            .try_into()
    }

    async fn delete_share(&self, id: &ResourceRef) -> Result<()> {
        self.delete(&ResourceIdent::Share(id.clone())).await
    }

    async fn list_shares(
        &self,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Share>, Option<String>)> {
        let (resources, token) = self
            .list(
                &ResourceIdent::Share(ResourceRef::Undefined),
                max_results,
                page_token,
            )
            .await?;
        let shares = resources.into_iter().map(|r| r.try_into()).try_collect()?;
        Ok((shares, token))
    }

    async fn add_schema(
        &self,
        share: &ResourceRef,
        name: &str,
        comment: Option<String>,
        properties: Option<PropertyMap>,
    ) -> Result<SharingSchemaInfo> {
        let ResourceRef::Name(share_name) = share else {
            return Err(Error::invalid_argument(
                "Only namespace / name references are allowed for create.",
            ));
        };
        if share_name.len() != 1 {
            return Err(Error::invalid_argument(
                "Only namespace / name references are allowed for create.",
            ));
        }
        let schema_info = SharingSchemaInfo {
            id: "".to_string(),
            share: share_name.to_string(),
            name: name.to_string(),
            properties: properties.map(|p| p.into_proto_struct()),
            description: comment,
            share_id: None,
        };
        let (schema, schema_ref) = self.create(schema_info.into()).await?;
        let from = ResourceIdent::Share(share.clone());
        let to = ResourceIdent::SharingSchema(schema_ref);
        self.add_association(&from, &to, &AssociationLabel::ParentOf, None)
            .await?;
        schema.try_into()
    }

    async fn get_schema(&self, id: &ResourceRef) -> Result<SharingSchemaInfo> {
        self.get(&ResourceIdent::SharingSchema(id.clone()))
            .await?
            .0
            .try_into()
    }

    async fn delete_schema(&self, id: &ResourceRef) -> Result<()> {
        self.delete(&ResourceIdent::SharingSchema(id.clone())).await
    }

    async fn list_schemas(
        &self,
        share: &ResourceRef,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<SharingSchema>, Option<String>)> {
        let ident = ResourceIdent::Share(share.clone());
        let (idents, token) = self
            .list_associations(
                &ident,
                &AssociationLabel::ParentOf,
                Some(&ResourceIdent::SharingSchema(ResourceRef::Undefined)),
                max_results,
                page_token,
            )
            .await?;
        let schemas = self
            .get_many(&idents)
            .await?
            .into_iter()
            .map(|(r, _)| r.try_into())
            .try_collect()?;
        Ok((schemas, token))
    }

    async fn list_schema_tables(
        &self,
        _schema: &ResourceRef,
        _max_results: Option<usize>,
        _page_token: Option<String>,
    ) -> Result<(Vec<SharingTable>, Option<String>)> {
        todo!();
    }

    async fn list_share_tables(
        &self,
        _share: &ResourceRef,
        _max_results: Option<usize>,
        _page_token: Option<String>,
    ) -> Result<(Vec<SharingTable>, Option<String>)> {
        todo!();
    }
}
