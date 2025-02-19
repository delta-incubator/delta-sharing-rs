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
                resource::Resource::SchemaInfo(v) => {
                    struct_ser.serialize_field("schemaInfo", v)?;
                }
                resource::Resource::Credential(v) => {
                    struct_ser.serialize_field("credential", v)?;
                }
                resource::Resource::StorageLocation(v) => {
                    struct_ser.serialize_field("storageLocation", v)?;
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
            "schema_info",
            "schemaInfo",
            "credential",
            "storage_location",
            "storageLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareInfo,
            SchemaInfo,
            Credential,
            StorageLocation,
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
                            "schemaInfo" | "schema_info" => Ok(GeneratedField::SchemaInfo),
                            "credential" => Ok(GeneratedField::Credential),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
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
                        GeneratedField::SchemaInfo => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaInfo"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::SchemaInfo)
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
