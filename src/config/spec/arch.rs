use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Spec {
    pub inspire: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}

pub(crate) struct BoolFlag;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
    #[serde(deserialize_with = "Spec::de_bool")]
    pub lint: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub format: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub audits: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub tests: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub fuzzes: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub benches: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub examples: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub comments: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub doc_blocks: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub doc_contracts: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub train: bool,
    #[serde(deserialize_with = "Spec::de_bool")]
    pub clear: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Gate {
    pub timeout: u64,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Agent {
    pub max_audits: u32,
    pub max_rounds: u32,
    pub max_fixes: u32,
    pub timeout: u64,
    #[serde(deserialize_with = "Spec::de_manager")]
    pub manager: String,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub requires: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub tasks: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub audits: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub tests: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub fuzzes: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub benches: Vec<String>,
    #[serde(deserialize_with = "Spec::de_roster")]
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    #[serde(default)]
    pub project: Spec,
    #[serde(default)]
    pub option: Options,
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
