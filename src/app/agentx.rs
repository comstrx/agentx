use std::path::Path as StdPath;

use crate::core::error::AppResult;
use super::lifecycle;

/// The public API. Embed agentx in another Rust program, or drive it from the
/// CLI — every command routes through here.
#[derive(Clone, Copy, Debug, Default)]
pub struct Agentx;

impl Agentx {

    /// Scaffold `Agentx.toml`, `agents/`, and `.agentx/` in `dir`.
    pub fn init ( dir: &StdPath ) -> AppResult<()> {

        lifecycle::init(dir)

    }

    /// Resolve the project root from `dir` and run a full cycle.
    pub fn start ( dir: &StdPath ) -> AppResult<()> {

        lifecycle::start(dir)

    }

    /// Stop the running cycle and its agents immediately.
    pub fn stop ( dir: &StdPath ) -> AppResult<()> {

        lifecycle::stop(dir)

    }

    /// Stop cleanly after the current turn (state kept, resumable).
    pub fn drain ( dir: &StdPath ) -> AppResult<()> {

        lifecycle::drain(dir)

    }

    /// Delete the `.agentx` cache.
    pub fn clean ( dir: &StdPath ) -> AppResult<()> {

        lifecycle::clean(dir)

    }

}
