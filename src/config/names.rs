//! Fixed names, tokens, and discovery tables.

pub const CONFIG_FILE: &str = "Agentx.toml";
pub const DOCS_DIR: &str = "agents";
pub const CACHE_DIR: &str = ".agentx";
pub const CONFIG_SECTION: &str = "project";

/// The exact final line an agent writes to signal convergence.
pub const CONVERGENCE: &str = "ship it";

/// The pipeline steps, in order.
pub const STEPS: [&str; 3] = ["arch", "work", "test"];

/// Durable context buckets under `agents/`.
pub const BUCKETS: [&str; 5] = ["overview", "contracts", "history", "tasks", "requires"];

/// Root-level files adopted as `overview` when there is no `agents/` directory.
pub const ROOT_FALLBACK_FILES: [&str; 3] = ["agents.md", "claude.md", "codex.md"];

/// The role label injected into each step's prompts.
pub fn role_of ( step: &str ) -> &'static str {

    match step {
        "arch" => "ARCHITECT",
        "work" => "EXECUTOR",
        "test" => "VERIFIER",
        _ => "AGENT",
    }

}

/// File stems / directory names that map a document into a bucket.
pub const DISCOVERY_STEMS: [(&str, &[&str]); 5] = [
    ( "overview",  &["overview", "workflow", "index", "agent", "agents"] ),
    ( "contracts", &["contract", "contracts", "style", "styles", "instruction", "instructions"] ),
    ( "history",   &["history", "decision", "decisions"] ),
    ( "tasks",     &["task", "tasks"] ),
    ( "requires",  &["require", "requires", "requirement", "requirements"] ),
];

/// Resolve a stem or directory name to its bucket, if any.
pub fn bucket_of ( stem: &str ) -> Option<&'static str> {

    DISCOVERY_STEMS.iter().find_map(|( bucket, stems )| stems.contains(&stem).then_some(*bucket))

}
