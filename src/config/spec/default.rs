use super::arch::{Agent, Gate, Options};
use crate::config::base::consts::{AGENT_TIMEOUT, DEFAULT_MODEL, GATE_TIMEOUT, MANAGER_MODEL, MAX_AUDITS, MAX_FIXES, MAX_ROUNDS};

impl Default for Options {

    fn default () -> Self {

        Self {
            lint: false, format: false, audits: false, tests: false, fuzzes: false, benches: false,
            examples: false, comments: false, doc_blocks: false, doc_contracts: false,
            train: true, clear: true,
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

        let one = || vec![DEFAULT_MODEL.to_string()];

        Self {
            max_audits: MAX_AUDITS,
            max_rounds: MAX_ROUNDS,
            max_fixes: MAX_FIXES,
            timeout: AGENT_TIMEOUT,
            manager: MANAGER_MODEL.to_string(),
            requires: one(),
            tasks: one(),
            audits: one(),
            tests: one(),
            fuzzes: one(),
            benches: one(),
            examples: one(),
        }

    }

}
