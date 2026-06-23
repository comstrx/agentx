#![forbid(unsafe_code)]

pub mod core;
pub mod config;
pub mod app;

pub use app::App;
pub use core::prelude::*;
