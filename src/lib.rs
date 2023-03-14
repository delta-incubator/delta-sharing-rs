#![allow(dead_code)]
pub mod config;
pub mod infra;
pub mod logging;
pub mod server;

pub const VERSION: &str = git_version::git_version!();
