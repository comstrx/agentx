use std::path::{Path as StdPath, PathBuf};

use super::arch::Paths;
use super::consts::{CACHE_DIR, CONFIG_FILE, DOCS_DIRS};

impl Paths {

    pub fn new ( root: &StdPath ) -> Self {

        let docs: Vec<PathBuf> = DOCS_DIRS.iter().map(|name| root.join(name)).collect();
        let cache = root.join(CACHE_DIR);
        let configs = cache.join("configs");
        let reports = cache.join("reports");

        Self {
            root: root.to_path_buf(),

            config_file: root.join(CONFIG_FILE),

            state: configs.join("state.json"),
            pid: configs.join("agentx.pid"),
            active: configs.join("active.pid"),
            sessions: configs.join("sessions.json"),
            drain: configs.join("drain"),
            gate_log: configs.join("gate.log"),

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
            configs,
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
