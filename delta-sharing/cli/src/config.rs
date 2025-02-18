use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub backend: Backend,
    #[serde(default)]
    pub credentials: Vec<StorageCredential>,
}

// catalog backend
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Postgres(PostgresBackendConfig),
    InMemory(InMemoryBackendConfig),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostgresBackendConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InMemoryBackendConfig {
    pub config: String,
}

// storage crendentials

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub name: String,
    pub credential: StorageCredential,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StorageCredential {
    Azure(AzureCredential),
    Gcs(GcsCredential),
    S3(S3Credential),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AzureCredential {
    SharedKey {
        account_name: String,
        account_key: String,
    },
    Sas {
        account_name: String,
        sas: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GcsCredential {
    ServiceAccount { file_path: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum S3Credential {
    AccessKey {
        access_key_id: String,
        secret_access_key: String,
    },
    Profile {
        profile: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_config() {
        let config = r#"
            {
                "url": "http://localhost:8080",
                "backend": {
                    "postgres": {
                        "url": "postgres://localhost:5432"
                    }
                }
            }
        "#;

        let config: Config = serde_json::from_str(config).unwrap();
        assert_eq!(config.url, "http://localhost:8080");
        assert!(matches!(config.backend, Backend::Postgres(_)));
    }
}
