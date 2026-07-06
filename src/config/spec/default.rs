use super::arch::{Agent, Gate, Member, Options, Seats, Spec};
use crate::config::base::consts::{AGENT_TIMEOUT, DEFAULT_MODEL, DEFAULT_STAGE, GATE_TIMEOUT, MANAGER_MODEL, MAX_AUDITS, MAX_FIXES, MAX_ROUNDS};

impl Default for Spec {

    fn default () -> Self {

        Self {
            inspire: String::new(),
            stage: DEFAULT_STAGE.to_string(),
            description: String::new(),
            ignore: Vec::new(),
            include: Vec::new(),
        }

    }

}

impl Default for Options {

    fn default () -> Self {

        Self {
            lint: false, format: false, audits: false, tests: false, fuzzes: false, benches: false,
            examples: false, comments: false, doc_blocks: false, doc_contracts: false,
            train: true, clear: true,
        }

    }

}

impl Options {

    pub fn active ( &self, phase: &str ) -> bool {

        match phase {
            "requires" | "tasks" => true,
            "audits"   => self.audits,
            "tests"    => self.tests,
            "benches"  => self.benches,
            "examples" => self.examples,
            "fuzzes"   => self.fuzzes,
            _          => false,
        }

    }

}

impl Default for Gate {

    fn default () -> Self {

        Self { timeout: GATE_TIMEOUT, command: String::new() }

    }

}

impl Default for Agent {

    fn default () -> Self {

        let bare = || Seats { members: vec![Member::backend(DEFAULT_MODEL)], seq: true };

        Self {
            max_audits: MAX_AUDITS,
            max_rounds: MAX_ROUNDS,
            max_fixes: MAX_FIXES,
            timeout: AGENT_TIMEOUT,
            manager: Member::backend(MANAGER_MODEL),
            requires: bare(),
            tasks: bare(),
            audits: bare(),
            tests: bare(),
            fuzzes: bare(),
            benches: bare(),
            examples: bare(),
        }

    }

}
