use std::path::{Path as StdPath, PathBuf};

use super::arch::Paths;
use super::names::{CACHE_DIR, CONFIG_FILE, DOCS_DIR};

impl Paths {

    /// Derive the full layout from a project root.
    pub fn new ( root: &StdPath ) -> Self {

        let docs = root.join(DOCS_DIR);
        let cache = root.join(CACHE_DIR);

        Self {
            root: root.to_path_buf(),

            overview: docs.join("overview.md"),
            contracts: docs.join("contracts"),
            history: docs.join("history"),
            tasks: docs.join("tasks"),
            requires: docs.join("requires"),

            reports: cache.join("reports"),
            rounds: cache.join("rounds"),
            tests: cache.join("tests"),
            probes: cache.join("probes"),
            prompts: cache.join("prompts"),
            runs: cache.join("runs"),

            review: cache.join("review.md"),
            control: cache.join("control.md"),
            gate_log: cache.join("gate.log"),
            pid: cache.join("agentx.pid"),
            active: cache.join("active.pid"),
            drain: cache.join("drain"),
            sessions: cache.join("sessions.json"),
            gitignore: cache.join(".gitignore"),
            config_file: root.join(CONFIG_FILE),

            docs,
            cache,
        }

    }

    /// Per-step reports directory, e.g. `.agentx/reports/arch`.
    pub fn reports_of ( &self, step: &str ) -> PathBuf {

        self.reports.join(step)

    }

    /// Per-step rounds directory, e.g. `.agentx/rounds/work`.
    pub fn rounds_of ( &self, step: &str ) -> PathBuf {

        self.rounds.join(step)

    }

}
