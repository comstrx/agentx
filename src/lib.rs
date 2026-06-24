#![forbid(unsafe_code)]

pub mod core;
pub mod config;
pub mod app;

pub use app::{App, Flags};
pub use core::prelude::*;
