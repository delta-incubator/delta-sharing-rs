// @generated
impl serde::Serialize for CreateProfileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.claims.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.CreateProfileRequest", len)?;
        if !self.claims.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("claims", pbjson::private::base64::encode(&self.claims).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProfileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "claims",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Claims,
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
                            "claims" => Ok(GeneratedField::Claims),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProfileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.profiles.v1.CreateProfileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProfileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut claims__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Claims => {
                            if claims__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claims"));
                            }
                            claims__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateProfileRequest {
                    claims: claims__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.CreateProfileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProfileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.profile.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.CreateProfileResponse", len)?;
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProfileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "profile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Profile,
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
                            "profile" => Ok(GeneratedField::Profile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProfileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.profiles.v1.CreateProfileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProfileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut profile__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateProfileResponse {
                    profile: profile__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.CreateProfileResponse", FIELDS, GeneratedVisitor)
    }
}
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.CreateSchemaRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.profiles.v1.CreateSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateSchemaRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.CreateSchemaRequest", FIELDS, GeneratedVisitor)
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
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.CreateTableRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
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
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Location,
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
                            "location" => Ok(GeneratedField::Location),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                formatter.write_str("struct delta_sharing.profiles.v1.CreateTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTableRequest {
                    name: name__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.CreateTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.CreateTableResponse", len)?;
        if !self.table_uri.is_empty() {
            struct_ser.serialize_field("tableUri", &self.table_uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_uri",
            "tableUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableUri,
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
                            "tableUri" | "table_uri" => Ok(GeneratedField::TableUri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.profiles.v1.CreateTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableUri => {
                            if table_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableUri"));
                            }
                            table_uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTableResponse {
                    table_uri: table_uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.CreateTableResponse", FIELDS, GeneratedVisitor)
    }
}
