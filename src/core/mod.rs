//! Neutral foundation — knows nothing about agentx.
//!
//! Three pillars, in dependency order:
//! - `error`    typed errors + report + exit codes (depends on nothing).
//! - `context`  global, thread-safe key/value store with a small DSL (depends on `error`).
//! - `support`  a personal std-lib: fs, net, parse, proc, thread, text, time, num, list.

pub mod error;
pub mod context;
pub mod support;
pub mod prelude;

pub use error::{AppError, AppExitCode, AppResult};
pub use context::{AppContext, ContextValue};
