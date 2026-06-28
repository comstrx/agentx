use crate::config::base::consts::{CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL};
use crate::config::worker::Worker;
use super::arch::Config;

impl Config {

    pub fn manager ( &self ) -> &str {

        &self.agent.manager

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        self.agent.roster(phase)

    }

    pub fn engine ( &self, agent: &str ) -> ( String, String ) {

        match Worker::resolve(agent) {
            Some("codex") => self.codex.resolved(CODEX_MODEL, CODEX_EFFORT),
            _             => self.claude.resolved(CLAUDE_MODEL, CLAUDE_EFFORT),
        }

    }

}
