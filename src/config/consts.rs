pub const CONVERGENCE: &str    = "ship it";

pub const CACHE_DIR: &str      = ".agentx";
pub const CONFIG_FILE: &str    = "Agentx.toml";
pub const DOCS_DIRS: [&str; 2] = ["agentx", "agents"];

pub const PHASES: [&str; 3]  = ["requires", "tasks", "tests"];
pub const BUCKETS: [&str; 5] = ["overview", "contracts", "skills", "history", "requires"];

pub const BUCKET_TITLES: [(&str, &str); 5] = [
    ( "overview",  "Overview / workflow" ),
    ( "contracts", "Contracts (LAW - they override everything)" ),
    ( "skills",    "Skills / proven know-how" ),
    ( "history",   "History / past decisions" ),
    ( "requires",  "Requirements to build" ),
];

pub const BUCKET_STEMS: [(&str, &[&str]); 4] = [
    ( "overview",  &["agentx", "agents", "agent", "overview", "workflow", "index", "claude", "codex"] ),
    ( "contracts", &["agentx", "contract", "contracts", "style", "styles", "instruction", "instructions"] ),
    ( "skills",    &["skill", "skills"] ),
    ( "requires",  &["require", "requires", "requirement", "requirements"] ),
];

pub const BUCKET_DIRS: [(&str, &[&str]); 4] = [
    ( "overview",  &["overview"] ),
    ( "contracts", &["contracts", "contract"] ),
    ( "skills",    &["skills", "skill"] ),
    ( "requires",  &["requires", "require", "requirements", "requirement"] ),
];

pub const CONTEXT_BUCKETS: [&str; 4] = ["overview", "contracts", "skills", "history"];

pub const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub const MAX_ROUNDS: u32      = 5;
pub const MAX_FIXES: u32       = 5;
pub const AGENT_RETRIES: u32   = 2;
pub const GATE_TIMEOUT: u64    = 900;
pub const AGENT_TIMEOUT: u64   = 10000;
pub const CONSULT_TIMEOUT: u64 = 600;
pub const PROBE_TIMEOUT: u64   = 15;

pub const MANAGER_MODEL: &str  = "claude";
pub const DEFAULT_MODEL: &str  = "claude";

pub const CLAUDE_MODEL: &str  = "opus";
pub const CLAUDE_EFFORT: &str = "max";
pub const CODEX_MODEL: &str   = "gpt-5-codex";
pub const CODEX_EFFORT: &str  = "high";

pub const DEFAULT_TOML: &str = r#"[project]
inspire    = ""
tests      = true
max_rounds = 5
max_fixes  = 5

[gate]
timeout = 900
command = ""

[agent]
timeout    = 10000
manager    = "claude"
architects = [ "claude" ]
executors  = [ "claude" ]
testers    = [ "claude" ]

[claude]
model  = "opus"
effort = "max"

[codex]
model  = "gpt-5-codex"
effort = "high"
"#;
