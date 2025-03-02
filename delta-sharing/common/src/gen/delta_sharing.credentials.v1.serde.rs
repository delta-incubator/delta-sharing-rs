// @generated
impl serde::Serialize for AzureManagedIdentity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureManagedIdentity", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                azure_managed_identity::Identifier::ObjectId(v) => {
                    struct_ser.serialize_field("object_id", v)?;
                }
                azure_managed_identity::Identifier::ApplicationId(v) => {
                    struct_ser.serialize_field("application_id", v)?;
                }
                azure_managed_identity::Identifier::MsiResourceId(v) => {
                    struct_ser.serialize_field("msi_resource_id", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureManagedIdentity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_id",
            "objectId",
            "application_id",
            "applicationId",
            "msi_resource_id",
            "msiResourceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectId,
            ApplicationId,
            MsiResourceId,
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
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "msiResourceId" | "msi_resource_id" => Ok(GeneratedField::MsiResourceId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureManagedIdentity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureManagedIdentity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureManagedIdentity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_managed_identity::Identifier::ObjectId);
                        }
                        GeneratedField::ApplicationId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_managed_identity::Identifier::ApplicationId);
                        }
                        GeneratedField::MsiResourceId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msiResourceId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_managed_identity::Identifier::MsiResourceId);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureManagedIdentity {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureManagedIdentity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureServicePrincipal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directory_id.is_empty() {
            len += 1;
        }
        if !self.application_id.is_empty() {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.AzureServicePrincipal", len)?;
        if !self.directory_id.is_empty() {
            struct_ser.serialize_field("directory_id", &self.directory_id)?;
        }
        if !self.application_id.is_empty() {
            struct_ser.serialize_field("application_id", &self.application_id)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                azure_service_principal::Credential::ClientSecret(v) => {
                    struct_ser.serialize_field("client_secret", v)?;
                }
                azure_service_principal::Credential::FederatedTokenFile(v) => {
                    struct_ser.serialize_field("federated_token_file", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureServicePrincipal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directory_id",
            "directoryId",
            "application_id",
            "applicationId",
            "client_secret",
            "clientSecret",
            "federated_token_file",
            "federatedTokenFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectoryId,
            ApplicationId,
            ClientSecret,
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
                            "directoryId" | "directory_id" => Ok(GeneratedField::DirectoryId),
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
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
            type Value = AzureServicePrincipal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.AzureServicePrincipal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureServicePrincipal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directory_id__ = None;
                let mut application_id__ = None;
                let mut credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DirectoryId => {
                            if directory_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryId"));
                            }
                            directory_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationId => {
                            if application_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            application_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSecret => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_service_principal::Credential::ClientSecret);
                        }
                        GeneratedField::FederatedTokenFile => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("federatedTokenFile"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_service_principal::Credential::FederatedTokenFile);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AzureServicePrincipal {
                    directory_id: directory_id__.unwrap_or_default(),
                    application_id: application_id__.unwrap_or_default(),
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.AzureServicePrincipal", FIELDS, GeneratedVisitor)
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.purpose != 0 {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.read_only.is_some() {
            len += 1;
        }
        if self.skip_validation {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.CreateCredentialRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.purpose != 0 {
            let v = Purpose::try_from(self.purpose)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.purpose)))?;
            struct_ser.serialize_field("purpose", &v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.read_only.as_ref() {
            struct_ser.serialize_field("read_only", v)?;
        }
        if self.skip_validation {
            struct_ser.serialize_field("skip_validation", &self.skip_validation)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                create_credential_request::Credential::AzureServicePrincipal(v) => {
                    struct_ser.serialize_field("azure_service_principal", v)?;
                }
                create_credential_request::Credential::AzureManagedIdentity(v) => {
                    struct_ser.serialize_field("azure_managed_identity", v)?;
                }
            }
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
            "name",
            "purpose",
            "comment",
            "read_only",
            "readOnly",
            "skip_validation",
            "skipValidation",
            "azure_service_principal",
            "azureServicePrincipal",
            "azure_managed_identity",
            "azureManagedIdentity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Purpose,
            Comment,
            ReadOnly,
            SkipValidation,
            AzureServicePrincipal,
            AzureManagedIdentity,
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
                            "purpose" => Ok(GeneratedField::Purpose),
                            "comment" => Ok(GeneratedField::Comment),
                            "readOnly" | "read_only" => Ok(GeneratedField::ReadOnly),
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            "azureServicePrincipal" | "azure_service_principal" => Ok(GeneratedField::AzureServicePrincipal),
                            "azureManagedIdentity" | "azure_managed_identity" => Ok(GeneratedField::AzureManagedIdentity),
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
                let mut name__ = None;
                let mut purpose__ = None;
                let mut comment__ = None;
                let mut read_only__ = None;
                let mut skip_validation__ = None;
                let mut credential__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Purpose => {
                            if purpose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            purpose__ = Some(map_.next_value::<Purpose>()? as i32);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::ReadOnly => {
                            if read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readOnly"));
                            }
                            read_only__ = map_.next_value()?;
                        }
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AzureServicePrincipal => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureServicePrincipal"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(create_credential_request::Credential::AzureServicePrincipal)
;
                        }
                        GeneratedField::AzureManagedIdentity => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureManagedIdentity"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(create_credential_request::Credential::AzureManagedIdentity)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateCredentialRequest {
                    name: name__.unwrap_or_default(),
                    purpose: purpose__.unwrap_or_default(),
                    comment: comment__,
                    read_only: read_only__,
                    skip_validation: skip_validation__.unwrap_or_default(),
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.CreateCredentialRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CredentialInfo {
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
        if self.purpose != 0 {
            len += 1;
        }
        if self.read_only {
            len += 1;
        }
        if self.comment.is_some() {
            len += 1;
        }
        if self.owner.is_some() {
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
        if self.used_for_managed_storage {
            len += 1;
        }
        if self.full_name.is_some() {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.CredentialInfo", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.purpose != 0 {
            let v = Purpose::try_from(self.purpose)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.purpose)))?;
            struct_ser.serialize_field("purpose", &v)?;
        }
        if self.read_only {
            struct_ser.serialize_field("read_only", &self.read_only)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("created_at", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("created_by", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updated_at", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.updated_by.as_ref() {
            struct_ser.serialize_field("updated_by", v)?;
        }
        if self.used_for_managed_storage {
            struct_ser.serialize_field("used_for_managed_storage", &self.used_for_managed_storage)?;
        }
        if let Some(v) = self.full_name.as_ref() {
            struct_ser.serialize_field("full_name", v)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                credential_info::Credential::AzureServicePrincipal(v) => {
                    struct_ser.serialize_field("azure_service_principal", v)?;
                }
                credential_info::Credential::AzureManagedIdentity(v) => {
                    struct_ser.serialize_field("azure_managed_identity", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CredentialInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "purpose",
            "read_only",
            "readOnly",
            "comment",
            "owner",
            "created_at",
            "createdAt",
            "created_by",
            "createdBy",
            "updated_at",
            "updatedAt",
            "updated_by",
            "updatedBy",
            "used_for_managed_storage",
            "usedForManagedStorage",
            "full_name",
            "fullName",
            "azure_service_principal",
            "azureServicePrincipal",
            "azure_managed_identity",
            "azureManagedIdentity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Purpose,
            ReadOnly,
            Comment,
            Owner,
            CreatedAt,
            CreatedBy,
            UpdatedAt,
            UpdatedBy,
            UsedForManagedStorage,
            FullName,
            AzureServicePrincipal,
            AzureManagedIdentity,
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
                            "purpose" => Ok(GeneratedField::Purpose),
                            "readOnly" | "read_only" => Ok(GeneratedField::ReadOnly),
                            "comment" => Ok(GeneratedField::Comment),
                            "owner" => Ok(GeneratedField::Owner),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "updatedBy" | "updated_by" => Ok(GeneratedField::UpdatedBy),
                            "usedForManagedStorage" | "used_for_managed_storage" => Ok(GeneratedField::UsedForManagedStorage),
                            "fullName" | "full_name" => Ok(GeneratedField::FullName),
                            "azureServicePrincipal" | "azure_service_principal" => Ok(GeneratedField::AzureServicePrincipal),
                            "azureManagedIdentity" | "azure_managed_identity" => Ok(GeneratedField::AzureManagedIdentity),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CredentialInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.CredentialInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CredentialInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut purpose__ = None;
                let mut read_only__ = None;
                let mut comment__ = None;
                let mut owner__ = None;
                let mut created_at__ = None;
                let mut created_by__ = None;
                let mut updated_at__ = None;
                let mut updated_by__ = None;
                let mut used_for_managed_storage__ = None;
                let mut full_name__ = None;
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
                        GeneratedField::Purpose => {
                            if purpose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            purpose__ = Some(map_.next_value::<Purpose>()? as i32);
                        }
                        GeneratedField::ReadOnly => {
                            if read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readOnly"));
                            }
                            read_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
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
                        GeneratedField::UsedForManagedStorage => {
                            if used_for_managed_storage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedForManagedStorage"));
                            }
                            used_for_managed_storage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FullName => {
                            if full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullName"));
                            }
                            full_name__ = map_.next_value()?;
                        }
                        GeneratedField::AzureServicePrincipal => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureServicePrincipal"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(credential_info::Credential::AzureServicePrincipal)
;
                        }
                        GeneratedField::AzureManagedIdentity => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureManagedIdentity"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(credential_info::Credential::AzureManagedIdentity)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CredentialInfo {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    purpose: purpose__.unwrap_or_default(),
                    read_only: read_only__.unwrap_or_default(),
                    comment: comment__,
                    owner: owner__,
                    created_at: created_at__,
                    created_by: created_by__,
                    updated_at: updated_at__,
                    updated_by: updated_by__,
                    used_for_managed_storage: used_for_managed_storage__.unwrap_or_default(),
                    full_name: full_name__,
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.CredentialInfo", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ListCredentialsRequest {
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
        if self.purpose.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.ListCredentialsRequest", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("max_results", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("page_token", v)?;
        }
        if let Some(v) = self.purpose.as_ref() {
            let v = Purpose::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("purpose", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCredentialsRequest {
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
            "purpose",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
            Purpose,
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
                            "purpose" => Ok(GeneratedField::Purpose),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListCredentialsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.ListCredentialsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCredentialsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                let mut purpose__ = None;
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
                        GeneratedField::Purpose => {
                            if purpose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            purpose__ = map_.next_value::<::std::option::Option<Purpose>>()?.map(|x| x as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListCredentialsRequest {
                    max_results: max_results__,
                    page_token: page_token__,
                    purpose: purpose__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.ListCredentialsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCredentialsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credentials.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.ListCredentialsResponse", len)?;
        if !self.credentials.is_empty() {
            struct_ser.serialize_field("credentials", &self.credentials)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("next_page_token", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCredentialsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credentials",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Credentials,
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
                            "credentials" => Ok(GeneratedField::Credentials),
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
            type Value = ListCredentialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.ListCredentialsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCredentialsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credentials__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = Some(map_.next_value()?);
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
                Ok(ListCredentialsResponse {
                    credentials: credentials__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.ListCredentialsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Purpose {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PURPOSE_UNSPECIFIED",
            Self::Storage => "STORAGE",
            Self::Service => "SERVICE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Purpose {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PURPOSE_UNSPECIFIED",
            "STORAGE",
            "SERVICE",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = Purpose;

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
                    "PURPOSE_UNSPECIFIED" => Ok(Purpose::Unspecified),
                    "STORAGE" => Ok(Purpose::Storage),
                    "SERVICE" => Ok(Purpose::Service),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCredentialRequest {
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
        if self.comment.is_some() {
            len += 1;
        }
        if self.read_only.is_some() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        if self.skip_validation.is_some() {
            len += 1;
        }
        if self.force.is_some() {
            len += 1;
        }
        if self.credential.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("delta_sharing.credentials.v1.UpdateCredentialRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.new_name.as_ref() {
            struct_ser.serialize_field("new_name", v)?;
        }
        if let Some(v) = self.comment.as_ref() {
            struct_ser.serialize_field("comment", v)?;
        }
        if let Some(v) = self.read_only.as_ref() {
            struct_ser.serialize_field("read_only", v)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        if let Some(v) = self.skip_validation.as_ref() {
            struct_ser.serialize_field("skip_validation", v)?;
        }
        if let Some(v) = self.force.as_ref() {
            struct_ser.serialize_field("force", v)?;
        }
        if let Some(v) = self.credential.as_ref() {
            match v {
                update_credential_request::Credential::AzureServicePrincipal(v) => {
                    struct_ser.serialize_field("azure_service_principal", v)?;
                }
                update_credential_request::Credential::AzureManagedIdentity(v) => {
                    struct_ser.serialize_field("azure_managed_identity", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCredentialRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "new_name",
            "newName",
            "comment",
            "read_only",
            "readOnly",
            "owner",
            "skip_validation",
            "skipValidation",
            "force",
            "azure_service_principal",
            "azureServicePrincipal",
            "azure_managed_identity",
            "azureManagedIdentity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            NewName,
            Comment,
            ReadOnly,
            Owner,
            SkipValidation,
            Force,
            AzureServicePrincipal,
            AzureManagedIdentity,
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
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            "comment" => Ok(GeneratedField::Comment),
                            "readOnly" | "read_only" => Ok(GeneratedField::ReadOnly),
                            "owner" => Ok(GeneratedField::Owner),
                            "skipValidation" | "skip_validation" => Ok(GeneratedField::SkipValidation),
                            "force" => Ok(GeneratedField::Force),
                            "azureServicePrincipal" | "azure_service_principal" => Ok(GeneratedField::AzureServicePrincipal),
                            "azureManagedIdentity" | "azure_managed_identity" => Ok(GeneratedField::AzureManagedIdentity),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCredentialRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct delta_sharing.credentials.v1.UpdateCredentialRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCredentialRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut new_name__ = None;
                let mut comment__ = None;
                let mut read_only__ = None;
                let mut owner__ = None;
                let mut skip_validation__ = None;
                let mut force__ = None;
                let mut credential__ = None;
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
                        GeneratedField::Comment => {
                            if comment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            comment__ = map_.next_value()?;
                        }
                        GeneratedField::ReadOnly => {
                            if read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readOnly"));
                            }
                            read_only__ = map_.next_value()?;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                        GeneratedField::SkipValidation => {
                            if skip_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipValidation"));
                            }
                            skip_validation__ = map_.next_value()?;
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = map_.next_value()?;
                        }
                        GeneratedField::AzureServicePrincipal => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureServicePrincipal"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(update_credential_request::Credential::AzureServicePrincipal)
;
                        }
                        GeneratedField::AzureManagedIdentity => {
                            if credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureManagedIdentity"));
                            }
                            credential__ = map_.next_value::<::std::option::Option<_>>()?.map(update_credential_request::Credential::AzureManagedIdentity)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateCredentialRequest {
                    name: name__.unwrap_or_default(),
                    new_name: new_name__,
                    comment: comment__,
                    read_only: read_only__,
                    owner: owner__,
                    skip_validation: skip_validation__,
                    force: force__,
                    credential: credential__,
                })
            }
        }
        deserializer.deserialize_struct("delta_sharing.credentials.v1.UpdateCredentialRequest", FIELDS, GeneratedVisitor)
    }
}
