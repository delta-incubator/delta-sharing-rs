#![allow(dead_code)]
pub mod config;
pub mod error;
pub mod logging;
mod macros;
pub mod middlewares;
pub mod server;
pub mod utils;

pub const VERSION: &str = git_version::git_version!();
