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

#[derive(Debug, Clone, Default)]
pub struct Member {
    pub agent: String,
    pub model: String,
    pub effort: String,
}

#[derive(Debug, Clone, Default)]
pub struct Seats {
    pub members: Vec<Member>,
    pub seq: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Agent {
    pub max_audits: u32,
    pub max_rounds: u32,
    pub max_fixes: u32,
    pub timeout: u64,
    #[serde(deserialize_with = "Spec::de_manager")]
    pub manager: Member,
    pub requires: Seats,
    pub tasks: Seats,
    pub audits: Seats,
    pub tests: Seats,
    pub fuzzes: Seats,
    pub benches: Seats,
    pub examples: Seats,
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
    pub claude: Engine,
    #[serde(default)]
    pub codex: Engine,
    #[serde(default)]
    pub agent: Agent,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Engine {
    pub model: String,
    pub effort: String,
}
