mod arch;
mod claude;
mod codex;

use std::path::Path as StdPath;

use crate::core::error::AppResult;
use claude::Claude;
use codex::Codex;

pub use arch::{Reply, Worker, model_of};

static CLAUDE: Claude = Claude;
static CODEX: Codex = Codex;

/// Pick the backend for an agent id by its model prefix.
pub fn dispatch ( agent: &str ) -> &'static dyn Worker {

    if model_of(agent).starts_with("codex") { &CODEX } else { &CLAUDE }

}

/// Run one agent turn.
pub fn run ( agent: &str, prompt: &str, session: Option<&str>, cwd: &StdPath, timeout: u64, active: &StdPath ) -> AppResult<Reply> {

    dispatch(agent).invoke(prompt, session, cwd, timeout, active)

}
