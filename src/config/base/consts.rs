pub const VERSION: &str        = env!("CARGO_PKG_VERSION");
pub const TOOL: &str           = "agentx";
pub const DOCS_DIR: &str       = "agentx";
pub const CACHE_DIR: &str      = ".agentx";
pub const CONFIG_FILE: &str    = "Agentx.toml";

pub const CONVERGENCE: &str    = "ship it";

pub const CONFIGS_DIR: &str    = "configs";
pub const REPORTS_DIR: &str    = "reports";
pub const MANAGER_DIR: &str    = "manager";
pub const INBOX_DIR: &str      = "requires";
pub const TASKS_DIR: &str      = "tasks";
pub const AUDIT_DIR: &str      = "audits";
pub const ROUNDS_DIR: &str     = "rounds";
pub const MANIFESTS_DIR: &str  = "manifests";
pub const BASE_DIR: &str       = "base";
pub const PROJECT_DIR: &str    = "project";
pub const HISTORY_DIR: &str    = "history";
pub const NODE_FILE: &str      = "config.json";

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
pub const REVIEW_SUFFIX: &str  = "-review.md";
pub const CONSULT_FILE: &str   = "agentx-consult";

pub const PHASES: [&str; 7]          = ["requires", "tasks", "audits", "tests", "benches", "examples", "fuzzes"];
pub const FRAMES: [&str; 10]         = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const FRAMES_PLAIN: [&str; 4]    = ["|", "/", "-", "\\"];
pub const CONTEXT_BUCKETS: [&str; 6] = ["overview", "contracts", "skills", "designs", "references", "history"];

pub const PAD_TOP: usize             = 1;
pub const PAD_BOTTOM: usize          = 0;
pub const TICK_MS: u64               = 90;
pub const RULE_WIDTH: usize          = 68;
pub const BAR_WIDTH: usize           = 18;
pub const LOG_TAIL: usize            = 12;
pub const BUSY_WIDTH: usize          = 56;

pub const GLYPH_OK: (&str, &str)     = ( "✅", "[+]" );
pub const GLYPH_FAIL: (&str, &str)   = ( "❌", "[x]" );
pub const GLYPH_WARN: (&str, &str)   = ( "⚠️", "[!]" );
pub const GLYPH_INFO: (&str, &str)   = ( "🔸", "[*]" );
pub const GLYPH_STEP: (&str, &str)   = ( "👉", "[>]" );
pub const GLYPH_BEAT: (&str, &str)   = ( "🔄", "[~]" );
pub const GLYPH_COOL: (&str, &str)   = ( "😎", "[B]" );
pub const GLYPH_RAGE: (&str, &str)   = ( "😡", "[#]" );
pub const GLYPH_THINK: (&str, &str)  = ( "🤔", "[?]" );
pub const GLYPH_PARTY: (&str, &str)  = ( "🥳", "[o]" );
pub const GLYPH_STUDY: (&str, &str)  = ( "🔥", "[s]" );
pub const GLYPH_TITLE: (&str, &str)  = ( "✨", "**" );
pub const GLYPH_ON: (&str, &str)     = ( "🟢", "(+)" );
pub const GLYPH_OFF: (&str, &str)    = ( "⚪", "( )" );
pub const GLYPH_RULE: (&str, &str)   = ( "─", "-" );
pub const GLYPH_FULL: (&str, &str)   = ( "█", "#" );
pub const GLYPH_REST: (&str, &str)   = ( "░", "-" );

pub const BUCKET_STEMS: [(&str, &[&str]); 6] = [
    ( "overview",   &["overview", "overviews", "agent", "agents", "agentx", "codex", "claude"] ),
    ( "contracts",  &["contract", "contracts", "instruction", "instructions"] ),
    ( "skills",     &["skill", "skills"] ),
    ( "designs",    &["design", "designs"] ),
    ( "references", &["reference", "references"] ),
    ( "requires",   &["require", "requires", "requirement", "requirements"] ),
];

pub const BUCKET_DIRS: [(&str, &[&str]); 7] = [
    ( "overview",   &["overview", "overviews"] ),
    ( "contracts",  &["contract", "contracts", "instruction", "instructions"] ),
    ( "skills",     &["skill", "skills"] ),
    ( "designs",    &["design", "designs"] ),
    ( "references", &["reference", "references"] ),
    ( "history",    &["history", "histories"] ),
    ( "requires",   &["require", "requires", "requirement", "requirements"] ),
];

pub const MAX_AUDITS: u32      = 3;
pub const MAX_ROUNDS: u32      = 3;
pub const MAX_FIXES: u32       = 3;
pub const AGENT_RETRIES: u32   = 2;
pub const BACKOFF_SHIFT: u32   = 4;
pub const BACKOFF_CAP: u64     = 15;
pub const POLL_MS: u64         = 100;
pub const POLL_TICKS: u32      = 10;

pub const PROBE_TIMEOUT: u64   = 15;
pub const TURN_TIMEOUT: u64    = 30;
pub const GATE_TIMEOUT: u64    = 1000;
pub const AGENT_TIMEOUT: u64   = 10000;

pub const MANAGER_MODEL: &str  = "claude";
pub const DEFAULT_MODEL: &str  = "claude";

pub const CLAUDE_MODEL: &str   = "claude-opus-4-8";
pub const CLAUDE_EFFORT: &str  = "max";

pub const CODEX_MODEL: &str    = "gpt-5.5";
pub const CODEX_EFFORT: &str   = "high";
