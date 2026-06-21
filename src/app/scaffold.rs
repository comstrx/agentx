use crate::config::Paths;
use crate::config::names::STEPS;
use crate::config::templates::{CACHE_GITIGNORE, DEFAULT_TOML};
use crate::core::error::AppResult;
use crate::core::support::fs::{Dir, File, Path};

/// Idempotently create the layout agentx owns: durable `agents/` directories and
/// `overview.md`, the ephemeral `.agentx/` cache, its `.gitignore`, and a default
/// `Agentx.toml`. Existing files are never overwritten.
pub fn run ( paths: &Paths ) -> AppResult<()> {

    for dir in [&paths.contracts, &paths.history, &paths.tasks, &paths.requires] {
        Dir::ensure(dir)?;
    }

    if !Path::exists(&paths.overview) {
        File::write(&paths.overview, "")?;
    }

    for step in STEPS {
        Dir::ensure(&paths.reports_of(step))?;
        Dir::ensure(&paths.rounds_of(step))?;
    }

    for dir in [&paths.tests, &paths.probes, &paths.prompts, &paths.runs] {
        Dir::ensure(dir)?;
    }

    if !Path::exists(&paths.gitignore) {
        File::write(&paths.gitignore, CACHE_GITIGNORE)?;
    }

    if !Path::exists(&paths.config_file) {
        File::write(&paths.config_file, DEFAULT_TOML)?;
    }

    Ok(())

}
