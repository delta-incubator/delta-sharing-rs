// @generated
impl serde::Serialize for ObjectRelations {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.owner.is_some() {
            len += 1;
        }
        if self.created_by.is_some() {
            len += 1;
        }
        if self.updated_by.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.internal.ObjectRelations", len)?;
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("created_by", v)?;
        }
        if let Some(v) = self.updated_by.as_ref() {
            struct_ser.serialize_field("updated_by", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectRelations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "created_by",
            "createdBy",
            "updated_by",
            "updatedBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            CreatedBy,
            UpdatedBy,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectRelations;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.internal.ObjectRelations")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ObjectRelations, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut created_by__ = None;
                let mut updated_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdBy"));
                            }
                            created_by__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedBy => {
                            if updated_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedBy"));
                            }
                            updated_by__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ObjectRelations {
                    owner: owner__,
                    created_by: created_by__,
                    updated_by: updated_by__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.internal.ObjectRelations", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.internal.Resource", len)?;
        if let Some(v) = self.resource.as_ref() {
            match v {
                resource::Resource::ShareInfo(v) => {
                    struct_ser.serialize_field("share_info", v)?;
                }
                resource::Resource::SharingSchemaInfo(v) => {
                    struct_ser.serialize_field("sharing_schema_info", v)?;
                }
                resource::Resource::SharingTable(v) => {
                    struct_ser.serialize_field("sharing_table", v)?;
                }
                resource::Resource::CredentialInfo(v) => {
                    struct_ser.serialize_field("credential_info", v)?;
                }
                resource::Resource::CatalogInfo(v) => {
                    struct_ser.serialize_field("catalog_info", v)?;
                }
                resource::Resource::SchemaInfo(v) => {
                    struct_ser.serialize_field("schema_info", v)?;
                }
                resource::Resource::TableInfo(v) => {
                    struct_ser.serialize_field("table_info", v)?;
                }
                resource::Resource::ColumnInfo(v) => {
                    struct_ser.serialize_field("column_info", v)?;
                }
                resource::Resource::ExternalLocationInfo(v) => {
                    struct_ser.serialize_field("external_location_info", v)?;
                }
                resource::Resource::RecipientInfo(v) => {
                    struct_ser.serialize_field("recipient_info", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share_info",
            "shareInfo",
            "sharing_schema_info",
            "sharingSchemaInfo",
            "sharing_table",
            "sharingTable",
            "credential_info",
            "credentialInfo",
            "catalog_info",
            "catalogInfo",
            "schema_info",
            "schemaInfo",
            "table_info",
            "tableInfo",
            "column_info",
            "columnInfo",
            "external_location_info",
            "externalLocationInfo",
            "recipient_info",
            "recipientInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareInfo,
            SharingSchemaInfo,
            SharingTable,
            CredentialInfo,
            CatalogInfo,
            SchemaInfo,
            TableInfo,
            ColumnInfo,
            ExternalLocationInfo,
            RecipientInfo,
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
                            "shareInfo" | "share_info" => Ok(GeneratedField::ShareInfo),
                            "sharingSchemaInfo" | "sharing_schema_info" => Ok(GeneratedField::SharingSchemaInfo),
                            "sharingTable" | "sharing_table" => Ok(GeneratedField::SharingTable),
                            "credentialInfo" | "credential_info" => Ok(GeneratedField::CredentialInfo),
                            "catalogInfo" | "catalog_info" => Ok(GeneratedField::CatalogInfo),
                            "schemaInfo" | "schema_info" => Ok(GeneratedField::SchemaInfo),
                            "tableInfo" | "table_info" => Ok(GeneratedField::TableInfo),
                            "columnInfo" | "column_info" => Ok(GeneratedField::ColumnInfo),
                            "externalLocationInfo" | "external_location_info" => Ok(GeneratedField::ExternalLocationInfo),
                            "recipientInfo" | "recipient_info" => Ok(GeneratedField::RecipientInfo),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.internal.Resource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShareInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::ShareInfo)
;
                        }
                        GeneratedField::SharingSchemaInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharingSchemaInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::SharingSchemaInfo)
;
                        }
                        GeneratedField::SharingTable => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharingTable"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::SharingTable)
;
                        }
                        GeneratedField::CredentialInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentialInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::CredentialInfo)
;
                        }
                        GeneratedField::CatalogInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::CatalogInfo)
;
                        }
                        GeneratedField::SchemaInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::SchemaInfo)
;
                        }
                        GeneratedField::TableInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::TableInfo)
;
                        }
                        GeneratedField::ColumnInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::ColumnInfo)
;
                        }
                        GeneratedField::ExternalLocationInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalLocationInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::ExternalLocationInfo)
;
                        }
                        GeneratedField::RecipientInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipientInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::RecipientInfo)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Resource {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.internal.Resource", FIELDS, GeneratedVisitor)
    }
}
