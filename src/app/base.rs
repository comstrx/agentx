use std::path::Path as StdPath;

use crate::config::names::STEPS;
use crate::config::{Config, Paths, spec};
use crate::core::error::AppResult;
use crate::core::support::fs::File;
use super::discovery;

/// Assemble the full run configuration from a resolved root.
pub fn assemble ( root: &StdPath ) -> AppResult<Config> {

    let paths = Paths::new(root);
    let spec = spec::load(&paths.config_file)?;
    let context = discovery::discover(&paths);

    Ok(Config { root: root.to_path_buf(), spec, paths, context })

}

/// The steps to run, given which context exists: `arch` only when there are
/// requirements; `work`/`test` whenever there is anything to build.
pub fn resolve_steps ( config: &Config ) -> Vec<String> {

    let context = &config.context;

    if !context.has_requires() && !context.has_tasks() {
        return Vec::new();
    }

    let enabled = |name: &str| config.spec.steps.iter().any(|step| step.as_str() == name);

    STEPS.iter().copied().filter(|&step| match step {
        "arch" => context.has_requires() && enabled("arch"),
        other => enabled(other),
    }).map(str::to_string).collect()

}

pub fn write_pid ( path: &StdPath ) -> AppResult<()> {

    File::write(path, &std::process::id().to_string())

}

pub fn read_pid ( path: &StdPath ) -> Option<i32> {

    File::read_opt(path).and_then(|body| body.trim().parse().ok())

}
