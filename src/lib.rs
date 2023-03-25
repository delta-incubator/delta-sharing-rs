#![allow(dead_code)]
mod bootstrap;
pub mod config;
pub mod logging;
mod macros;
pub mod server;

pub const VERSION: &str = git_version::git_version!();
