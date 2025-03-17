// @generated
impl serde::Serialize for ColumnInfo {
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
        if !self.type_text.is_empty() {
            len += 1;
        }
        if !self.type_json.is_empty() {
            len += 1;
        }
        if self.type_name != 0 {
            len += 1;
        }
        if self.type_precision.is_some() {
            len += 1;
        }
        if self.type_scale.is_some() {
            len += 1;
        }
        if self.type_interval_type.is_some() {
            len += 1;
        }
        if self.position.is_some() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.nullable.is_some() {
            len += 1;
        }
        if self.partition_index.is_some() {
            len += 1;
        }
        if self.column_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.ColumnInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.type_text.is_empty() {
            struct_ser.serialize_field("typeText", &self.type_text)?;
        }
        if !self.type_json.is_empty() {
            struct_ser.serialize_field("typeJson", &self.type_json)?;
        }
        if self.type_name != 0 {
            let v = ColumnTypeName::try_from(self.type_name)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.type_name)))?;
            struct_ser.serialize_field("typeName", &v)?;
        }
        if let Some(v) = self.type_precision.as_ref() {
            struct_ser.serialize_field("typePrecision", v)?;
        }
        if let Some(v) = self.type_scale.as_ref() {
            struct_ser.serialize_field("typeScale", v)?;
        }
        if let Some(v) = self.type_interval_type.as_ref() {
            struct_ser.serialize_field("typeIntervalType", v)?;
        }
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.nullable.as_ref() {
            struct_ser.serialize_field("nullable", v)?;
        }
        if let Some(v) = self.partition_index.as_ref() {
            struct_ser.serialize_field("partitionIndex", v)?;
        }
        if let Some(v) = self.column_id.as_ref() {
            struct_ser.serialize_field("columnId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type_text",
            "typeText",
            "type_json",
            "typeJson",
            "type_name",
            "typeName",
            "type_precision",
            "typePrecision",
            "type_scale",
            "typeScale",
            "type_interval_type",
            "typeIntervalType",
            "position",
            "comment",
            "nullable",
            "partition_index",
            "partitionIndex",
            "column_id",
            "columnId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypeText,
            TypeJson,
            TypeName,
            TypePrecision,
            TypeScale,
            TypeIntervalType,
            Position,
            Comment,
            Nullable,
            PartitionIndex,
            ColumnId,
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
                            "typeText" | "type_text" => Ok(GeneratedField::TypeText),
                            "typeJson" | "type_json" => Ok(GeneratedField::TypeJson),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "typePrecision" | "type_precision" => Ok(GeneratedField::TypePrecision),
                            "typeScale" | "type_scale" => Ok(GeneratedField::TypeScale),
                            "typeIntervalType" | "type_interval_type" => Ok(GeneratedField::TypeIntervalType),
                            "position" => Ok(GeneratedField::Position),
                            "comment" => Ok(GeneratedField::Comment),
                            "nullable" => Ok(GeneratedField::Nullable),
                            "partitionIndex" | "partition_index" => Ok(GeneratedField::PartitionIndex),
                            "columnId" | "column_id" => Ok(GeneratedField::ColumnId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.ColumnInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut type_text__ = None;
                let mut type_json__ = None;
                let mut type_name__ = None;
                let mut type_precision__ = None;
                let mut type_scale__ = None;
                let mut type_interval_type__ = None;
                let mut position__ = None;
                let mut comment__ = None;
                let mut nullable__ = None;
                let mut partition_index__ = None;
                let mut column_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeText => {
                            if type_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeText"));
                            }
                            type_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeJson => {
                            if type_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeJson"));
                            }
                            type_json__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = Some(map_.next_value::<ColumnTypeName>()? as i32);
                        }
                        GeneratedField::TypePrecision => {
                            if type_precision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typePrecision"));
                            }
                            type_precision__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TypeScale => {
                            if type_scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeScale"));
                            }
                            type_scale__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TypeIntervalType => {
                            if type_interval_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeIntervalType"));
                            }
                            type_interval_type__ = map_.next_value()?;
                        }
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Nullable => {
                            if nullable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullable"));
                            }
                            nullable__ = map_.next_value()?;
                        }
                        GeneratedField::PartitionIndex => {
                            if partition_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionIndex"));
                            }
                            partition_index__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ColumnId => {
                            if column_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnId"));
                            }
                            column_id__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ColumnInfo {
                    name: name__.unwrap_or_default(),
                    type_text: type_text__.unwrap_or_default(),
                    type_json: type_json__.unwrap_or_default(),
                    type_name: type_name__.unwrap_or_default(),
                    type_precision: type_precision__,
                    type_scale: type_scale__,
                    type_interval_type: type_interval_type__,
                    position: position__,
                    comment: comment__,
                    nullable: nullable__,
                    partition_index: partition_index__,
                    column_id: column_id__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.ColumnInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnTypeName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COLUMN_TYPE_NAME_UNSPECIFIED",
            Self::Boolean => "BOOLEAN",
            Self::Byte => "BYTE",
            Self::Short => "SHORT",
            Self::Int => "INT",
            Self::Long => "LONG",
            Self::Float => "FLOAT",
            Self::Double => "DOUBLE",
            Self::Date => "DATE",
            Self::Timestamp => "TIMESTAMP",
            Self::String => "STRING",
            Self::Binary => "BINARY",
            Self::Decimal => "DECIMAL",
            Self::Interval => "INTERVAL",
            Self::Array => "ARRAY",
            Self::Struct => "STRUCT",
            Self::Map => "MAP",
            Self::Char => "CHAR",
            Self::Null => "NULL",
            Self::UserDefinedType => "USER_DEFINED_TYPE",
            Self::TimestampNtz => "TIMESTAMP_NTZ",
            Self::Variant => "VARIANT",
            Self::TableType => "TABLE_TYPE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ColumnTypeName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COLUMN_TYPE_NAME_UNSPECIFIED",
            "BOOLEAN",
            "BYTE",
            "SHORT",
            "INT",
            "LONG",
            "FLOAT",
            "DOUBLE",
            "DATE",
            "TIMESTAMP",
            "STRING",
            "BINARY",
            "DECIMAL",
            "INTERVAL",
            "ARRAY",
            "STRUCT",
            "MAP",
            "CHAR",
            "NULL",
            "USER_DEFINED_TYPE",
            "TIMESTAMP_NTZ",
            "VARIANT",
            "TABLE_TYPE",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = ColumnTypeName;

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
                    "COLUMN_TYPE_NAME_UNSPECIFIED" => Ok(ColumnTypeName::Unspecified),
                    "BOOLEAN" => Ok(ColumnTypeName::Boolean),
                    "BYTE" => Ok(ColumnTypeName::Byte),
                    "SHORT" => Ok(ColumnTypeName::Short),
                    "INT" => Ok(ColumnTypeName::Int),
                    "LONG" => Ok(ColumnTypeName::Long),
                    "FLOAT" => Ok(ColumnTypeName::Float),
                    "DOUBLE" => Ok(ColumnTypeName::Double),
                    "DATE" => Ok(ColumnTypeName::Date),
                    "TIMESTAMP" => Ok(ColumnTypeName::Timestamp),
                    "STRING" => Ok(ColumnTypeName::String),
                    "BINARY" => Ok(ColumnTypeName::Binary),
                    "DECIMAL" => Ok(ColumnTypeName::Decimal),
                    "INTERVAL" => Ok(ColumnTypeName::Interval),
                    "ARRAY" => Ok(ColumnTypeName::Array),
                    "STRUCT" => Ok(ColumnTypeName::Struct),
                    "MAP" => Ok(ColumnTypeName::Map),
                    "CHAR" => Ok(ColumnTypeName::Char),
                    "NULL" => Ok(ColumnTypeName::Null),
                    "USER_DEFINED_TYPE" => Ok(ColumnTypeName::UserDefinedType),
                    "TIMESTAMP_NTZ" => Ok(ColumnTypeName::TimestampNtz),
                    "VARIANT" => Ok(ColumnTypeName::Variant),
                    "TABLE_TYPE" => Ok(ColumnTypeName::TableType),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTableRequest {
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
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.storage_location.is_some() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.CreateTableRequest", len)?;
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
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if let Some(v) = self.storage_location.as_ref() {
            struct_ser.serialize_field("storageLocation", v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTableRequest {
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
            "columns",
            "storage_location",
            "storageLocation",
            "comment",
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SchemaName,
            CatalogName,
            TableType,
            DataSourceFormat,
            Columns,
            StorageLocation,
            Comment,
            Properties,
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
                            "columns" => Ok(GeneratedField::Columns),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.CreateTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut schema_name__ = None;
                let mut catalog_name__ = None;
                let mut table_type__ = None;
                let mut data_source_format__ = None;
                let mut columns__ = None;
                let mut storage_location__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
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
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageLocation => {
                            if storage_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            storage_location__ = map_.next_value()?;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateTableRequest {
                    name: name__.unwrap_or_default(),
                    schema_name: schema_name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    table_type: table_type__.unwrap_or_default(),
                    data_source_format: data_source_format__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                    storage_location: storage_location__,
                    comment: comment__,
                    properties: properties__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.CreateTableRequest", FIELDS, GeneratedVisitor)
    }
}
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
impl serde::Serialize for DeleteTableRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.DeleteTableRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTableRequest {
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
            type Value = DeleteTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.DeleteTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTableRequest, V::Error>
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
                Ok(DeleteTableRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.DeleteTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableExistsRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.GetTableExistsRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableExistsRequest {
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
            type Value = GetTableExistsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.GetTableExistsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableExistsRequest, V::Error>
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
                Ok(GetTableExistsRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.GetTableExistsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableExistsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_exists {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.GetTableExistsResponse", len)?;
        if self.table_exists {
            struct_ser.serialize_field("tableExists", &self.table_exists)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableExistsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_exists",
            "tableExists",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableExists,
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
                            "tableExists" | "table_exists" => Ok(GeneratedField::TableExists),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableExistsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.GetTableExistsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableExistsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_exists__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableExists => {
                            if table_exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableExists"));
                            }
                            table_exists__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetTableExistsResponse {
                    table_exists: table_exists__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.GetTableExistsResponse", FIELDS, GeneratedVisitor)
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
        if self.include_delta_metadata.is_some() {
            len += 1;
        }
        if self.include_browse.is_some() {
            len += 1;
        }
        if self.include_manifest_capabilities.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.GetTableRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if let Some(v) = self.include_delta_metadata.as_ref() {
            struct_ser.serialize_field("includeDeltaMetadata", v)?;
        }
        if let Some(v) = self.include_browse.as_ref() {
            struct_ser.serialize_field("includeBrowse", v)?;
        }
        if let Some(v) = self.include_manifest_capabilities.as_ref() {
            struct_ser.serialize_field("includeManifestCapabilities", v)?;
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
            "include_delta_metadata",
            "includeDeltaMetadata",
            "include_browse",
            "includeBrowse",
            "include_manifest_capabilities",
            "includeManifestCapabilities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
            IncludeDeltaMetadata,
            IncludeBrowse,
            IncludeManifestCapabilities,
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
                            "includeDeltaMetadata" | "include_delta_metadata" => Ok(GeneratedField::IncludeDeltaMetadata),
                            "includeBrowse" | "include_browse" => Ok(GeneratedField::IncludeBrowse),
                            "includeManifestCapabilities" | "include_manifest_capabilities" => Ok(GeneratedField::IncludeManifestCapabilities),
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
                let mut include_delta_metadata__ = None;
                let mut include_browse__ = None;
                let mut include_manifest_capabilities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeDeltaMetadata => {
                            if include_delta_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeDeltaMetadata"));
                            }
                            include_delta_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeBrowse => {
                            if include_browse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeBrowse"));
                            }
                            include_browse__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeManifestCapabilities => {
                            if include_manifest_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeManifestCapabilities"));
                            }
                            include_manifest_capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetTableRequest {
                    full_name: full_name__.unwrap_or_default(),
                    include_delta_metadata: include_delta_metadata__,
                    include_browse: include_browse__,
                    include_manifest_capabilities: include_manifest_capabilities__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.GetTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTableSummariesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if self.schema_name_pattern.is_some() {
            len += 1;
        }
        if self.table_name_pattern.is_some() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        if self.include_manifest_capabilities.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.ListTableSummariesRequest", len)?;
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if let Some(v) = self.schema_name_pattern.as_ref() {
            struct_ser.serialize_field("schemaNamePattern", v)?;
        }
        if let Some(v) = self.table_name_pattern.as_ref() {
            struct_ser.serialize_field("tableNamePattern", v)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        if let Some(v) = self.include_manifest_capabilities.as_ref() {
            struct_ser.serialize_field("includeManifestCapabilities", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTableSummariesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "catalog_name",
            "catalogName",
            "schema_name_pattern",
            "schemaNamePattern",
            "table_name_pattern",
            "tableNamePattern",
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
            "include_manifest_capabilities",
            "includeManifestCapabilities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CatalogName,
            SchemaNamePattern,
            TableNamePattern,
            MaxResults,
            PageToken,
            IncludeManifestCapabilities,
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
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "schemaNamePattern" | "schema_name_pattern" => Ok(GeneratedField::SchemaNamePattern),
                            "tableNamePattern" | "table_name_pattern" => Ok(GeneratedField::TableNamePattern),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "includeManifestCapabilities" | "include_manifest_capabilities" => Ok(GeneratedField::IncludeManifestCapabilities),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTableSummariesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.ListTableSummariesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTableSummariesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut catalog_name__ = None;
                let mut schema_name_pattern__ = None;
                let mut table_name_pattern__ = None;
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut include_manifest_capabilities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaNamePattern => {
                            if schema_name_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaNamePattern"));
                            }
                            schema_name_pattern__ = map_.next_value()?;
                        }
                        GeneratedField::TableNamePattern => {
                            if table_name_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNamePattern"));
                            }
                            table_name_pattern__ = map_.next_value()?;
                        }
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
                        GeneratedField::IncludeManifestCapabilities => {
                            if include_manifest_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeManifestCapabilities"));
                            }
                            include_manifest_capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListTableSummariesRequest {
                    catalog_name: catalog_name__.unwrap_or_default(),
                    schema_name_pattern: schema_name_pattern__,
                    table_name_pattern: table_name_pattern__,
                    max_results: max_results__,
                    page_token: page_token__,
                    include_manifest_capabilities: include_manifest_capabilities__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.ListTableSummariesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTableSummariesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tables.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.ListTableSummariesResponse", len)?;
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTableSummariesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tables",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tables,
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
                            "tables" => Ok(GeneratedField::Tables),
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
            type Value = ListTableSummariesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.ListTableSummariesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTableSummariesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tables__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
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
                Ok(ListTableSummariesResponse {
                    tables: tables__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.ListTableSummariesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema_name.is_empty() {
            len += 1;
        }
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        if self.include_delta_metadata.is_some() {
            len += 1;
        }
        if self.omit_columns.is_some() {
            len += 1;
        }
        if self.omit_properties.is_some() {
            len += 1;
        }
        if self.omit_username.is_some() {
            len += 1;
        }
        if self.include_browse.is_some() {
            len += 1;
        }
        if self.include_manifest_capabilities.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.ListTablesRequest", len)?;
        if !self.schema_name.is_empty() {
            struct_ser.serialize_field("schemaName", &self.schema_name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        if let Some(v) = self.include_delta_metadata.as_ref() {
            struct_ser.serialize_field("includeDeltaMetadata", v)?;
        }
        if let Some(v) = self.omit_columns.as_ref() {
            struct_ser.serialize_field("omitColumns", v)?;
        }
        if let Some(v) = self.omit_properties.as_ref() {
            struct_ser.serialize_field("omitProperties", v)?;
        }
        if let Some(v) = self.omit_username.as_ref() {
            struct_ser.serialize_field("omitUsername", v)?;
        }
        if let Some(v) = self.include_browse.as_ref() {
            struct_ser.serialize_field("includeBrowse", v)?;
        }
        if let Some(v) = self.include_manifest_capabilities.as_ref() {
            struct_ser.serialize_field("includeManifestCapabilities", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema_name",
            "schemaName",
            "catalog_name",
            "catalogName",
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
            "include_delta_metadata",
            "includeDeltaMetadata",
            "omit_columns",
            "omitColumns",
            "omit_properties",
            "omitProperties",
            "omit_username",
            "omitUsername",
            "include_browse",
            "includeBrowse",
            "include_manifest_capabilities",
            "includeManifestCapabilities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SchemaName,
            CatalogName,
            MaxResults,
            PageToken,
            IncludeDeltaMetadata,
            OmitColumns,
            OmitProperties,
            OmitUsername,
            IncludeBrowse,
            IncludeManifestCapabilities,
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
                            "schemaName" | "schema_name" => Ok(GeneratedField::SchemaName),
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "includeDeltaMetadata" | "include_delta_metadata" => Ok(GeneratedField::IncludeDeltaMetadata),
                            "omitColumns" | "omit_columns" => Ok(GeneratedField::OmitColumns),
                            "omitProperties" | "omit_properties" => Ok(GeneratedField::OmitProperties),
                            "omitUsername" | "omit_username" => Ok(GeneratedField::OmitUsername),
                            "includeBrowse" | "include_browse" => Ok(GeneratedField::IncludeBrowse),
                            "includeManifestCapabilities" | "include_manifest_capabilities" => Ok(GeneratedField::IncludeManifestCapabilities),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.ListTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema_name__ = None;
                let mut catalog_name__ = None;
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut include_delta_metadata__ = None;
                let mut omit_columns__ = None;
                let mut omit_properties__ = None;
                let mut omit_username__ = None;
                let mut include_browse__ = None;
                let mut include_manifest_capabilities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::IncludeDeltaMetadata => {
                            if include_delta_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeDeltaMetadata"));
                            }
                            include_delta_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::OmitColumns => {
                            if omit_columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitColumns"));
                            }
                            omit_columns__ = map_.next_value()?;
                        }
                        GeneratedField::OmitProperties => {
                            if omit_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitProperties"));
                            }
                            omit_properties__ = map_.next_value()?;
                        }
                        GeneratedField::OmitUsername => {
                            if omit_username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitUsername"));
                            }
                            omit_username__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeBrowse => {
                            if include_browse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeBrowse"));
                            }
                            include_browse__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeManifestCapabilities => {
                            if include_manifest_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeManifestCapabilities"));
                            }
                            include_manifest_capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListTablesRequest {
                    schema_name: schema_name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    max_results: max_results__,
                    page_token: page_token__,
                    include_delta_metadata: include_delta_metadata__,
                    omit_columns: omit_columns__,
                    omit_properties: omit_properties__,
                    omit_username: omit_username__,
                    include_browse: include_browse__,
                    include_manifest_capabilities: include_manifest_capabilities__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.ListTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tables.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.ListTablesResponse", len)?;
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tables",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tables,
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
                            "tables" => Ok(GeneratedField::Tables),
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
            type Value = ListTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.ListTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tables__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
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
                Ok(ListTablesResponse {
                    tables: tables__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.ListTablesResponse", FIELDS, GeneratedVisitor)
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
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.storage_location.is_some() {
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
        if self.storage_credential_name.is_some() {
            len += 1;
        }
        if self.full_name.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.created_by.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.updated_by.is_some() {
            len += 1;
        }
        if self.deleted_at.is_some() {
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
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if let Some(v) = self.storage_location.as_ref() {
            struct_ser.serialize_field("storageLocation", v)?;
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
        if let Some(v) = self.storage_credential_name.as_ref() {
            struct_ser.serialize_field("storageCredentialName", v)?;
        }
        if let Some(v) = self.full_name.as_ref() {
            struct_ser.serialize_field("fullName", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("createdBy", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.updated_by.as_ref() {
            struct_ser.serialize_field("updatedBy", v)?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deletedAt", ToString::to_string(&v).as_str())?;
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
            "columns",
            "storage_location",
            "storageLocation",
            "owner",
            "comment",
            "properties",
            "storage_credential_name",
            "storageCredentialName",
            "full_name",
            "fullName",
            "created_at",
            "createdAt",
            "created_by",
            "createdBy",
            "updated_at",
            "updatedAt",
            "updated_by",
            "updatedBy",
            "deleted_at",
            "deletedAt",
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
            Columns,
            StorageLocation,
            Owner,
            Comment,
            Properties,
            StorageCredentialName,
            FullName,
            CreatedAt,
            CreatedBy,
            UpdatedAt,
            UpdatedBy,
            DeletedAt,
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
                            "columns" => Ok(GeneratedField::Columns),
                            "storageLocation" | "storage_location" => Ok(GeneratedField::StorageLocation),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "storageCredentialName" | "storage_credential_name" => Ok(GeneratedField::StorageCredentialName),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            "deletedAt" | "deleted_at" => Ok(GeneratedField::DeletedAt),
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
                let mut columns__ = None;
                let mut storage_location__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut storage_credential_name__ = None;
                let mut full_name__ = None;
                let mut created_at__ = None;
                let mut created_by__ = None;
                let mut updated_at__ = None;
                let mut updated_by__ = None;
                let mut deleted_at__ = None;
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
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageLocation => {
                            if storage_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocation"));
                            }
                            storage_location__ = map_.next_value()?;
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
                        GeneratedField::StorageCredentialName => {
                            if storage_credential_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageCredentialName"));
                            }
                            storage_credential_name__ = map_.next_value()?;
                        }
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdBy"));
                            }
                            created_by__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UpdatedBy => {
                            if updated_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedBy"));
                            }
                            updated_by__ = map_.next_value()?;
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedAt"));
                            }
                            deleted_at__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
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
                    columns: columns__.unwrap_or_default(),
                    storage_location: storage_location__,
                    owner: owner__,
                    comment: comment__,
                    properties: properties__,
                    storage_credential_name: storage_credential_name__,
                    full_name: full_name__,
                    created_at: created_at__,
                    created_by: created_by__,
                    updated_at: updated_at__,
                    updated_by: updated_by__,
                    deleted_at: deleted_at__,
                    table_id: table_id__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.TableInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableSummary {
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
        if self.table_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.tables.v1.TableSummary", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if self.table_type != 0 {
            let v = TableType::try_from(self.table_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.table_type)))?;
            struct_ser.serialize_field("tableType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
            "table_type",
            "tableType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
            TableType,
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
                            "tableType" | "table_type" => Ok(GeneratedField::TableType),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.tables.v1.TableSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                let mut table_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableType => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableType"));
                            }
                            table_type__ = Some(map_.next_value::<TableType>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TableSummary {
                    full_name: full_name__.unwrap_or_default(),
                    table_type: table_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.tables.v1.TableSummary", FIELDS, GeneratedVisitor)
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
