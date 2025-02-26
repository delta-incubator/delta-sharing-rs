// @generated
impl serde::Serialize for CreateSchemaRequest {
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
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.CreateSchemaRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
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
impl<'de> serde::Deserialize<'de> for CreateSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "catalog_name",
            "catalogName",
            "comment",
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
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
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
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
            type Value = CreateSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.CreateSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
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
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
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
                Ok(CreateSchemaRequest {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.CreateSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSchemaRequest {
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
        if self.force.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.DeleteSchemaRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        if let Some(v) = self.force.as_ref() {
            struct_ser.serialize_field("force", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
            "force",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
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
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
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
            type Value = DeleteSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.DeleteSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                let mut force__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
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
                Ok(DeleteSchemaRequest {
                    full_name: full_name__.unwrap_or_default(),
                    force: force__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.DeleteSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSchemaRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.GetSchemaRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSchemaRequest {
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
            type Value = GetSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.GetSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSchemaRequest, V::Error>
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
                Ok(GetSchemaRequest {
                    full_name: full_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.GetSchemaRequest", FIELDS, GeneratedVisitor)
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
        if !self.catalog_name.is_empty() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.ListSchemasRequest", len)?;
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
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
            "catalog_name",
            "catalogName",
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CatalogName,
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
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
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
            type Value = ListSchemasRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.ListSchemasRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemasRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut catalog_name__ = None;
                let mut max_results__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListSchemasRequest {
                    catalog_name: catalog_name__.unwrap_or_default(),
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.ListSchemasRequest", FIELDS, GeneratedVisitor)
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
        if !self.schemas.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.ListSchemasResponse", len)?;
        if !self.schemas.is_empty() {
            struct_ser.serialize_field("schemas", &self.schemas)?;
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
            "schemas",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schemas,
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
                            "schemas" => Ok(GeneratedField::Schemas),
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
            type Value = ListSchemasResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.ListSchemasResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSchemasResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schemas__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schemas => {
                            if schemas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemas"));
                            }
                            schemas__ = Some(map_.next_value()?);
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
                Ok(ListSchemasResponse {
                    schemas: schemas__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.ListSchemasResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SchemaInfo {
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
        if !self.catalog_name.is_empty() {
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
        if self.schema_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.SchemaInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.catalog_name.is_empty() {
            struct_ser.serialize_field("catalogName", &self.catalog_name)?;
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
        if let Some(v) = self.schema_id.as_ref() {
            struct_ser.serialize_field("schemaId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SchemaInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "catalog_name",
            "catalogName",
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
            "schema_id",
            "schemaId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CatalogName,
            Comment,
            Properties,
            FullName,
            Owner,
            CreateAt,
            CreatedBy,
            UpdateAt,
            UpdatedBy,
            SchemaId,
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
                            "catalogName" | "catalog_name" => Ok(GeneratedField::CatalogName),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "owner" => Ok(GeneratedField::Owner),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updateAt" | "update_at" => Ok(GeneratedField::UpdateAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            "schemaId" | "schema_id" => Ok(GeneratedField::SchemaId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SchemaInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.SchemaInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SchemaInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut catalog_name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut full_name__ = None;
                let mut owner__ = None;
                let mut create_at__ = None;
                let mut created_by__ = None;
                let mut update_at__ = None;
                let mut updated_by__ = None;
                let mut schema_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CatalogName => {
                            if catalog_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogName"));
                            }
                            catalog_name__ = Some(map_.next_value()?);
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
                        GeneratedField::SchemaId => {
                            if schema_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemaId"));
                            }
                            schema_id__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SchemaInfo {
                    name: name__.unwrap_or_default(),
                    catalog_name: catalog_name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                    full_name: full_name__,
                    owner: owner__,
                    create_at: create_at__,
                    created_by: created_by__,
                    update_at: update_at__,
                    updated_by: updated_by__,
                    schema_id: schema_id__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.SchemaInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSchemaRequest {
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
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if !self.new_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.schemas.v1.UpdateSchemaRequest", len)?;
        if !self.full_name.is_empty() {
            struct_ser.serialize_field("fullName", &self.full_name)?;
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
impl<'de> serde::Deserialize<'de> for UpdateSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_name",
            "fullName",
            "comment",
            "properties",
            "new_name",
            "newName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullName,
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
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
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
            type Value = UpdateSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.schemas.v1.UpdateSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_name__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut new_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = Some(map_.next_value()?);
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
                Ok(UpdateSchemaRequest {
                    full_name: full_name__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                    new_name: new_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.schemas.v1.UpdateSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
