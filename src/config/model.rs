use std::path::PathBuf;

use super::arch::{Config, Context, Document, Engine};
use super::consts::{BUCKETS, BUCKET_DIRS, BUCKET_STEMS, CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL};

impl Config {

    pub fn manager ( &self ) -> &str {

        &self.agent.manager

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        self.agent.roster(phase)

    }

    pub fn engine ( &self, agent: &str ) -> ( String, String ) {

        match agent.starts_with("codex") {
            true  => self.codex.resolved(CODEX_MODEL, CODEX_EFFORT),
            false => self.claude.resolved(CLAUDE_MODEL, CLAUDE_EFFORT),
        }

    }

}

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

    pub fn engines ( &self ) -> ( ( String, String ), ( String, String ) ) {

        ( self.claude.resolved(CLAUDE_MODEL, CLAUDE_EFFORT), self.codex.resolved(CODEX_MODEL, CODEX_EFFORT) )

    }

}

impl Context {

    pub fn bucket ( &self, name: &str ) -> &[PathBuf] {

        match name {
            "overview" => &self.overview,
            "contracts" => &self.contracts,
            "skills" => &self.skills,
            "history" => &self.history,
            "requires" => &self.requires,
            _ => &[],
        }

    }

    pub fn add ( &mut self, name: &str, path: PathBuf ) {

        let target = match name {
            "overview" => &mut self.overview,
            "contracts" => &mut self.contracts,
            "skills" => &mut self.skills,
            "history" => &mut self.history,
            "requires" => &mut self.requires,
            _ => return,
        };

        if !target.contains(&path) { target.push(path); }

    }

    pub fn buckets_of_stem ( stem: &str ) -> Vec<&'static str> {

        BUCKET_STEMS.iter().filter_map(|( bucket, stems )| stems.contains(&stem).then_some(*bucket)).collect()

    }

    pub fn bucket_of_dir ( name: &str ) -> Option<&'static str> {

        BUCKET_DIRS.iter().find_map(|( bucket, names )| names.contains(&name).then_some(*bucket))

    }

    pub fn has_requires ( &self ) -> bool {

        !self.requires.is_empty()

    }

    pub fn is_empty ( &self ) -> bool {

        BUCKETS.iter().all(|name| self.bucket(name).is_empty())

    }

}
