// @generated
impl serde::Serialize for DeltaFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaFormat", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Options,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaFormat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(DeltaFormat {
                    provider: provider__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.format.is_some() {
            len += 1;
        }
        if !self.schema_string.is_empty() {
            len += 1;
        }
        if !self.partition_columns.is_empty() {
            len += 1;
        }
        if self.created_time.is_some() {
            len += 1;
        }
        if !self.configuration.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaMetadata", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.format.as_ref() {
            struct_ser.serialize_field("format", v)?;
        }
        if !self.schema_string.is_empty() {
            struct_ser.serialize_field("schemaString", &self.schema_string)?;
        }
        if !self.partition_columns.is_empty() {
            struct_ser.serialize_field("partitionColumns", &self.partition_columns)?;
        }
        if let Some(v) = self.created_time.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdTime", ToString::to_string(&v).as_str())?;
        }
        if !self.configuration.is_empty() {
            struct_ser.serialize_field("configuration", &self.configuration)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "format",
            "schemaString",
            "partitionColumns",
            "createdTime",
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Format,
            SchemaString,
            PartitionColumns,
            CreatedTime,
            Configuration,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "format" => Ok(GeneratedField::Format),
                            "schemaString" => Ok(GeneratedField::SchemaString),
                            "partitionColumns" => Ok(GeneratedField::PartitionColumns),
                            "createdTime" => Ok(GeneratedField::CreatedTime),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut format__ = None;
                let mut schema_string__ = None;
                let mut partition_columns__ = None;
                let mut created_time__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = map_.next_value()?;
                        }
                        GeneratedField::SchemaString => {
                            if schema_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaString"));
                            }
                            schema_string__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartitionColumns => {
                            if partition_columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionColumns"));
                            }
                            partition_columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedTime => {
                            if created_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdTime"));
                            }
                            created_time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(DeltaMetadata {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    format: format__,
                    schema_string: schema_string__.unwrap_or_default(),
                    partition_columns: partition_columns__.unwrap_or_default(),
                    created_time: created_time__,
                    configuration: configuration__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_reader_version != 0 {
            len += 1;
        }
        if self.min_writer_version != 0 {
            len += 1;
        }
        if !self.reader_features.is_empty() {
            len += 1;
        }
        if !self.writer_features.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaProtocol", len)?;
        if self.min_reader_version != 0 {
            struct_ser.serialize_field("minReaderVersion", &self.min_reader_version)?;
        }
        if self.min_writer_version != 0 {
            struct_ser.serialize_field("minWriterVersion", &self.min_writer_version)?;
        }
        if !self.reader_features.is_empty() {
            struct_ser.serialize_field("readerFeatures", &self.reader_features)?;
        }
        if !self.writer_features.is_empty() {
            struct_ser.serialize_field("writerFeatures", &self.writer_features)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minReaderVersion",
            "minWriterVersion",
            "readerFeatures",
            "writerFeatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinReaderVersion,
            MinWriterVersion,
            ReaderFeatures,
            WriterFeatures,
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
                            "minReaderVersion" => Ok(GeneratedField::MinReaderVersion),
                            "minWriterVersion" => Ok(GeneratedField::MinWriterVersion),
                            "readerFeatures" => Ok(GeneratedField::ReaderFeatures),
                            "writerFeatures" => Ok(GeneratedField::WriterFeatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaProtocol")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaProtocol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_reader_version__ = None;
                let mut min_writer_version__ = None;
                let mut reader_features__ = None;
                let mut writer_features__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinReaderVersion => {
                            if min_reader_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minReaderVersion"));
                            }
                            min_reader_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinWriterVersion => {
                            if min_writer_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minWriterVersion"));
                            }
                            min_writer_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReaderFeatures => {
                            if reader_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readerFeatures"));
                            }
                            reader_features__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WriterFeatures => {
                            if writer_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writerFeatures"));
                            }
                            writer_features__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeltaProtocol {
                    min_reader_version: min_reader_version__.unwrap_or_default(),
                    min_writer_version: min_writer_version__.unwrap_or_default(),
                    reader_features: reader_features__.unwrap_or_default(),
                    writer_features: writer_features__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaProtocol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaWrappedAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaWrappedAction", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                delta_wrapped_action::Action::DeltaProtocol(v) => {
                    struct_ser.serialize_field("deltaProtocol", v)?;
                }
                delta_wrapped_action::Action::DeltaMetadata(v) => {
                    struct_ser.serialize_field("deltaMetadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaWrappedAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delta_protocol",
            "deltaProtocol",
            "delta_metadata",
            "deltaMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeltaProtocol,
            DeltaMetadata,
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
                            "deltaProtocol" | "delta_protocol" => Ok(GeneratedField::DeltaProtocol),
                            "deltaMetadata" | "delta_metadata" => Ok(GeneratedField::DeltaMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaWrappedAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaWrappedAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaWrappedAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeltaProtocol => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deltaProtocol"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(delta_wrapped_action::Action::DeltaProtocol)
;
                        }
                        GeneratedField::DeltaMetadata => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deltaMetadata"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(delta_wrapped_action::Action::DeltaMetadata)
;
                        }
                    }
                }
                Ok(DeltaWrappedAction {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaWrappedAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaWrappedMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delta_metadata.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        if self.num_files.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaWrappedMetadata", len)?;
        if let Some(v) = self.delta_metadata.as_ref() {
            struct_ser.serialize_field("deltaMetadata", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.size.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.num_files.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("numFiles", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaWrappedMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deltaMetadata",
            "version",
            "size",
            "numFiles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeltaMetadata,
            Version,
            Size,
            NumFiles,
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
                            "deltaMetadata" => Ok(GeneratedField::DeltaMetadata),
                            "version" => Ok(GeneratedField::Version),
                            "size" => Ok(GeneratedField::Size),
                            "numFiles" => Ok(GeneratedField::NumFiles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaWrappedMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaWrappedMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaWrappedMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delta_metadata__ = None;
                let mut version__ = None;
                let mut size__ = None;
                let mut num_files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeltaMetadata => {
                            if delta_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deltaMetadata"));
                            }
                            delta_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NumFiles => {
                            if num_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiles"));
                            }
                            num_files__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(DeltaWrappedMetadata {
                    delta_metadata: delta_metadata__,
                    version: version__,
                    size: size__,
                    num_files: num_files__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaWrappedMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaWrappedProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delta_protocol.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaWrappedProtocol", len)?;
        if let Some(v) = self.delta_protocol.as_ref() {
            struct_ser.serialize_field("deltaProtocol", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaWrappedProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deltaProtocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeltaProtocol,
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
                            "deltaProtocol" => Ok(GeneratedField::DeltaProtocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaWrappedProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaWrappedProtocol")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaWrappedProtocol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delta_protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeltaProtocol => {
                            if delta_protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deltaProtocol"));
                            }
                            delta_protocol__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeltaWrappedProtocol {
                    delta_protocol: delta_protocol__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaWrappedProtocol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaWrappedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.DeltaWrappedResponse", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaWrappedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
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
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaWrappedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.DeltaWrappedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaWrappedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeltaWrappedResponse {
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.DeltaWrappedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShareRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.share.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetShareRequest", len)?;
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShareRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Share,
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
                            "share" => Ok(GeneratedField::Share),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShareRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetShareRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShareRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetShareRequest {
                    share: share__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetShareRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetShareResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.share.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetShareResponse", len)?;
        if let Some(v) = self.share.as_ref() {
            struct_ser.serialize_field("share", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetShareResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Share,
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
                            "share" => Ok(GeneratedField::Share),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetShareResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetShareResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetShareResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetShareResponse {
                    share: share__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetShareResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableMetadataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.share.is_empty() {
            len += 1;
        }
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.table.is_empty() {
            len += 1;
        }
        if !self.response_format.is_empty() {
            len += 1;
        }
        if !self.reader_features.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetTableMetadataRequest", len)?;
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.table.is_empty() {
            struct_ser.serialize_field("table", &self.table)?;
        }
        if !self.response_format.is_empty() {
            struct_ser.serialize_field("responseFormat", &self.response_format)?;
        }
        if !self.reader_features.is_empty() {
            struct_ser.serialize_field("readerFeatures", &self.reader_features)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableMetadataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share",
            "schema",
            "table",
            "response_format",
            "responseFormat",
            "reader_features",
            "readerFeatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Share,
            Schema,
            Table,
            ResponseFormat,
            ReaderFeatures,
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
                            "share" => Ok(GeneratedField::Share),
                            "schema" => Ok(GeneratedField::Schema),
                            "table" => Ok(GeneratedField::Table),
                            "responseFormat" | "response_format" => Ok(GeneratedField::ResponseFormat),
                            "readerFeatures" | "reader_features" => Ok(GeneratedField::ReaderFeatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableMetadataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetTableMetadataRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableMetadataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share__ = None;
                let mut schema__ = None;
                let mut table__ = None;
                let mut response_format__ = None;
                let mut reader_features__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseFormat => {
                            if response_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseFormat"));
                            }
                            response_format__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReaderFeatures => {
                            if reader_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readerFeatures"));
                            }
                            reader_features__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetTableMetadataRequest {
                    share: share__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                    table: table__.unwrap_or_default(),
                    response_format: response_format__.unwrap_or_default(),
                    reader_features: reader_features__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetTableMetadataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetTableMetadataResponse", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                get_table_metadata_response::Response::Parquet(v) => {
                    struct_ser.serialize_field("parquet", v)?;
                }
                get_table_metadata_response::Response::Delta(v) => {
                    struct_ser.serialize_field("delta", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableMetadataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "parquet",
            "delta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Parquet,
            Delta,
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
                            "version" => Ok(GeneratedField::Version),
                            "parquet" => Ok(GeneratedField::Parquet),
                            "delta" => Ok(GeneratedField::Delta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetTableMetadataResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Parquet => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parquet"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_table_metadata_response::Response::Parquet)
;
                        }
                        GeneratedField::Delta => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delta"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_table_metadata_response::Response::Delta)
;
                        }
                    }
                }
                Ok(GetTableMetadataResponse {
                    version: version__.unwrap_or_default(),
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetTableMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.share.is_empty() {
            len += 1;
        }
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.table.is_empty() {
            len += 1;
        }
        if self.starting_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetTableVersionRequest", len)?;
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.table.is_empty() {
            struct_ser.serialize_field("table", &self.table)?;
        }
        if let Some(v) = self.starting_timestamp.as_ref() {
            struct_ser.serialize_field("startingTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share",
            "schema",
            "table",
            "starting_timestamp",
            "startingTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Share,
            Schema,
            Table,
            StartingTimestamp,
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
                            "share" => Ok(GeneratedField::Share),
                            "schema" => Ok(GeneratedField::Schema),
                            "table" => Ok(GeneratedField::Table),
                            "startingTimestamp" | "starting_timestamp" => Ok(GeneratedField::StartingTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetTableVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share__ = None;
                let mut schema__ = None;
                let mut table__ = None;
                let mut starting_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartingTimestamp => {
                            if starting_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingTimestamp"));
                            }
                            starting_timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTableVersionRequest {
                    share: share__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                    table: table__.unwrap_or_default(),
                    starting_timestamp: starting_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetTableVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTableVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.GetTableVersionResponse", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTableVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTableVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.GetTableVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTableVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetTableVersionResponse {
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.GetTableVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSchemaTablesRequest {
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
        if !self.share.is_empty() {
            len += 1;
        }
        if !self.schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSchemaTablesRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSchemaTablesRequest {
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
            "share",
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
            Share,
            Schema,
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
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "share" => Ok(GeneratedField::Share),
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSchemaTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSchemaTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemaTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut share__ = None;
                let mut schema__ = None;
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
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSchemaTablesRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                    share: share__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSchemaTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSchemaTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSchemaTablesResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSchemaTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
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
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSchemaTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSchemaTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemaTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListSchemaTablesResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSchemaTablesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSchemasRequest {
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
        if !self.share.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSchemasRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSchemasRequest {
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
            "share",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
            Share,
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
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "share" => Ok(GeneratedField::Share),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSchemasRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSchemasRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemasRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut share__ = None;
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
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSchemasRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                    share: share__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSchemasRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSchemasResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSchemasResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSchemasResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
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
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSchemasResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSchemasResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemasResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListSchemasResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSchemasResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListShareTablesRequest {
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
        if !self.share.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListShareTablesRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListShareTablesRequest {
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
            "share",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
            Share,
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
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "share" => Ok(GeneratedField::Share),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListShareTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListShareTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListShareTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut share__ = None;
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
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListShareTablesRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                    share: share__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListShareTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListShareTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListShareTablesResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListShareTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
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
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListShareTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListShareTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListShareTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListShareTablesResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListShareTablesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSharesRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSharesRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSharesRequest {
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
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSharesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSharesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSharesRequest, V::Error>
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
                    }
                }
                Ok(ListSharesRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSharesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ListSharesResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
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
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSharesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ListSharesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSharesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListSharesResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ListSharesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParquetFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ParquetFormat", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParquetFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParquetFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ParquetFormat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParquetFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParquetFormat {
                    provider: provider__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ParquetFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParquetWrappedAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ParquetWrappedAction", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                parquet_wrapped_action::Action::ParquetProtocol(v) => {
                    struct_ser.serialize_field("parquetProtocol", v)?;
                }
                parquet_wrapped_action::Action::ParquetMetadata(v) => {
                    struct_ser.serialize_field("parquetMetadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParquetWrappedAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parquet_protocol",
            "parquetProtocol",
            "parquet_metadata",
            "parquetMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParquetProtocol,
            ParquetMetadata,
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
                            "parquetProtocol" | "parquet_protocol" => Ok(GeneratedField::ParquetProtocol),
                            "parquetMetadata" | "parquet_metadata" => Ok(GeneratedField::ParquetMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParquetWrappedAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ParquetWrappedAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParquetWrappedAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParquetProtocol => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parquetProtocol"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(parquet_wrapped_action::Action::ParquetProtocol)
;
                        }
                        GeneratedField::ParquetMetadata => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parquetMetadata"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(parquet_wrapped_action::Action::ParquetMetadata)
;
                        }
                    }
                }
                Ok(ParquetWrappedAction {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ParquetWrappedAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParquetWrappedMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.format.is_some() {
            len += 1;
        }
        if !self.schema_string.is_empty() {
            len += 1;
        }
        if !self.partition_columns.is_empty() {
            len += 1;
        }
        if !self.configuration.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        if self.num_files.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ParquetWrappedMetadata", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.format.as_ref() {
            struct_ser.serialize_field("format", v)?;
        }
        if !self.schema_string.is_empty() {
            struct_ser.serialize_field("schemaString", &self.schema_string)?;
        }
        if !self.partition_columns.is_empty() {
            struct_ser.serialize_field("partitionColumns", &self.partition_columns)?;
        }
        if !self.configuration.is_empty() {
            struct_ser.serialize_field("configuration", &self.configuration)?;
        }
        if let Some(v) = self.version.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.size.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("size", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.num_files.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("numFiles", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParquetWrappedMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "format",
            "schemaString",
            "partitionColumns",
            "configuration",
            "version",
            "size",
            "numFiles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Format,
            SchemaString,
            PartitionColumns,
            Configuration,
            Version,
            Size,
            NumFiles,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "format" => Ok(GeneratedField::Format),
                            "schemaString" => Ok(GeneratedField::SchemaString),
                            "partitionColumns" => Ok(GeneratedField::PartitionColumns),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "version" => Ok(GeneratedField::Version),
                            "size" => Ok(GeneratedField::Size),
                            "numFiles" => Ok(GeneratedField::NumFiles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParquetWrappedMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ParquetWrappedMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParquetWrappedMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut format__ = None;
                let mut schema_string__ = None;
                let mut partition_columns__ = None;
                let mut configuration__ = None;
                let mut version__ = None;
                let mut size__ = None;
                let mut num_files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = map_.next_value()?;
                        }
                        GeneratedField::SchemaString => {
                            if schema_string__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaString"));
                            }
                            schema_string__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartitionColumns => {
                            if partition_columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionColumns"));
                            }
                            partition_columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NumFiles => {
                            if num_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFiles"));
                            }
                            num_files__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ParquetWrappedMetadata {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    format: format__,
                    schema_string: schema_string__.unwrap_or_default(),
                    partition_columns: partition_columns__.unwrap_or_default(),
                    configuration: configuration__.unwrap_or_default(),
                    version: version__,
                    size: size__,
                    num_files: num_files__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ParquetWrappedMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParquetWrappedProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_reader_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ParquetWrappedProtocol", len)?;
        if self.min_reader_version != 0 {
            struct_ser.serialize_field("minReaderVersion", &self.min_reader_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParquetWrappedProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minReaderVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinReaderVersion,
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
                            "minReaderVersion" => Ok(GeneratedField::MinReaderVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParquetWrappedProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ParquetWrappedProtocol")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParquetWrappedProtocol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_reader_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinReaderVersion => {
                            if min_reader_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minReaderVersion"));
                            }
                            min_reader_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ParquetWrappedProtocol {
                    min_reader_version: min_reader_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ParquetWrappedProtocol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParquetWrappedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.ParquetWrappedResponse", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParquetWrappedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
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
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParquetWrappedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.ParquetWrappedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParquetWrappedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParquetWrappedResponse {
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.ParquetWrappedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Profile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.share_credentials_version != 0 {
            len += 1;
        }
        if !self.endpoint.is_empty() {
            len += 1;
        }
        if !self.bearer_token.is_empty() {
            len += 1;
        }
        if !self.expiration_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.Profile", len)?;
        if self.share_credentials_version != 0 {
            struct_ser.serialize_field("shareCredentialsVersion", &self.share_credentials_version)?;
        }
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if !self.bearer_token.is_empty() {
            struct_ser.serialize_field("bearerToken", &self.bearer_token)?;
        }
        if !self.expiration_time.is_empty() {
            struct_ser.serialize_field("expirationTime", &self.expiration_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Profile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "share_credentials_version",
            "shareCredentialsVersion",
            "endpoint",
            "bearer_token",
            "bearerToken",
            "expiration_time",
            "expirationTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShareCredentialsVersion,
            Endpoint,
            BearerToken,
            ExpirationTime,
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
                            "shareCredentialsVersion" | "share_credentials_version" => Ok(GeneratedField::ShareCredentialsVersion),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "bearerToken" | "bearer_token" => Ok(GeneratedField::BearerToken),
                            "expirationTime" | "expiration_time" => Ok(GeneratedField::ExpirationTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.Profile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Profile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut share_credentials_version__ = None;
                let mut endpoint__ = None;
                let mut bearer_token__ = None;
                let mut expiration_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShareCredentialsVersion => {
                            if share_credentials_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareCredentialsVersion"));
                            }
                            share_credentials_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BearerToken => {
                            if bearer_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bearerToken"));
                            }
                            bearer_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Profile {
                    share_credentials_version: share_credentials_version__.unwrap_or_default(),
                    endpoint: endpoint__.unwrap_or_default(),
                    bearer_token: bearer_token__.unwrap_or_default(),
                    expiration_time: expiration_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Schema {
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
        if !self.share.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.Schema", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Schema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "share",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Share,
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
                            "name" => Ok(GeneratedField::Name),
                            "share" => Ok(GeneratedField::Share),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Schema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.Schema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Schema, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut share__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Schema {
                    name: name__.unwrap_or_default(),
                    share: share__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.Schema", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Share {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.Share", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Share {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Share;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.Share")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Share, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
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
                    }
                }
                Ok(Share {
                    id: id__,
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.Share", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Table {
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
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.share.is_empty() {
            len += 1;
        }
        if self.share_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.v1.Table", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.share.is_empty() {
            struct_ser.serialize_field("share", &self.share)?;
        }
        if let Some(v) = self.share_id.as_ref() {
            struct_ser.serialize_field("shareId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Table {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "schema",
            "share",
            "share_id",
            "shareId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Schema,
            Share,
            ShareId,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "schema" => Ok(GeneratedField::Schema),
                            "share" => Ok(GeneratedField::Share),
                            "shareId" | "share_id" => Ok(GeneratedField::ShareId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.v1.Table")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Table, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut schema__ = None;
                let mut share__ = None;
                let mut share_id__ = None;
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
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Share => {
                            if share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("share"));
                            }
                            share__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShareId => {
                            if share_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shareId"));
                            }
                            share_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Table {
                    id: id__,
                    name: name__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                    share: share__.unwrap_or_default(),
                    share_id: share_id__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.v1.Table", FIELDS, GeneratedVisitor)
    }
}
