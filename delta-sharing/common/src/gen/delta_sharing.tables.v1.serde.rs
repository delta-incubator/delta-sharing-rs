// @generated
impl serde::Serialize for DataSourceFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DATA_SOURCE_FORMAT_UNSPECIFIED",
            Self::Delta => "DELTA",
            Self::Iceberg => "ICEBERG",
            Self::Hudi => "HUDI",
            Self::Parquet => "PARQUET",
            Self::Csv => "CSV",
            Self::Json => "JSON",
            Self::Orc => "ORC",
            Self::Avro => "AVRO",
            Self::Text => "TEXT",
            Self::UnityCatalog => "UNITY_CATALOG",
            Self::Deltasharing => "DELTASHARING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DataSourceFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DATA_SOURCE_FORMAT_UNSPECIFIED",
            "DELTA",
            "ICEBERG",
            "HUDI",
            "PARQUET",
            "CSV",
            "JSON",
            "ORC",
            "AVRO",
            "TEXT",
            "UNITY_CATALOG",
            "DELTASHARING",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = DataSourceFormat;

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
                    "DATA_SOURCE_FORMAT_UNSPECIFIED" => Ok(DataSourceFormat::Unspecified),
                    "DELTA" => Ok(DataSourceFormat::Delta),
                    "ICEBERG" => Ok(DataSourceFormat::Iceberg),
                    "HUDI" => Ok(DataSourceFormat::Hudi),
                    "PARQUET" => Ok(DataSourceFormat::Parquet),
                    "CSV" => Ok(DataSourceFormat::Csv),
                    "JSON" => Ok(DataSourceFormat::Json),
                    "ORC" => Ok(DataSourceFormat::Orc),
                    "AVRO" => Ok(DataSourceFormat::Avro),
                    "TEXT" => Ok(DataSourceFormat::Text),
                    "UNITY_CATALOG" => Ok(DataSourceFormat::UnityCatalog),
                    "DELTASHARING" => Ok(DataSourceFormat::Deltasharing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.GetTableRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
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
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.GetTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetTableRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.GetTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableInfo {
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
        if !self.schema_name.is_empty() {
            len += 1;
        }
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if self.table_type != 0 {
            len += 1;
        }
        if self.data_source_format != 0 {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.full_name.is_some() {
            len += 1;
        }
        if self.owner.is_some() {
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
        if self.table_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.TableInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if self.table_type != 0 {
            let v = TableType::try_from(self.table_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.table_type)))?;
            struct_ser.serialize_field("tableType", &v)?;
        }
        if self.data_source_format != 0 {
            let v = DataSourceFormat::try_from(self.data_source_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_source_format)))?;
            struct_ser.serialize_field("dataSourceFormat", &v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.full_name.as_ref() {
            struct_ser.serialize_field("fullName", v)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
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
        if let Some(v) = self.table_id.as_ref() {
            struct_ser.serialize_field("tableId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "schema_name",
            "schemaName",
            "catalog_name",
            "catalogName",
            "table_type",
            "tableType",
            "data_source_format",
            "dataSourceFormat",
            "comment",
            "properties",
            "full_name",
            "fullName",
            "owner",
            "create_at",
            "createAt",
            "created_by",
            "createdBy",
            "update_at",
            "updateAt",
            "updated_by",
            "updatedBy",
            "table_id",
            "tableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SchemaName,
            CatalogName,
            TableType,
            DataSourceFormat,
            Comment,
            Properties,
            FullName,
            Owner,
            CreateAt,
            CreatedBy,
            UpdateAt,
            UpdatedBy,
            TableId,
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
                            "schemaName" | "schema_name" => Ok(GeneratedField::SchemaName),
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "tableType" | "table_type" => Ok(GeneratedField::TableType),
                            "dataSourceFormat" | "data_source_format" => Ok(GeneratedField::DataSourceFormat),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "owner" => Ok(GeneratedField::Owner),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updateAt" | "update_at" => Ok(GeneratedField::UpdateAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.TableInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut schema_name__ = None;
                let mut catalog_name__ = None;
                let mut table_type__ = None;
                let mut data_source_format__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut full_name__ = None;
                let mut owner__ = None;
                let mut create_at__ = None;
                let mut created_by__ = None;
                let mut update_at__ = None;
                let mut updated_by__ = None;
                let mut table_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaName => {
                            if schema_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaName"));
                            }
                            schema_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableType => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableType"));
                            }
                            table_type__ = Some(map_.next_value::<TableType>()? as i32);
                        }
                        GeneratedField::DataSourceFormat => {
                            if data_source_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataSourceFormat"));
                            }
                            data_source_format__ = Some(map_.next_value::<DataSourceFormat>()? as i32);
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
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = map_.next_value()?;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
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
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TableInfo {
                    name: name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    table_type: table_type__.unwrap_or_default(),
                    data_source_format: data_source_format__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                    full_name: full_name__,
                    owner: owner__,
                    create_at: create_at__,
                    created_by: created_by__,
                    update_at: update_at__,
                    updated_by: updated_by__,
                    table_id: table_id__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.TableInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TABLE_TYPE_UNSPECIFIED",
            Self::Managed => "MANAGED",
            Self::External => "EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TableType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TABLE_TYPE_UNSPECIFIED",
            "MANAGED",
            "EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = TableType;

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
                    "TABLE_TYPE_UNSPECIFIED" => Ok(TableType::Unspecified),
                    "MANAGED" => Ok(TableType::Managed),
                    "EXTERNAL" => Ok(TableType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
