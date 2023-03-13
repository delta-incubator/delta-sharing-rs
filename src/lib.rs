pub mod config;
pub mod infra;
pub mod logging;

pub const VERSION: &str = git_version::git_version!();
