use std::path::PathBuf;
use serde::Deserialize;

use super::names::BUCKETS;

/// The `[project]` table from `Agentx.toml`. Missing fields fall back to
/// [`Default`]; loading + sanitising lives in `spec.rs`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Spec {

    pub max_rounds: u32,
    pub max_fixes: u32,
    pub gate_cmd: String,
    pub gate_timeout: u64,
    pub manager_model: String,
    pub steps: Vec<String>,
    pub arch_models: Vec<String>,
    pub work_models: Vec<String>,
    pub test_models: Vec<String>,
}

/// Every path agentx owns, derived once from the project root (see `paths.rs`).
#[derive(Debug, Clone)]
pub struct Paths {

    pub root: PathBuf,
    pub docs: PathBuf,
    pub cache: PathBuf,

    pub overview: PathBuf,
    pub contracts: PathBuf,
    pub history: PathBuf,
    pub tasks: PathBuf,
    pub requires: PathBuf,

    pub reports: PathBuf,
    pub rounds: PathBuf,
    pub tests: PathBuf,
    pub probes: PathBuf,
    pub prompts: PathBuf,
    pub runs: PathBuf,

    pub review: PathBuf,
    pub control: PathBuf,
    pub gate_log: PathBuf,
    pub pid: PathBuf,
    pub active: PathBuf,
    pub drain: PathBuf,
    pub sessions: PathBuf,
    pub gitignore: PathBuf,
    pub config_file: PathBuf,
}

/// The discovered project context: durable docs bucketed by kind.
#[derive(Debug, Clone, Default)]
pub struct Context {

    pub overview: Vec<PathBuf>,
    pub contracts: Vec<PathBuf>,
    pub history: Vec<PathBuf>,
    pub tasks: Vec<PathBuf>,
    pub requires: Vec<PathBuf>,
}

/// The fully assembled run configuration handed to the orchestrator.
#[derive(Debug, Clone)]
pub struct Config {

    pub root: PathBuf,
    pub spec: Spec,
    pub paths: Paths,
    pub context: Context,
}

impl Context {

    pub fn bucket ( &self, name: &str ) -> &[PathBuf] {

        match name {
            "overview" => &self.overview,
            "contracts" => &self.contracts,
            "history" => &self.history,
            "tasks" => &self.tasks,
            "requires" => &self.requires,
            _ => &[],
        }

    }

    pub fn add ( &mut self, name: &str, path: PathBuf ) {

        let target = match name {
            "overview" => &mut self.overview,
            "contracts" => &mut self.contracts,
            "history" => &mut self.history,
            "tasks" => &mut self.tasks,
            "requires" => &mut self.requires,
            _ => return,
        };

        if !target.contains(&path) {
            target.push(path);
        }

    }

    pub fn has_requires ( &self ) -> bool {

        !self.requires.is_empty()

    }

    pub fn has_tasks ( &self ) -> bool {

        !self.tasks.is_empty()

    }

    pub fn is_empty ( &self ) -> bool {

        BUCKETS.iter().all(|name| self.bucket(name).is_empty())

    }

}

impl Config {

    pub fn manager ( &self ) -> &str {

        &self.spec.manager_model

    }

}
