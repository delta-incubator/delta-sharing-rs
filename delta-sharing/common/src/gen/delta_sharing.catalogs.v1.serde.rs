// @generated
impl serde::Serialize for CatalogInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.storage_root.is_some() {
            len += 1;
        }
        if self.provider_name.is_some() {
            len += 1;
        }
        if self.share_name.is_some() {
            len += 1;
        }
        if self.catalog_type.is_some() {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        if self.created_by.is_some() {
            len += 1;
        }
        if self.update_at.is_some() {
            len += 1;
        }
        if self.updated_by.is_some() {
            len += 1;
        }
        if self.browse_only.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.CatalogInfo", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.storage_root.as_ref() {
            struct_ser.serialize_field("storageRoot", v)?;
        }
        if let Some(v) = self.provider_name.as_ref() {
            struct_ser.serialize_field("providerName", v)?;
        }
        if let Some(v) = self.share_name.as_ref() {
            struct_ser.serialize_field("shareName", v)?;
        }
        if let Some(v) = self.catalog_type.as_ref() {
            let v = CatalogType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("catalogType", &v)?;
        }
        if let Some(v) = self.create_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("createdBy", v)?;
        }
        if let Some(v) = self.update_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updateAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.updated_by.as_ref() {
            struct_ser.serialize_field("updatedBy", v)?;
        }
        if let Some(v) = self.browse_only.as_ref() {
            struct_ser.serialize_field("browseOnly", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CatalogInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "owner",
            "comment",
            "properties",
            "storage_root",
            "storageRoot",
            "provider_name",
            "providerName",
            "share_name",
            "shareName",
            "catalog_type",
            "catalogType",
            "create_at",
            "createAt",
            "created_by",
            "createdBy",
            "update_at",
            "updateAt",
            "updated_by",
            "updatedBy",
            "browse_only",
            "browseOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Owner,
            Comment,
            Properties,
            StorageRoot,
            ProviderName,
            ShareName,
            CatalogType,
            CreateAt,
            CreatedBy,
            UpdateAt,
            UpdatedBy,
            BrowseOnly,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "storageRoot" | "storage_root" => Ok(GeneratedField::StorageRoot),
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "shareName" | "share_name" => Ok(GeneratedField::ShareName),
                            "catalogType" | "catalog_type" => Ok(GeneratedField::CatalogType),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updateAt" | "update_at" => Ok(GeneratedField::UpdateAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            "browseOnly" | "browse_only" => Ok(GeneratedField::BrowseOnly),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CatalogInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.CatalogInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CatalogInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut storage_root__ = None;
                let mut provider_name__ = None;
                let mut share_name__ = None;
                let mut catalog_type__ = None;
                let mut create_at__ = None;
                let mut created_by__ = None;
                let mut update_at__ = None;
                let mut updated_by__ = None;
                let mut browse_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
                        }
                        GeneratedField::StorageRoot => {
                            if storage_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageRoot"));
                            }
                            storage_root__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = map_.next_value()?;
                        }
                        GeneratedField::ShareName => {
                            if share_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareName"));
                            }
                            share_name__ = map_.next_value()?;
                        }
                        GeneratedField::CatalogType => {
                            if catalog_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogType"));
                            }
                            catalog_type__ = map_.next_value::<::std::option::Option<CatalogType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::CreateAt => {
                            if create_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createAt"));
                            }
                            create_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdBy"));
                            }
                            created_by__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateAt => {
                            if update_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateAt"));
                            }
                            update_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UpdatedBy => {
                            if updated_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedBy"));
                            }
                            updated_by__ = map_.next_value()?;
                        }
                        GeneratedField::BrowseOnly => {
                            if browse_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("browseOnly"));
                            }
                            browse_only__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CatalogInfo {
                    id: id__,
                    name: name__.unwrap_or_default(),
                    owner: owner__,
                    comment: comment__,
                    properties: properties__,
                    storage_root: storage_root__,
                    provider_name: provider_name__,
                    share_name: share_name__,
                    catalog_type: catalog_type__,
                    create_at: create_at__,
                    created_by: created_by__,
                    update_at: update_at__,
                    updated_by: updated_by__,
                    browse_only: browse_only__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.CatalogInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CatalogType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CATALOG_TYPE_UNSPECIFIED",
            Self::ManagedCatalog => "MANAGED_CATALOG",
            Self::DeltasharingCatalog => "DELTASHARING_CATALOG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CatalogType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CATALOG_TYPE_UNSPECIFIED",
            "MANAGED_CATALOG",
            "DELTASHARING_CATALOG",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = CatalogType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CATALOG_TYPE_UNSPECIFIED" => Ok(CatalogType::Unspecified),
                    "MANAGED_CATALOG" => Ok(CatalogType::ManagedCatalog),
                    "DELTASHARING_CATALOG" => Ok(CatalogType::DeltasharingCatalog),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCatalogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.storage_root.is_some() {
            len += 1;
        }
        if self.provider_name.is_some() {
            len += 1;
        }
        if self.share_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.CreateCatalogRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.storage_root.as_ref() {
            struct_ser.serialize_field("storageRoot", v)?;
        }
        if let Some(v) = self.provider_name.as_ref() {
            struct_ser.serialize_field("providerName", v)?;
        }
        if let Some(v) = self.share_name.as_ref() {
            struct_ser.serialize_field("shareName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCatalogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "comment",
            "properties",
            "storage_root",
            "storageRoot",
            "provider_name",
            "providerName",
            "share_name",
            "shareName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Comment,
            Properties,
            StorageRoot,
            ProviderName,
            ShareName,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "storageRoot" | "storage_root" => Ok(GeneratedField::StorageRoot),
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "shareName" | "share_name" => Ok(GeneratedField::ShareName),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCatalogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.CreateCatalogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCatalogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut storage_root__ = None;
                let mut provider_name__ = None;
                let mut share_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
                        }
                        GeneratedField::StorageRoot => {
                            if storage_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageRoot"));
                            }
                            storage_root__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = map_.next_value()?;
                        }
                        GeneratedField::ShareName => {
                            if share_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareName"));
                            }
                            share_name__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateCatalogRequest {
                    name: name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                    storage_root: storage_root__,
                    provider_name: provider_name__,
                    share_name: share_name__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.CreateCatalogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteCatalogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.force.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.DeleteCatalogRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.force.as_ref() {
            struct_ser.serialize_field("force", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteCatalogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "force",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Force,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "force" => Ok(GeneratedField::Force),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteCatalogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.DeleteCatalogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteCatalogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut force__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteCatalogRequest {
                    name: name__.unwrap_or_default(),
                    force: force__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.DeleteCatalogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCatalogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.include_browse.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.GetCatalogRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.include_browse.as_ref() {
            struct_ser.serialize_field("includeBrowse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCatalogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "include_browse",
            "includeBrowse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            IncludeBrowse,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "includeBrowse" | "include_browse" => Ok(GeneratedField::IncludeBrowse),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCatalogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.GetCatalogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCatalogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut include_browse__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeBrowse => {
                            if include_browse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeBrowse"));
                            }
                            include_browse__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCatalogRequest {
                    name: name__.unwrap_or_default(),
                    include_browse: include_browse__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.GetCatalogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCatalogsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.ListCatalogsRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCatalogsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListCatalogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.ListCatalogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCatalogsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListCatalogsRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.ListCatalogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCatalogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.catalogs.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.ListCatalogsResponse", len)?;
        if !self.catalogs.is_empty() {
            struct_ser.serialize_field("catalogs", &self.catalogs)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCatalogsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "catalogs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Catalogs,
            NextPageToken,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "catalogs" => Ok(GeneratedField::Catalogs),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListCatalogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.ListCatalogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCatalogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut catalogs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Catalogs => {
                            if catalogs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogs"));
                            }
                            catalogs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListCatalogsResponse {
                    catalogs: catalogs__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.ListCatalogsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCatalogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if !self.new_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.catalogs.v1.UpdateCatalogRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if !self.new_name.is_empty() {
            struct_ser.serialize_field("newName", &self.new_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCatalogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "owner",
            "comment",
            "properties",
            "new_name",
            "newName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Owner,
            Comment,
            Properties,
            NewName,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCatalogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.catalogs.v1.UpdateCatalogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCatalogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut new_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
                        }
                        GeneratedField::NewName => {
                            if new_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newName"));
                            }
                            new_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateCatalogRequest {
                    name: name__.unwrap_or_default(),
                    owner: owner__,
                    comment: comment__,
                    properties: properties__,
                    new_name: new_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.catalogs.v1.UpdateCatalogRequest", FIELDS, GeneratedVisitor)
    }
}
