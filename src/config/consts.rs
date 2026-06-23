pub const CONFIG_FILE: &str = "Agentx.toml";
pub const DOCS_DIR: &str    = "agents";
pub const CACHE_DIR: &str   = ".agentx";
pub const CONVERGENCE: &str = "ship it";

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

pub const MAX_ROUNDS: u32     = 5;
pub const MAX_FIXES: u32      = 5;
pub const GATE_TIMEOUT: u64   = 900;
pub const MANAGER_MODEL: &str = "claude";
pub const DEFAULT_MODEL: &str = "claude";

pub const DEFAULT_TOML: &str = r#"[project]
project_type     = ""
max_rounds       = 5
max_fixes        = 5
gate_cmd         = ""
gate_timeout     = 900
manager_model    = "claude"
architect_models = [ "claude" ]
executor_models  = [ "claude" ]
tester_models    = [ "claude" ]
"#;

pub const CACHE_GITIGNORE: &str = "*\n";
