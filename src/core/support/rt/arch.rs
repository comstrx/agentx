use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// One shared multi-thread runtime, built on first use.
pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// The async/sync bridge: run futures to completion from synchronous code, so
/// the upper layers never turn `async`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rt;
