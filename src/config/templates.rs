//! Scaffolding payloads written by `init`.

/// The `Agentx.toml` created on `init` when none exists.
pub const DEFAULT_TOML: &str = r#"[project]
max_rounds    = 5
max_fixes     = 5
gate_cmd      = ""
gate_timeout  = 900
manager_model = "claude"
steps         = [ "arch", "work", "test" ]
arch_models   = [ "claude" ]
work_models   = [ "claude" ]
test_models   = [ "claude" ]
"#;

/// The `.agentx/.gitignore` body — the whole cache is scratch.
pub const CACHE_GITIGNORE: &str = "*\n";
