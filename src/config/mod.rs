//! Static definitions — constants, defaults, paths, spec, prompts, templates.
//! Depends only on `core`; holds no runtime state.

pub mod arch;
pub mod defaults;
pub mod names;
pub mod paths;
pub mod prompts;
pub mod spec;
pub mod templates;

pub use arch::{Config, Context, Paths, Spec};
