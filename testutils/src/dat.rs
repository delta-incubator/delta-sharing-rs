#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use delta_kernel::snapshot::Snapshot;
use delta_kernel::{Engine, Error, Table, Version};
use futures::stream::TryStreamExt;
use object_store::{local::LocalFileSystem, ObjectStore};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum AssertionError {
    #[error("Invalid test case data")]
    InvalidTestCase,

    #[error("Kernel error: {0}")]
    KernelError(#[from] Error),
}

pub type DatResult<T, E = AssertionError> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct TestCaseInfoJson {
    name: String,
    description: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TestCaseInfo {
    name: String,
    description: String,
    root_dir: PathBuf,
}

impl TestCaseInfo {
    /// Root path for this test cases Delta table.
    pub fn table_root(&self) -> DatResult<Url> {
        let table_root = self.root_dir.join("delta");
        Url::from_directory_path(table_root).map_err(|_| AssertionError::InvalidTestCase)
    }

    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    async fn versions(&self) -> DatResult<(TableVersionMetaData, Vec<TableVersionMetaData>)> {
        let expected_root = self.root_dir.join("expected");
        let store = LocalFileSystem::new_with_prefix(&expected_root).unwrap();

        let files = store.list(None).try_collect::<Vec<_>>().await.unwrap();

        let raw_cases = files.into_iter().filter(|meta| {
            meta.location.filename() == Some("table_version_metadata.json")
                && !meta
                    .location
                    .prefix_matches(&object_store::path::Path::from("latest"))
        });

        let mut cases = Vec::new();
        for case in raw_cases {
            let case_file = expected_root.join(case.location.as_ref());
            let file = File::open(case_file).map_err(|_| AssertionError::InvalidTestCase)?;
            let info: TableVersionMetaData =
                serde_json::from_reader(file).map_err(|_| AssertionError::InvalidTestCase)?;
            cases.push(info);
        }

        let case_file = expected_root.join("latest/table_version_metadata.json");
        let file = File::open(case_file).map_err(|_| AssertionError::InvalidTestCase)?;
        let latest: TableVersionMetaData =
            serde_json::from_reader(file).map_err(|_| AssertionError::InvalidTestCase)?;

        Ok((latest, cases))
    }

    fn assert_snapshot_meta(
        &self,
        case: &TableVersionMetaData,
        snapshot: &Snapshot,
    ) -> DatResult<()> {
        assert_eq!(snapshot.version(), case.version);

        // assert correct metadata is read
        let metadata = snapshot.metadata();
        let protocol = snapshot.protocol();
        let tvm = TableVersionMetaData {
            version: snapshot.version(),
            properties: metadata
                .configuration
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            min_reader_version: protocol.min_reader_version() as u32,
            min_writer_version: protocol.min_writer_version() as u32,
        };
        assert_eq!(&tvm, case);
        Ok(())
    }

    pub async fn assert_metadata(&self, engine: Arc<dyn Engine>) -> DatResult<()> {
        let engine = engine.as_ref();
        let table = Table::new(self.table_root()?);

        let (latest, versions) = self.versions().await?;

        let snapshot = table.snapshot(engine, None)?;
        self.assert_snapshot_meta(&latest, &snapshot)?;

        for table_version in versions {
            let snapshot = table.snapshot(engine, Some(table_version.version))?;
            self.assert_snapshot_meta(&table_version, &snapshot)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TableVersionMetaData {
    version: Version,
    properties: HashMap<String, String>,
    min_reader_version: u32,
    min_writer_version: u32,
}

pub fn read_dat_case(case_root: impl AsRef<Path>) -> DatResult<TestCaseInfo> {
    let info_path = case_root.as_ref().join("test_case_info.json");
    let file = File::open(info_path).map_err(|_| AssertionError::InvalidTestCase)?;
    let info: TestCaseInfoJson =
        serde_json::from_reader(file).map_err(|_| AssertionError::InvalidTestCase)?;
    Ok(TestCaseInfo {
        root_dir: case_root.as_ref().into(),
        name: info.name,
        description: info.description,
    })
}

pub fn find_dat_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("DAT_DATA_DIR") {
        let path = std::path::Path::new(&dir);
        return std::fs::canonicalize(path).ok();
    }
    if let Ok(path) = std::fs::canonicalize(std::path::Path::new("./dat")) {
        return Some(path);
    }
    if let Some(root) = find_git_root() {
        return std::fs::canonicalize(root.join("dat")).ok();
    }
    None
}

fn find_git_root() -> Option<PathBuf> {
    // extract dat bundle
    let child = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("git command is installed");
    let out = child.wait_with_output().ok()?;
    let path = String::from_utf8(out.stdout).ok()?.trim().to_string();
    std::fs::canonicalize(std::path::Path::new(&path)).ok()
}
