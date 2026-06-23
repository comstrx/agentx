use std::path::{Path as StdPath, PathBuf};

use super::arch::Paths;
use super::consts::{CACHE_DIR, CONFIG_FILE, DOCS_DIR};

impl Paths {

    pub fn new ( root: &StdPath ) -> Self {

        let docs = root.join(DOCS_DIR);
        let cache = root.join(CACHE_DIR);
        let reports = cache.join("reports");

        Self {
            root: root.to_path_buf(),

            overview: docs.join("overview.md"),
            contracts: docs.join("contracts"),
            skills: docs.join("skills"),
            requires: docs.join("requires"),

            config_file: root.join(CONFIG_FILE),
            gitignore: cache.join(".gitignore"),

            state: cache.join("state.json"),
            pid: cache.join("agentx.pid"),
            active: cache.join("active.pid"),
            sessions: cache.join("sessions.json"),
            drain: cache.join("drain"),
            gate_log: cache.join("gate.log"),

            inbox: cache.join("requires"),
            tasks: cache.join("tasks"),
            manager: reports.join("manager"),
            rounds: cache.join("rounds"),
            tests: cache.join("tests"),
            probes: cache.join("probes"),
            prompts: cache.join("prompts"),
            reports,

            docs,
            cache,
        }

    }

    pub fn reports_of ( &self, phase: &str ) -> PathBuf {

        self.reports.join(phase)

    }

    pub fn report_of ( &self, phase: &str, agent: &str ) -> PathBuf {

        self.reports.join(phase).join(format!("{agent}.md"))

    }

    pub fn review_of ( &self, phase: &str ) -> PathBuf {

        self.manager.join(format!("{phase}-review.md"))

    }

    pub fn summary ( &self ) -> PathBuf {

        self.manager.join("summary.md")

    }

    pub fn rounds_of ( &self, phase: &str ) -> PathBuf {

        self.rounds.join(phase)

    }

    pub fn task_rounds ( &self, task: &str ) -> PathBuf {

        self.rounds.join("tasks").join(task)

    }

}
