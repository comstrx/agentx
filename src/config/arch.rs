use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Spec {
    pub inspire: String,
    #[serde(deserialize_with = "Spec::de_tests")]
    pub tests: bool,
    pub max_rounds: u32,
    pub max_fixes: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

pub(crate) struct TestsFlag;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Gate {
    pub timeout: u64,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Agent {
    pub timeout: u64,
    pub manager: String,
    pub architects: Vec<String>,
    pub executors: Vec<String>,
    pub testers: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    #[serde(default)]
    pub project: Spec,
    #[serde(default)]
    pub gate: Gate,
    #[serde(default)]
    pub agent: Agent,
    #[serde(default)]
    pub claude: Engine,
    #[serde(default)]
    pub codex: Engine,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Engine {
    pub model: String,
    pub effort: String,
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub root: PathBuf,
    pub docs: Vec<PathBuf>,
    pub cache: PathBuf,
    pub configs: PathBuf,

    pub config_file: PathBuf,

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
    pub gate: Gate,
    pub agent: Agent,
    pub paths: Paths,
    pub context: Context,
    pub claude: Engine,
    pub codex: Engine,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Train;
