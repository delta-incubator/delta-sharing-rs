// @generated
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
                    struct_ser.serialize_field("shareInfo", v)?;
                }
                resource::Resource::SharingSchemaInfo(v) => {
                    struct_ser.serialize_field("sharingSchemaInfo", v)?;
                }
                resource::Resource::SharingTable(v) => {
                    struct_ser.serialize_field("sharingTable", v)?;
                }
                resource::Resource::Credential(v) => {
                    struct_ser.serialize_field("credential", v)?;
                }
                resource::Resource::StorageLocation(v) => {
                    struct_ser.serialize_field("storageLocation", v)?;
                }
                resource::Resource::CatalogInfo(v) => {
                    struct_ser.serialize_field("catalogInfo", v)?;
                }
                resource::Resource::SchemaInfo(v) => {
                    struct_ser.serialize_field("schemaInfo", v)?;
                }
                resource::Resource::TableInfo(v) => {
                    struct_ser.serialize_field("tableInfo", v)?;
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
            "credential",
            "storage_location",
            "storageLocation",
            "catalog_info",
            "catalogInfo",
            "schema_info",
            "schemaInfo",
            "table_info",
            "tableInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareInfo,
            SharingSchemaInfo,
            SharingTable,
            Credential,
            StorageLocation,
            CatalogInfo,
            SchemaInfo,
            TableInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "credential" => Ok(GeneratedField::Credential),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "catalogInfo" | "catalog_info" => Ok(GeneratedField::CatalogInfo),
                            "schemaInfo" | "schema_info" => Ok(GeneratedField::SchemaInfo),
                            "tableInfo" | "table_info" => Ok(GeneratedField::TableInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                        GeneratedField::Credential => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credential"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::Credential)
;
                        }
                        GeneratedField::StorageLocation => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::StorageLocation)
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
