use std::path::Path;
use std::process::Command;

use clap::{Parser, Subcommand};
use object_store::azure::MicrosoftAzureBuilder;
use object_store::local::LocalFileSystem;
use object_store::ObjectStore;

use crate::utils::TestResult;

mod dat;
mod storage;
mod utils;

#[derive(Parser)]
#[command(name = "delta-sharing", version, about = "CLI to manage delta.sharing services.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Load delta acceptance test tabeles.")]
    LoadDat(DatArgs),
    SyncAzurite(SyncAzArgs),
}

#[derive(Parser)]
struct DatArgs {
    #[clap(long, default_value = "0.0.3")]
    version: String,

    #[clap(long, short, default_value = "dat")]
    output: String,
}

#[derive(Parser)]
struct SyncAzArgs {
    #[clap(long, default_value = "dat")]
    dat_source: String,

    #[clap(long, short, default_value = "dat")]
    container: String,
}

#[tokio::main]
pub async fn main() -> TestResult {
    let args = Cli::parse();

    match args.command {
        Commands::LoadDat(args) => load_dat(args).await?,
        Commands::SyncAzurite(args) => load_azurite(args).await?,
    }

    Ok(())
}

async fn load_azurite(args: SyncAzArgs) -> TestResult {
    let _azurite = MicrosoftAzureBuilder::new()
        .with_use_emulator(true)
        .with_container_name(args.container)
        .build()?;

    Ok(())
}

async fn load_dat(args: DatArgs) -> TestResult {
    // download dat bundle
    let dat_url = format!(
        "https://github.com/delta-incubator/dat/releases/download/v{}/deltalake-dat-v{}.tar.gz",
        args.version, args.version
    );
    // check if dat path exists
    let dat_out = Path::new(&args.output);
    if !dat_out.exists() {
        std::fs::create_dir(&args.output)?;
    }
    let dat_out = dat_out.canonicalize()?;

    let resp = reqwest::get(&dat_url).await?.bytes().await?;

    let tmp_dir = tempfile::tempdir()?;
    let tmp_fs = LocalFileSystem::new_with_prefix(tmp_dir.path())?;
    tmp_fs
        .put(&object_store::path::Path::from("dat.tar.gz"), resp.into())
        .await?;
    let tar_file = tmp_dir.path().join("dat.tar.gz");

    // extract dat bundle
    let mut child = Command::new("tar")
        .args([
            "--no-same-permissions",
            "-xzf",
            tar_file.to_str().unwrap(),
            "--directory",
            dat_out.to_str().unwrap(),
        ])
        .spawn()
        .expect("tar command is installed");
    child.wait()?;

    Ok(())
}
