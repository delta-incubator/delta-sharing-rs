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
            #[allow(clippy::needless_borrows_for_generic_args)]
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
        if self.expiration_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.profiles.v1.Profile", len)?;
        if self.share_credentials_version != 0 {
            struct_ser.serialize_field("share_credentials_version", &self.share_credentials_version)?;
        }
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if !self.bearer_token.is_empty() {
            struct_ser.serialize_field("bearer_token", &self.bearer_token)?;
        }
        if let Some(v) = self.expiration_time.as_ref() {
            struct_ser.serialize_field("expiration_time", v)?;
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
                            "shareCredentialsVersion" | "share_credentials_version" => Ok(GeneratedField::ShareCredentialsVersion),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "bearerToken" | "bearer_token" => Ok(GeneratedField::BearerToken),
                            "expirationTime" | "expiration_time" => Ok(GeneratedField::ExpirationTime),
                            _ => Ok(GeneratedField::__SkipField__),
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
                formatter.write_str("struct delta_sharing.profiles.v1.Profile")
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
                            expiration_time__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Profile {
                    share_credentials_version: share_credentials_version__.unwrap_or_default(),
                    endpoint: endpoint__.unwrap_or_default(),
                    bearer_token: bearer_token__.unwrap_or_default(),
                    expiration_time: expiration_time__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.profiles.v1.Profile", FIELDS, GeneratedVisitor)
    }
}
