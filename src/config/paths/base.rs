use std::path::{Path as StdPath, PathBuf};

use crate::config::base::consts::{
    ACTIVE_FILE, AUDIT_DIR, CACHE_DIR, CONFIGS_DIR, CONFIG_FILE, DOCS_DIR, DRAIN_FILE, GATE_LOG, INBOX_DIR,
    MANAGER_DIR, MD_EXT, PID_FILE, PROBES_DIR, REPORTS_DIR, REVIEW_SUFFIX, ROUNDS_DIR,
    SESSIONS_FILE, STATE_FILE, TASKS_DIR, TESTS_DIR,
};
use super::arch::Paths;

impl Paths {

    pub fn new ( root: &StdPath ) -> Self {

        let docs = root.join(DOCS_DIR);
        let cache = root.join(CACHE_DIR);
        let configs = cache.join(CONFIGS_DIR);
        let reports = cache.join(REPORTS_DIR);

        Self {
            root: root.to_path_buf(),

            config_file: root.join(CONFIG_FILE),

            state: configs.join(STATE_FILE),
            pid: configs.join(PID_FILE),
            active: configs.join(ACTIVE_FILE),
            sessions: configs.join(SESSIONS_FILE),
            drain: configs.join(DRAIN_FILE),
            gate_log: configs.join(GATE_LOG),

            inbox: cache.join(INBOX_DIR),
            tasks: cache.join(TASKS_DIR),
            audit: cache.join(AUDIT_DIR),
            manager: reports.join(MANAGER_DIR),
            rounds: cache.join(ROUNDS_DIR),
            tests: cache.join(TESTS_DIR),
            probes: cache.join(PROBES_DIR),
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

        self.reports.join(phase).join(format!("{agent}.{MD_EXT}"))

    }

    pub fn review_of ( &self, phase: &str ) -> PathBuf {

        self.manager.join(format!("{phase}{REVIEW_SUFFIX}"))

    }

    pub fn rounds_of ( &self, phase: &str ) -> PathBuf {

        self.rounds.join(phase)

    }

    pub fn task_rounds ( &self, task: &str ) -> PathBuf {

        self.rounds.join(TASKS_DIR).join(task)

    }

}
