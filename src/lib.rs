#![allow(dead_code)]
pub mod config;
pub mod logging;
pub mod server;
pub mod wrappers;

pub const VERSION: &str = git_version::git_version!();
