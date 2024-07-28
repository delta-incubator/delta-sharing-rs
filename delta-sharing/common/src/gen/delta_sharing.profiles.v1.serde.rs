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
            __SkipField__,
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
                            _ => Ok(GeneratedField::__SkipField__),
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
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
            __SkipField__,
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
                            _ => Ok(GeneratedField::__SkipField__),
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
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
