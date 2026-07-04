use crate::config::base::consts::{CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL};
use crate::config::worker::Worker;
use super::arch::{Document, Engine, Member};

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

impl Member {

    pub fn resolved ( &self, claude: &Engine, codex: &Engine ) -> ( String, String ) {

        let ( base_model, base_effort ) = match Worker::resolve(&self.agent) {
            Some("codex") => codex.resolved(CODEX_MODEL, CODEX_EFFORT),
            _             => claude.resolved(CLAUDE_MODEL, CLAUDE_EFFORT),
        };

        let model = if self.model.trim().is_empty() { base_model } else { self.model.trim().to_string() };
        let effort = if self.effort.trim().is_empty() { base_effort } else { self.effort.trim().to_string() };

        ( model, effort )

    }

}

impl Document {

    pub fn fill_defaults ( &mut self ) -> bool {

        let claude = self.claude.fill(CLAUDE_MODEL, CLAUDE_EFFORT);
        let codex = self.codex.fill(CODEX_MODEL, CODEX_EFFORT);

        claude || codex

    }

    pub fn resolve_member ( &self, member: &Member ) -> ( String, String ) {

        member.resolved(&self.claude, &self.codex)

    }

}
