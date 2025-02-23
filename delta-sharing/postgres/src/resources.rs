use delta_sharing_common::models::PropertyMap;
use delta_sharing_common::{
    AssociationLabel, Error, IntoJson, Object, ObjectLabel, Resource, ResourceIdent, ResourceName,
    ResourceRef, ResourceStore, Result, EMPTY_RESOURCE_NAME,
};
use itertools::Itertools;

use crate::GraphStore;

pub trait IdentRefs {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef);
}

impl IdentRefs for ResourceIdent {
    fn ident(&self) -> (&ObjectLabel, &ResourceRef) {
        (self.as_ref(), self.as_ref())
    }
}

#[async_trait::async_trait]
impl ResourceStore for GraphStore {
    /// Get a resource by its identifier.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to get.
    ///
    /// # Returns
    /// The resource with the given identifier.
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)> {
        match id.as_ref() {
            ResourceRef::Uuid(uuid) => Ok((self.get_object(uuid).await?.try_into()?, id.into())),
            ResourceRef::Name(name) => {
                let object = self.get_object_by_name(id.as_ref(), name).await?;
                let id_new = ResourceRef::Uuid(object.id);
                Ok((object.try_into()?, id_new))
            }
            ResourceRef::Undefined => Err(Error::generic("Cannot get undefined resource")),
        }
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
    ) -> Result<(Vec<Resource>, Option<String>)> {
        let namespace = namespace.unwrap_or_else(|| &EMPTY_RESOURCE_NAME);
        let objects = self
            .list_objects(label, namespace, page_token.as_deref(), max_results)
            .await?;
        Ok((
            objects
                .0
                .into_iter()
                .map(|object| object.try_into())
                .try_collect()?,
            objects.1,
        ))
    }

    /// Create a new resource.
    ///
    /// # Arguments
    /// - `resource`: The resource to create.
    ///
    /// # Returns
    /// The created resource.
    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)> {
        let object: Object = resource.try_into()?;
        let object = self
            .add_object(&object.label, &object.name, object.properties)
            .await?;
        let id = ResourceRef::Uuid(object.id);
        Ok((object.try_into()?, id))
    }

    /// Delete a resource by its identifier.
    ///
    /// # Arguments
    /// - `id`: The identifier of the resource to delete.
    async fn delete(&self, id: &ResourceIdent) -> Result<()> {
        match id.as_ref() {
            ResourceRef::Uuid(uuid) => self.delete_object(uuid).await?,
            ResourceRef::Name(name) => {
                let obj = self.get_object_by_name(id.as_ref(), name).await?;
                self.delete_object(&obj.id).await?;
            }
            ResourceRef::Undefined => {
                return Err(Error::generic("Cannot delete undefined resource"))
            }
        };
        Ok(())
    }

    /// Update a resource.
    ///
    /// # Arguments
    /// - `resource`: The resource to update.
    ///
    /// # Returns
    /// The updated resource.
    async fn update(
        &self,
        id: &ResourceIdent,
        resource: Resource,
    ) -> Result<(Resource, ResourceRef)> {
        todo!("update")
    }

    /// Add an association between two resources.
    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()> {
        let from_id = match from.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_object_by_name(from.as_ref(), name).await?.id,
            ResourceRef::Undefined => {
                return Err(Error::generic(
                    "Cannot add association to undefined resource",
                ))
            }
        };
        let to_id = match to.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_object_by_name(to.as_ref(), name).await?.id,
            ResourceRef::Undefined => {
                return Err(Error::generic(
                    "Cannot add association to undefined resource",
                ))
            }
        };
        self.add_association(&from_id, label, &to_id, properties.map(|p| p.into_json()))
            .await?;
        Ok(())
    }

    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()> {
        todo!("remove_association")
    }

    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)> {
        let target_label = target_label.map(|r| r.ident().0);
        let target_id = self.ident_to_uuid(resource).await?;
        let (associations, token) = self
            .list_associations(
                &target_id,
                label,
                target_label,
                page_token.as_deref(),
                max_results,
            )
            .await?;
        let idents = associations
            .into_iter()
            .map(|assoc| assoc.to_label.to_ident(assoc.to_id))
            .collect();
        Ok((idents, token))
    }
}
