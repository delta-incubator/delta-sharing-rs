// @generated
impl serde::Serialize for AuthenticationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTHENTICATION_TYPE_UNSPECIFIED",
            Self::Token => "TOKEN",
            Self::OauthClientCredentials => "OAUTH_CLIENT_CREDENTIALS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTHENTICATION_TYPE_UNSPECIFIED",
            "TOKEN",
            "OAUTH_CLIENT_CREDENTIALS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationType;

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
                    "AUTHENTICATION_TYPE_UNSPECIFIED" => Ok(AuthenticationType::Unspecified),
                    "TOKEN" => Ok(AuthenticationType::Token),
                    "OAUTH_CLIENT_CREDENTIALS" => Ok(AuthenticationType::OauthClientCredentials),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRecipientRequest {
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
        if self.authentication_type != 0 {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.expiration_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.CreateRecipientRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.authentication_type != 0 {
            let v = AuthenticationType::try_from(self.authentication_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.authentication_type)))?;
            struct_ser.serialize_field("authenticationType", &v)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.expiration_time.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("expirationTime", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRecipientRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "authentication_type",
            "authenticationType",
            "owner",
            "comment",
            "properties",
            "expiration_time",
            "expirationTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            AuthenticationType,
            Owner,
            Comment,
            Properties,
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
                            "name" => Ok(GeneratedField::Name),
                            "authenticationType" | "authentication_type" => Ok(GeneratedField::AuthenticationType),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
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
            type Value = CreateRecipientRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.CreateRecipientRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRecipientRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut authentication_type__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut expiration_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthenticationType => {
                            if authentication_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticationType"));
                            }
                            authentication_type__ = Some(map_.next_value::<AuthenticationType>()? as i32);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
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
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(CreateRecipientRequest {
                    name: name__.unwrap_or_default(),
                    authentication_type: authentication_type__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    comment: comment__,
                    properties: properties__,
                    expiration_time: expiration_time__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.CreateRecipientRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRecipientRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.DeleteRecipientRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRecipientRequest {
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
            type Value = DeleteRecipientRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.DeleteRecipientRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRecipientRequest, V::Error>
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
                Ok(DeleteRecipientRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.DeleteRecipientRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRecipientRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.GetRecipientRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRecipientRequest {
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
            type Value = GetRecipientRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.GetRecipientRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRecipientRequest, V::Error>
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
                Ok(GetRecipientRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.GetRecipientRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRecipientsRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.ListRecipientsRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRecipientsRequest {
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
            type Value = ListRecipientsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.ListRecipientsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRecipientsRequest, V::Error>
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
                Ok(ListRecipientsRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.ListRecipientsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRecipientsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.recipients.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.ListRecipientsResponse", len)?;
        if !self.recipients.is_empty() {
            struct_ser.serialize_field("recipients", &self.recipients)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRecipientsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipients",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipients,
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
                            "recipients" => Ok(GeneratedField::Recipients),
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
            type Value = ListRecipientsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.ListRecipientsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRecipientsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipients__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Recipients => {
                            if recipients__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipients"));
                            }
                            recipients__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListRecipientsResponse {
                    recipients: recipients__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.ListRecipientsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecipientInfo {
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
        if self.authentication_type != 0 {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.comment.is_empty() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.created_by.is_some() {
            len += 1;
        }
        if !self.tokens.is_empty() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.updated_by.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.RecipientInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.authentication_type != 0 {
            let v = AuthenticationType::try_from(self.authentication_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.authentication_type)))?;
            struct_ser.serialize_field("authenticationType", &v)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.comment.is_empty() {
            struct_ser.serialize_field("comment", &self.comment)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("createdBy", v)?;
        }
        if !self.tokens.is_empty() {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.updated_by.as_ref() {
            struct_ser.serialize_field("updatedBy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecipientInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "authentication_type",
            "authenticationType",
            "owner",
            "comment",
            "properties",
            "created_at",
            "createdAt",
            "created_by",
            "createdBy",
            "tokens",
            "updated_at",
            "updatedAt",
            "updated_by",
            "updatedBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            AuthenticationType,
            Owner,
            Comment,
            Properties,
            CreatedAt,
            CreatedBy,
            Tokens,
            UpdatedAt,
            UpdatedBy,
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
                            "authenticationType" | "authentication_type" => Ok(GeneratedField::AuthenticationType),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "tokens" => Ok(GeneratedField::Tokens),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecipientInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.RecipientInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecipientInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut authentication_type__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut created_at__ = None;
                let mut created_by__ = None;
                let mut tokens__ = None;
                let mut updated_at__ = None;
                let mut updated_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthenticationType => {
                            if authentication_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticationType"));
                            }
                            authentication_type__ = Some(map_.next_value::<AuthenticationType>()? as i32);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
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
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(RecipientInfo {
                    name: name__.unwrap_or_default(),
                    authentication_type: authentication_type__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    comment: comment__.unwrap_or_default(),
                    properties: properties__,
                    created_at: created_at__,
                    created_by: created_by__,
                    tokens: tokens__.unwrap_or_default(),
                    updated_at: updated_at__,
                    updated_by: updated_by__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.RecipientInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecipientToken {
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
        if self.created_at != 0 {
            len += 1;
        }
        if !self.created_by.is_empty() {
            len += 1;
        }
        if !self.activation_url.is_empty() {
            len += 1;
        }
        if self.expiration_time != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if !self.updated_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.RecipientToken", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if !self.created_by.is_empty() {
            struct_ser.serialize_field("createdBy", &self.created_by)?;
        }
        if !self.activation_url.is_empty() {
            struct_ser.serialize_field("activationUrl", &self.activation_url)?;
        }
        if self.expiration_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("expirationTime", ToString::to_string(&self.expiration_time).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if !self.updated_by.is_empty() {
            struct_ser.serialize_field("updatedBy", &self.updated_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecipientToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "created_at",
            "createdAt",
            "created_by",
            "createdBy",
            "activation_url",
            "activationUrl",
            "expiration_time",
            "expirationTime",
            "updated_at",
            "updatedAt",
            "updated_by",
            "updatedBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            CreatedBy,
            ActivationUrl,
            ExpirationTime,
            UpdatedAt,
            UpdatedBy,
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
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "activationUrl" | "activation_url" => Ok(GeneratedField::ActivationUrl),
                            "expirationTime" | "expiration_time" => Ok(GeneratedField::ExpirationTime),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecipientToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.RecipientToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecipientToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut created_by__ = None;
                let mut activation_url__ = None;
                let mut expiration_time__ = None;
                let mut updated_at__ = None;
                let mut updated_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdBy"));
                            }
                            created_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActivationUrl => {
                            if activation_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activationUrl"));
                            }
                            activation_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedBy => {
                            if updated_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedBy"));
                            }
                            updated_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RecipientToken {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    created_by: created_by__.unwrap_or_default(),
                    activation_url: activation_url__.unwrap_or_default(),
                    expiration_time: expiration_time__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    updated_by: updated_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.RecipientToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRecipientRequest {
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
        if self.new_name.is_some() {
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
        if self.expiration_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.recipients.v1.UpdateRecipientRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.new_name.as_ref() {
            struct_ser.serialize_field("newName", v)?;
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
        if let Some(v) = self.expiration_time.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("expirationTime", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRecipientRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "new_name",
            "newName",
            "owner",
            "comment",
            "properties",
            "expiration_time",
            "expirationTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            NewName,
            Owner,
            Comment,
            Properties,
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
                            "name" => Ok(GeneratedField::Name),
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            "owner" => Ok(GeneratedField::Owner),
                            "comment" => Ok(GeneratedField::Comment),
                            "properties" => Ok(GeneratedField::Properties),
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
            type Value = UpdateRecipientRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.recipients.v1.UpdateRecipientRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRecipientRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut new_name__ = None;
                let mut owner__ = None;
                let mut comment__ = None;
                let mut properties__ = None;
                let mut expiration_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewName => {
                            if new_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newName"));
                            }
                            new_name__ = map_.next_value()?;
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
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(UpdateRecipientRequest {
                    name: name__.unwrap_or_default(),
                    new_name: new_name__,
                    owner: owner__,
                    comment: comment__,
                    properties: properties__,
                    expiration_time: expiration_time__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.recipients.v1.UpdateRecipientRequest", FIELDS, GeneratedVisitor)
    }
}
