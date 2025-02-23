use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AssociationLabel, Error, PropertyMap, ResourceExt, Result, TableLocationResover};
use crate::{ObjectLabel, Resource, ResourceIdent, ResourceName, ResourceRef, ResourceStore};

const MAX_PAGE_SIZE: usize = 10000;

/// An in-memory implementation of a resource store.
///
/// This store is not intended for production use, but is useful for testing and development.
#[derive(Debug, Clone)]
pub struct InMemoryResourceStore {
    resources: Arc<DashMap<Uuid, Resource>>,
    id_map: Arc<DashMap<ObjectLabel, DashMap<ResourceName, Uuid>>>,
    associations: Arc<DashMap<AssociationLabel, DashMap<Uuid, (Uuid, Option<PropertyMap>)>>>,
}

impl Default for InMemoryResourceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryResourceStore {
    pub fn new() -> Self {
        Self {
            resources: DashMap::new().into(),
            id_map: DashMap::new().into(),
            associations: DashMap::new().into(),
        }
    }

    fn get_uuid(&self, label: &ObjectLabel, name: &ResourceName) -> Option<Uuid> {
        self.id_map
            .get(label)
            .and_then(|map| map.value().get(name).map(|uuid| *uuid.value()))
    }

    fn remove_uuid(&self, label: &ObjectLabel, name: &ResourceName) -> Option<Uuid> {
        self.id_map
            .get(label)
            .and_then(|map| map.value().remove(name).map(|(_, uuid)| uuid))
    }

    fn new_uuid(&self, label: &ObjectLabel, name: &ResourceName) -> Result<Uuid> {
        if self.get_uuid(label, name).is_some() {
            return Err(Error::AlreadyExists);
        }
        let map = self
            .id_map
            .entry(label.clone())
            .or_insert_with(DashMap::new);
        let uuid = Uuid::now_v7();
        map.insert(name.clone(), uuid);
        Ok(uuid)
    }
}

#[async_trait::async_trait]
impl TableLocationResover for InMemoryResourceStore {
    async fn resolve(&self, table: &ResourceRef) -> Result<url::Url> {
        let ident = ObjectLabel::TableInfo.to_ident(table.clone());
        let (_resource, _) = self.get(&ident).await?;
        todo!()
    }
}

#[async_trait::async_trait]
impl ResourceStore for InMemoryResourceStore {
    async fn get(&self, id: &ResourceIdent) -> Result<(Resource, ResourceRef)> {
        let resource = match id.as_ref() {
            ResourceRef::Uuid(uuid) => self.resources.get(uuid),
            ResourceRef::Name(name) => {
                let uuid = self.get_uuid(id.label(), name).ok_or(Error::NotFound)?;
                self.resources.get(&uuid)
            }
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        match resource {
            Some(resource) => Ok((resource.value().clone(), resource.key().into())),
            None => Err(Error::NotFound),
        }
    }

    async fn create(&self, resource: Resource) -> Result<(Resource, ResourceRef)> {
        if self
            .get_uuid(resource.resource_label(), &resource.resource_name())
            .is_some()
        {
            return Err(Error::AlreadyExists);
        }
        let uuid = self.new_uuid(resource.resource_label(), &resource.resource_name())?;
        self.resources.insert(uuid, resource.clone());
        Ok((resource, ResourceRef::Uuid(uuid)))
    }

    async fn delete(&self, id: &ResourceIdent) -> Result<()> {
        let uuid = match id.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(id.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        match self.resources.remove(&uuid) {
            Some((_, resource)) => self.remove_uuid(id.label(), &resource.resource_name()),
            None => None,
        };
        Ok(())
    }

    async fn update(
        &self,
        id: &ResourceIdent,
        resource: Resource,
    ) -> Result<(Resource, ResourceRef)> {
        let uuid = match id.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(id.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        // Need to clone to avoid locking the map while holding a reference to the value
        let existing = self
            .resources
            .get(&uuid)
            .ok_or(Error::NotFound)?
            .value()
            .clone();
        if existing.resource_label() != resource.resource_label() {
            self.id_map
                .get(existing.resource_label())
                .and_then(|map| map.value().remove(&existing.resource_name()));
            self.id_map
                .entry(resource.resource_label().clone())
                .or_insert_with(DashMap::new)
                .insert(resource.resource_name().clone(), uuid);
        } else if existing.resource_name() != resource.resource_name() {
            self.id_map
                .get(existing.resource_label())
                .and_then(|map| map.value().remove(&existing.resource_name()));
            self.id_map
                .get(existing.resource_label())
                .and_then(|map| map.value().insert(resource.resource_name(), uuid));
        }
        self.resources.insert(uuid, resource.clone());
        Ok((resource, ResourceRef::Uuid(uuid)))
    }

    async fn list(
        &self,
        label: &ObjectLabel,
        namespace: Option<&ResourceName>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<Resource>, Option<String>)> {
        let page_token = page_token.map(|t| Uuid::parse_str(&t)).transpose()?;
        let mut resource_ids = self
            .id_map
            .get(label)
            .map(|map| {
                map.value()
                    .iter()
                    .filter(|entry| {
                        namespace.is_none_or(|ns| entry.key().prefix_matches(ns))
                            && page_token.is_none_or(|t| &t > entry.value())
                    })
                    .map(|entry| *entry.value())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if resource_ids.is_empty() {
            return Ok((Vec::new(), None));
        }
        resource_ids.sort_unstable();

        let max_page_size = usize::min(max_results.unwrap_or(MAX_PAGE_SIZE), MAX_PAGE_SIZE);
        let mut resources = Vec::new();
        let mut last_id = &Uuid::nil();
        for uuid in resource_ids.iter().rev().take(max_page_size) {
            let resource = self.resources.get(uuid).ok_or(Error::NotFound)?.clone();
            last_id = uuid;
            resources.push(resource);
        }
        let next_page_token = (resources.len() == max_page_size).then(|| last_id.to_string());
        Ok((resources, next_page_token))
    }

    async fn add_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
        properties: Option<PropertyMap>,
    ) -> Result<()> {
        let from_uuid = match from.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(from.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        let to_uuid = match to.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(to.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        let map = self
            .associations
            .entry(label.clone())
            .or_insert_with(DashMap::new);
        map.insert(from_uuid, (to_uuid, properties.clone()));
        if let Some(inverse) = label.inverse() {
            let inverse_map = self
                .associations
                .entry(inverse)
                .or_insert_with(DashMap::new);
            inverse_map.insert(to_uuid, (from_uuid, properties.clone()));
        }
        Ok(())
    }

    async fn remove_association(
        &self,
        from: &ResourceIdent,
        to: &ResourceIdent,
        label: &AssociationLabel,
    ) -> Result<()> {
        let from_uuid = match from.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(from.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        let to_uuid = match to.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self.get_uuid(to.label(), name).ok_or(Error::NotFound)?,
            ResourceRef::Undefined => return Err(Error::NotFound),
        };
        let map = self.associations.get(label).ok_or(Error::NotFound)?;
        map.remove(&from_uuid);
        if let Some(inverse) = label.inverse() {
            let inverse_map = self.associations.get(&inverse).ok_or(Error::NotFound)?;
            inverse_map.remove(&to_uuid);
        }
        Ok(())
    }

    async fn list_associations(
        &self,
        resource: &ResourceIdent,
        label: &AssociationLabel,
        target_label: Option<&ResourceIdent>,
        max_results: Option<usize>,
        page_token: Option<String>,
    ) -> Result<(Vec<ResourceIdent>, Option<String>)> {
        let resource_uuid = match resource.as_ref() {
            ResourceRef::Uuid(uuid) => *uuid,
            ResourceRef::Name(name) => self
                .get_uuid(resource.label(), name)
                .ok_or(Error::NotFound)?,
            ResourceRef::Undefined => {
                return Err(Error::invalid_argument("resource must not be undefined"))
            }
        };
        let target_uuid = target_label
            .map(|tl| match tl.as_ref() {
                ResourceRef::Uuid(uuid) => Ok(*uuid),
                ResourceRef::Name(name) => self.get_uuid(tl.label(), name).ok_or(Error::NotFound),
                ResourceRef::Undefined => Err(Error::invalid_argument(
                    "target resource must not be undefined",
                )),
            })
            .transpose()?;
        let page_token = page_token.map(|t| Uuid::parse_str(&t)).transpose()?;
        let mut association_ids = self
            .associations
            .get(label)
            .map(|map| {
                map.value()
                    .get(&resource_uuid)
                    .iter()
                    .filter(|entry| {
                        target_uuid.is_none_or(|uuid| entry.value().0 == uuid)
                            && page_token.is_none_or(|t| &t > entry.key())
                    })
                    .map(|entry| entry.value().0)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if association_ids.is_empty() {
            return Ok((Vec::new(), None));
        }
        association_ids.sort_unstable();

        let max_page_size = usize::min(max_results.unwrap_or(MAX_PAGE_SIZE), MAX_PAGE_SIZE);
        let mut resources = Vec::new();
        let mut last_id = &Uuid::nil();
        for uuid in association_ids.iter().rev().take(max_page_size) {
            let resource = self.resources.get(uuid).ok_or(Error::NotFound)?;
            last_id = uuid;
            resources.push(resource.resource_ident());
        }
        let next_page_token = (resources.len() == max_page_size).then(|| last_id.to_string());
        Ok((resources, next_page_token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CatalogInfo, ObjectLabel};

    #[tokio::test]
    async fn test_create_get_delete() {
        let store = InMemoryResourceStore::new();
        let resource: Resource = CatalogInfo {
            name: "new_catalog".into(),
            ..Default::default()
        }
        .into();
        let (created, reference) = store.create(resource.clone()).await.unwrap();
        assert_eq!(created.resource_name(), resource.resource_name());

        let ident = ObjectLabel::CatalogInfo.to_ident(reference);
        let (retrieved, _) = store.get(&ident).await.unwrap();
        assert_eq!(retrieved, created);

        store.delete(&ident).await.unwrap();
        let result = store.get(&ident).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NotFound));
    }

    #[tokio::test]
    async fn test_list() {
        let store = InMemoryResourceStore::new();
        let resource: Resource = CatalogInfo {
            name: "new_catalog".into(),
            ..Default::default()
        }
        .into();
        let (created, _) = store.create(resource.clone()).await.unwrap();

        let (resources, next) = store
            .list(&ObjectLabel::CatalogInfo, None, None, None)
            .await
            .unwrap();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0], created);
        assert!(next.is_none());

        // add more resources
        let resource: Resource = CatalogInfo {
            name: "new_catalog2".into(),
            ..Default::default()
        }
        .into();
        store.create(resource).await.unwrap();
        let resource: Resource = CatalogInfo {
            name: "new_catalog3".into(),
            ..Default::default()
        }
        .into();
        store.create(resource).await.unwrap();

        let (resources, next) = store
            .list(&ObjectLabel::CatalogInfo, None, Some(2), None)
            .await
            .unwrap();
        assert_eq!(resources.len(), 2);
        assert!(next.is_some());

        let (resources, next) = store
            .list(&ObjectLabel::CatalogInfo, None, Some(2), next)
            .await
            .unwrap();
        assert_eq!(resources.len(), 1);
        assert!(next.is_none());
    }
}
