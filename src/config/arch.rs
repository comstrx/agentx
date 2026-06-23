use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Spec {
    pub project_type: String,
    pub max_rounds: u32,
    pub max_fixes: u32,
    pub gate_cmd: String,
    pub gate_timeout: u64,
    pub manager_model: String,
    pub architect_models: Vec<String>,
    pub executor_models: Vec<String>,
    pub tester_models: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    #[serde(default)]
    pub project: Spec,
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub root: PathBuf,
    pub docs: PathBuf,
    pub cache: PathBuf,

    pub overview: PathBuf,
    pub contracts: PathBuf,
    pub skills: PathBuf,
    pub requires: PathBuf,

    pub config_file: PathBuf,
    pub gitignore: PathBuf,

    pub state: PathBuf,
    pub pid: PathBuf,
    pub active: PathBuf,
    pub sessions: PathBuf,
    pub drain: PathBuf,
    pub gate_log: PathBuf,

    pub inbox: PathBuf,
    pub tasks: PathBuf,
    pub reports: PathBuf,
    pub manager: PathBuf,
    pub rounds: PathBuf,
    pub tests: PathBuf,
    pub probes: PathBuf,
    pub prompts: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub overview: Vec<PathBuf>,
    pub contracts: Vec<PathBuf>,
    pub skills: Vec<PathBuf>,
    pub history: Vec<PathBuf>,
    pub requires: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub root: PathBuf,
    pub spec: Spec,
    pub paths: Paths,
    pub context: Context,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Train;
