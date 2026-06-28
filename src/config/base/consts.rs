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
pub const REFERENCES: &str     = "references";
pub const HISTORY: &str        = "history";
pub const REQUIRES: &str       = "requires";
pub const AUDITS: &str         = "audits";

pub const MD_EXT: &str         = "md";
pub const ABOUT_FILE: &str     = "about.md";
pub const REVIEW_SUFFIX: &str  = "-review.md";
pub const CONSULT_FILE: &str   = "agentx-consult";

pub const PHASES: [&str; 7]          = ["requires", "tasks", "audits", "tests", "benches", "examples", "fuzzes"];
pub const FRAMES: [&str; 10]         = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const CONTEXT_BUCKETS: [&str; 6] = ["overview", "contracts", "skills", "designs", "references", "history"];

pub const BUCKET_STEMS: [(&str, &[&str]); 6] = [
    ( "overview",   &["overview", "agent", "agents", "agentx", "codex", "claude"] ),
    ( "contracts",  &["contract", "contracts", "instruction", "instructions"] ),
    ( "skills",     &["skill", "skills"] ),
    ( "designs",    &["design", "designs"] ),
    ( "references", &["reference", "references"] ),
    ( "requires",   &["require", "requires", "requirement", "requirements"] ),
];

pub const BUCKET_DIRS: [(&str, &[&str]); 6] = [
    ( "overview",   &["overview"] ),
    ( "contracts",  &["contract", "contracts", "instruction", "instructions"] ),
    ( "skills",     &["skill", "skills"] ),
    ( "designs",    &["design", "designs"] ),
    ( "references", &["reference", "references"] ),
    ( "requires",   &["require", "requires", "requirement", "requirements"] ),
];

pub const MAX_AUDITS: u32      = 3;
pub const MAX_ROUNDS: u32      = 3;
pub const MAX_FIXES: u32       = 3;
pub const AGENT_RETRIES: u32   = 2;
pub const PROBE_TIMEOUT: u64   = 15;
pub const PROBE_TURN_TIMEOUT: u64 = 30;
pub const PROBE_PROMPT: &str   = "Reply with the single word: pong — nothing else. Do not use any tool and do not read or write any file.";
pub const GATE_TIMEOUT: u64    = 1000;
pub const AGENT_TIMEOUT: u64   = 10000;

pub const MANAGER_MODEL: &str  = "claude";
pub const DEFAULT_MODEL: &str  = "claude";
pub const CLAUDE_MODEL: &str   = "opus";
pub const CODEX_MODEL: &str    = "gpt-5.5";
pub const CLAUDE_EFFORT: &str  = "high";
pub const CODEX_EFFORT: &str   = "high";
