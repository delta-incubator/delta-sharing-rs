// @generated
impl serde::Serialize for AzureClientCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant_id.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureClientCredential", len)?;
        if !self.tenant_id.is_empty() {
            struct_ser.serialize_field("tenantId", &self.tenant_id)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                azure_client_credential::Credential::ClientSecret(v) => {
                    struct_ser.serialize_field("clientSecret", v)?;
                }
                azure_client_credential::Credential::ClientCertificate(v) => {
                    struct_ser.serialize_field("clientCertificate", v)?;
                }
                azure_client_credential::Credential::FederatedTokenFile(v) => {
                    struct_ser.serialize_field("federatedTokenFile", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureClientCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant_id",
            "tenantId",
            "client_id",
            "clientId",
            "client_secret",
            "clientSecret",
            "client_certificate",
            "clientCertificate",
            "federated_token_file",
            "federatedTokenFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TenantId,
            ClientId,
            ClientSecret,
            ClientCertificate,
            FederatedTokenFile,
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
                            "tenantId" | "tenant_id" => Ok(GeneratedField::TenantId),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
                            "clientCertificate" | "client_certificate" => Ok(GeneratedField::ClientCertificate),
                            "federatedTokenFile" | "federated_token_file" => Ok(GeneratedField::FederatedTokenFile),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureClientCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureClientCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureClientCredential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant_id__ = None;
                let mut client_id__ = None;
                let mut credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TenantId => {
                            if tenant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantId"));
                            }
                            tenant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSecret => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_client_credential::Credential::ClientSecret);
                        }
                        GeneratedField::ClientCertificate => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientCertificate"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_client_credential::Credential::ClientCertificate);
                        }
                        GeneratedField::FederatedTokenFile => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("federatedTokenFile"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_client_credential::Credential::FederatedTokenFile);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureClientCredential {
                    tenant_id: tenant_id__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureClientCredential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureCredential", len)?;
        if let Some(v) = self.credential.as_ref() {
            match v {
                azure_credential::Credential::AccountKey(v) => {
                    struct_ser.serialize_field("accountKey", v)?;
                }
                azure_credential::Credential::Sas(v) => {
                    struct_ser.serialize_field("sas", v)?;
                }
                azure_credential::Credential::Client(v) => {
                    struct_ser.serialize_field("client", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_key",
            "accountKey",
            "sas",
            "client",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountKey,
            Sas,
            Client,
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
                            "accountKey" | "account_key" => Ok(GeneratedField::AccountKey),
                            "sas" => Ok(GeneratedField::Sas),
                            "client" => Ok(GeneratedField::Client),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureCredential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountKey => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountKey"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_credential::Credential::AccountKey)
;
                        }
                        GeneratedField::Sas => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sas"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_credential::Credential::Sas)
;
                        }
                        GeneratedField::Client => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("client"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_credential::Credential::Client)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureCredential {
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureCredential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureKeyCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_name.is_empty() {
            len += 1;
        }
        if !self.account_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureKeyCredential", len)?;
        if !self.account_name.is_empty() {
            struct_ser.serialize_field("accountName", &self.account_name)?;
        }
        if !self.account_key.is_empty() {
            struct_ser.serialize_field("accountKey", &self.account_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureKeyCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_name",
            "accountName",
            "account_key",
            "accountKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountName,
            AccountKey,
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
                            "accountName" | "account_name" => Ok(GeneratedField::AccountName),
                            "accountKey" | "account_key" => Ok(GeneratedField::AccountKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureKeyCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureKeyCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureKeyCredential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_name__ = None;
                let mut account_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountName => {
                            if account_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountName"));
                            }
                            account_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountKey => {
                            if account_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountKey"));
                            }
                            account_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureKeyCredential {
                    account_name: account_name__.unwrap_or_default(),
                    account_key: account_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureKeyCredential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureSasCredential {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_name.is_empty() {
            len += 1;
        }
        if !self.sas_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureSasCredential", len)?;
        if !self.account_name.is_empty() {
            struct_ser.serialize_field("accountName", &self.account_name)?;
        }
        if !self.sas_token.is_empty() {
            struct_ser.serialize_field("sasToken", &self.sas_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureSasCredential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_name",
            "accountName",
            "sas_token",
            "sasToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountName,
            SasToken,
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
                            "accountName" | "account_name" => Ok(GeneratedField::AccountName),
                            "sasToken" | "sas_token" => Ok(GeneratedField::SasToken),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureSasCredential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureSasCredential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureSasCredential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_name__ = None;
                let mut sas_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountName => {
                            if account_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountName"));
                            }
                            account_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SasToken => {
                            if sas_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sasToken"));
                            }
                            sas_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureSasCredential {
                    account_name: account_name__.unwrap_or_default(),
                    sas_token: sas_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureSasCredential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCredentialRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.CreateCredentialRequest", len)?;
        if let Some(v) = self.credential.as_ref() {
            struct_ser.serialize_field("credential", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCredentialRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credential",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Credential,
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
                            "credential" => Ok(GeneratedField::Credential),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCredentialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.CreateCredentialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCredentialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Credential => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credential"));
                            }
                            credential__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateCredentialRequest {
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.CreateCredentialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateStorageLocationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.location.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.CreateStorageLocationRequest", len)?;
        if let Some(v) = self.location.as_ref() {
            struct_ser.serialize_field("location", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateStorageLocationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
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
                            "location" => Ok(GeneratedField::Location),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateStorageLocationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.CreateStorageLocationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateStorageLocationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateStorageLocationRequest {
                    location: location__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.CreateStorageLocationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Credential {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.Credential", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                credential::Credential::Azure(v) => {
                    struct_ser.serialize_field("azure", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Credential {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "azure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Azure,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "azure" => Ok(GeneratedField::Azure),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Credential;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.Credential")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Credential, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut credential__ = None;
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
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Azure => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azure"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(credential::Credential::Azure)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Credential {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.Credential", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteCredentialRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.DeleteCredentialRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteCredentialRequest {
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteCredentialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.DeleteCredentialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteCredentialRequest, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteCredentialRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.DeleteCredentialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteStorageLocationRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.DeleteStorageLocationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteStorageLocationRequest {
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteStorageLocationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.DeleteStorageLocationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteStorageLocationRequest, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteStorageLocationRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.DeleteStorageLocationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCredentialRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.GetCredentialRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCredentialRequest {
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCredentialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.GetCredentialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCredentialRequest, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCredentialRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.GetCredentialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStorageLocationRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.GetStorageLocationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStorageLocationRequest {
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStorageLocationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.GetStorageLocationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStorageLocationRequest, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetStorageLocationRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.GetStorageLocationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStorageLocationsRequest {
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
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.ListStorageLocationsRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStorageLocationsRequest {
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
            type Value = ListStorageLocationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.ListStorageLocationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStorageLocationsRequest, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListStorageLocationsRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.ListStorageLocationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStorageLocationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.storage_locations.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.ListStorageLocationsResponse", len)?;
        if !self.storage_locations.is_empty() {
            struct_ser.serialize_field("storageLocations", &self.storage_locations)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStorageLocationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "storage_locations",
            "storageLocations",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StorageLocations,
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
                            "storageLocations" | "storage_locations" => Ok(GeneratedField::StorageLocations),
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
            type Value = ListStorageLocationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.ListStorageLocationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStorageLocationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut storage_locations__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StorageLocations => {
                            if storage_locations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLocations"));
                            }
                            storage_locations__ = Some(map_.next_value()?);
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
                Ok(ListStorageLocationsResponse {
                    storage_locations: storage_locations__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.ListStorageLocationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StorageLocation {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.credential.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.StorageLocation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if self.r#type != 0 {
            let v = StorageType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.credential.is_empty() {
            struct_ser.serialize_field("credential", &self.credential)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "url",
            "type",
            "credential",
            "description",
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Url,
            Type,
            Credential,
            Description,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "url" => Ok(GeneratedField::Url),
                            "type" => Ok(GeneratedField::Type),
                            "credential" => Ok(GeneratedField::Credential),
                            "description" => Ok(GeneratedField::Description),
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
            type Value = StorageLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.StorageLocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut url__ = None;
                let mut r#type__ = None;
                let mut credential__ = None;
                let mut description__ = None;
                let mut properties__ = None;
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
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<StorageType>()? as i32);
                        }
                        GeneratedField::Credential => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credential"));
                            }
                            credential__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
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
                Ok(StorageLocation {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    credential: credential__.unwrap_or_default(),
                    description: description__,
                    properties: properties__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.StorageLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StorageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STORAGE_TYPE_UNSPECIFIED",
            Self::Azure => "STORAGE_TYPE_AZURE",
            Self::Google => "STORAGE_TYPE_GOOGLE",
            Self::S3 => "STORAGE_TYPE_S3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StorageType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STORAGE_TYPE_UNSPECIFIED",
            "STORAGE_TYPE_AZURE",
            "STORAGE_TYPE_GOOGLE",
            "STORAGE_TYPE_S3",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = StorageType;

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
                    "STORAGE_TYPE_UNSPECIFIED" => Ok(StorageType::Unspecified),
                    "STORAGE_TYPE_AZURE" => Ok(StorageType::Azure),
                    "STORAGE_TYPE_GOOGLE" => Ok(StorageType::Google),
                    "STORAGE_TYPE_S3" => Ok(StorageType::S3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
