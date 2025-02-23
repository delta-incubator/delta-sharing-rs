use std::sync::Arc;

use itertools::Itertools;

use crate::{
    AssociationLabel, ObjectLabel, PropertyMap, Resource, ResourceIdent, ResourceName, ResourceRef,
    Result,
};

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
        label: &ObjectLabel,
        namespace: Option<&ResourceName>,
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

    /// Update a resource.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to update.
    /// - `resource`: The updated resource.
    ///
    /// # Returns
    /// The updated resource.
    async fn update(
        &self,
        id: &ResourceIdent,
        resource: Resource,
    ) -> Result<(Resource, ResourceRef)>;

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

pub trait ProvidesResourceStore: Send + Sync + 'static {
    fn store(&self) -> &dyn ResourceStore;
}

#[async_trait::async_trait]
impl<T: ResourceStore> ResourceStore for Arc<T> {
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)> {
        T::get(self, id).await
    }

    async fn get_many(&self, ids: &[ResourceIdent]) -> Result<Vec<(Resource, ResourceRef)>> {
        T::get_many(self, ids).await
    }

    async fn list(
        &self,
        label: &ObjectLabel,
        namespace: Option<&ResourceName>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Resource>, Option<String>)> {
        T::list(self, label, namespace, max_results, page_token).await
    }

    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)> {
        T::create(self, resource).await
    }

    async fn delete(&self, id: &ResourceIdent) -> Result<()> {
        T::delete(self, id).await
    }

    async fn update(
        &self,
        id: &ResourceIdent,
        resource: Resource,
    ) -> Result<(Resource, ResourceRef)> {
        T::update(self, id, resource).await
    }

    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()> {
        T::add_association(self, from, to, label, properties).await
    }

    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()> {
        T::remove_association(self, from, to, label).await
    }

    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)> {
        T::list_associations(self, resource, label, target_label, max_results, page_token).await
    }
}

#[async_trait::async_trait]
impl<T: ProvidesResourceStore> ResourceStore for T {
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)> {
        self.store().get(id).await
    }

    async fn get_many(&self, ids: &[ResourceIdent]) -> Result<Vec<(Resource, ResourceRef)>> {
        self.store().get_many(ids).await
    }

    async fn list(
        &self,
        label: &ObjectLabel,
        namespace: Option<&ResourceName>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Resource>, Option<String>)> {
        self.store()
            .list(label, namespace, max_results, page_token)
            .await
    }

    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)> {
        self.store().create(resource).await
    }

    async fn delete(&self, id: &ResourceIdent) -> Result<()> {
        self.store().delete(id).await
    }

    async fn update(
        &self,
        id: &ResourceIdent,
        resource: Resource,
    ) -> Result<(Resource, ResourceRef)> {
        self.store().update(id, resource).await
    }

    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()> {
        self.store()
            .add_association(from, to, label, properties)
            .await
    }

    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()> {
        self.store().remove_association(from, to, label).await
    }

    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)> {
        self.store()
            .list_associations(resource, label, target_label, max_results, page_token)
            .await
    }
}
