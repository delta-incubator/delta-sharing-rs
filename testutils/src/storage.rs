#![allow(dead_code)]

/// Set environment variable if it is not set
pub fn set_env_if_not_set(key: impl AsRef<str>, value: impl AsRef<str>) {
    if std::env::var(key.as_ref()).is_err() {
        std::env::set_var(key.as_ref(), value.as_ref())
    };
}

/// small wrapper around az cli
mod az_cli {
    use super::set_env_if_not_set;
    use std::process::{Command, ExitStatus};

    /// Create a new bucket
    pub fn create_container(container_name: impl AsRef<str>) -> std::io::Result<ExitStatus> {
        let mut child = Command::new("az")
            .args([
                "storage",
                "container",
                "create",
                "-n",
                container_name.as_ref(),
            ])
            .spawn()
            .expect("az command is installed");
        child.wait()
    }

    /// delete bucket
    pub fn delete_container(container_name: impl AsRef<str>) -> std::io::Result<ExitStatus> {
        let mut child = Command::new("az")
            .args([
                "storage",
                "container",
                "delete",
                "-n",
                container_name.as_ref(),
            ])
            .spawn()
            .expect("az command is installed");
        child.wait()
    }

    /// copy directory
    pub fn copy_directory(
        source: impl AsRef<str>,
        destination: impl AsRef<str>,
    ) -> std::io::Result<ExitStatus> {
        let mut child = Command::new("az")
            .args([
                "storage",
                "blob",
                "upload-batch",
                "-s",
                source.as_ref(),
                "-d",
                destination.as_ref(),
            ])
            .spawn()
            .expect("az command is installed");
        child.wait()
    }

    /// prepare_env
    pub fn prepare_env() {
        set_env_if_not_set("AZURE_STORAGE_USE_EMULATOR", "1");
        set_env_if_not_set("AZURE_STORAGE_ACCOUNT_NAME", "devstoreaccount1");
        set_env_if_not_set("AZURE_STORAGE_ACCOUNT_KEY", "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");
        set_env_if_not_set(
            "AZURE_STORAGE_CONNECTION_STRING",
            "DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://localhost:10000/devstoreaccount1;"
        );
    }
}
