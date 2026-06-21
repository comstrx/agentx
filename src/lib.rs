//! agentx — hierarchical multi-agent orchestrator.
//!
//! The crate is built in four layers:
//! - [`core`]   neutral foundation (error, context, support) — knows nothing about agentx.
//! - `config`   static definitions (names, defaults, paths, spec, prompts, templates).
//! - `app`      the orchestrator itself — all business logic.
//! - `cli`      a thin terminal wrapper over `app`.

#![forbid(unsafe_code)]

pub mod core;
pub mod config;
pub mod app;
pub mod cli;

pub use app::Agentx;
pub use core::prelude::*;
