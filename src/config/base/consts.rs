pub const TOOL: &str           = "agentx";
pub const VERSION: &str        = env!("CARGO_PKG_VERSION");

pub const DOCS_DIR: &str       = "agentx";
pub const CACHE_DIR: &str      = ".agentx";
pub const CONFIG_FILE: &str    = "Agentx.toml";

pub const CONVERGENCE: &str    = "ship it";

pub const CONFIGS_DIR: &str    = "configs";
pub const REPORTS_DIR: &str    = "reports";
pub const MANAGER_DIR: &str    = "manager";
pub const INBOX_DIR: &str      = "requires";
pub const TASKS_DIR: &str      = "tasks";
pub const AUDIT_DIR: &str      = "audit";
pub const ROUNDS_DIR: &str     = "rounds";
pub const TESTS_DIR: &str      = "tests";
pub const PROBES_DIR: &str     = "probes";
pub const MANIFESTS_DIR: &str  = "manifests";
pub const TRAIN_DIR: &str      = "train";

pub const STATE_FILE: &str     = "state.json";
pub const PID_FILE: &str       = "agentx.pid";
pub const ACTIVE_FILE: &str    = "active.pid";
pub const SESSIONS_FILE: &str  = "sessions.json";
pub const DRAIN_FILE: &str     = "drain";
pub const GATE_LOG: &str       = "gate.log";
pub const RUN_LOG: &str        = "run.log";

pub const OVERVIEW: &str       = "overview";
pub const CONTRACTS: &str      = "contracts";
pub const SKILLS: &str         = "skills";
pub const DESIGNS: &str        = "designs";
pub const HISTORY: &str        = "history";
pub const REQUIRES: &str       = "requires";
pub const AUDITS: &str         = "audits";

pub const MD_EXT: &str         = "md";
pub const ABOUT_FILE: &str     = "about.md";
pub const REVIEW_SUFFIX: &str  = "-review.md";
pub const CONSULT_FILE: &str   = "agentx-consult";

pub const PHASES: [&str; 7]          = ["requires", "tasks", "audits", "tests", "benches", "examples", "fuzzes"];
pub const FRAMES: [&str; 10]         = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const CONTEXT_BUCKETS: [&str; 5] = ["overview", "contracts", "skills", "designs", "history"];

pub const BUCKET_STEMS: [(&str, &[&str]); 5] = [
    ( "overview",  &["agentx", "agents", "agent", "overview", "workflow", "index", "claude", "codex"] ),
    ( "contracts", &["agentx", "overview", "contract", "contracts", "style", "styles", "instruction", "instructions"] ),
    ( "skills",    &["skill", "skills"] ),
    ( "designs",   &["design", "designs"] ),
    ( "requires",  &["require", "requires", "requirement", "requirements"] ),
];

pub const BUCKET_DIRS: [(&str, &[&str]); 5] = [
    ( "overview",  &["overview"] ),
    ( "contracts", &["contracts", "contract", "overview"] ),
    ( "skills",    &["skills", "skill"] ),
    ( "designs",   &["designs", "design"] ),
    ( "requires",  &["requires", "require", "requirements", "requirement"] ),
];

pub const MAX_AUDITS: u32      = 3;
pub const MAX_ROUNDS: u32      = 3;
pub const MAX_FIXES: u32       = 3;
pub const AGENT_RETRIES: u32   = 2;
pub const PROBE_TIMEOUT: u64   = 15;
pub const GATE_TIMEOUT: u64    = 1000;
pub const AGENT_TIMEOUT: u64   = 10000;

pub const MANAGER_MODEL: &str  = "claude";
pub const DEFAULT_MODEL: &str  = "claude";
pub const CLAUDE_MODEL: &str   = "opus";
pub const CLAUDE_EFFORT: &str  = "max";
pub const CODEX_MODEL: &str    = "gpt-5-codex";
pub const CODEX_EFFORT: &str   = "high";

