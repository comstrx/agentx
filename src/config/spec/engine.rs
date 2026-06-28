use crate::config::base::consts::{CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL};
use super::arch::{Document, Engine};

impl Engine {

    pub fn resolved ( &self, model: &str, effort: &str ) -> ( String, String ) {

        let resolved_model = if self.model.trim().is_empty() { model } else { self.model.trim() };
        let resolved_effort = if self.effort.trim().is_empty() { effort } else { self.effort.trim() };

        ( resolved_model.to_string(), resolved_effort.to_string() )

    }

    pub fn fill ( &mut self, model: &str, effort: &str ) -> bool {

        let mut dirty = false;

        if self.model.trim().is_empty() { self.model = model.to_string(); dirty = true; }

        if self.effort.trim().is_empty() { self.effort = effort.to_string(); dirty = true; }

        dirty

    }

}

impl Document {

    pub fn fill_defaults ( &mut self ) -> bool {

        let claude = self.claude.fill(CLAUDE_MODEL, CLAUDE_EFFORT);
        let codex = self.codex.fill(CODEX_MODEL, CODEX_EFFORT);

        claude || codex

    }

    pub fn engine_of ( &self, backend: &str ) -> ( String, String ) {

        match backend {
            "codex" => self.codex.resolved(CODEX_MODEL, CODEX_EFFORT),
            _       => self.claude.resolved(CLAUDE_MODEL, CLAUDE_EFFORT),
        }

    }

}
