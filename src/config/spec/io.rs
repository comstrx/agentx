use std::collections::HashMap;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::fs::File;
use crate::core::parse::Toml;
use super::arch::{Agent, Document, Gate, Spec};
use crate::config::base::consts::{
    AGENT_TIMEOUT, CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL, DEFAULT_MODEL,
    GATE_TIMEOUT, MANAGER_MODEL, MAX_AUDITS, MAX_FIXES, MAX_ROUNDS,
};

impl Spec {

    pub(crate) fn default_toml () -> String {

        let one = DEFAULT_MODEL;

        format!(
"[project]
inspire       = \"\"
description   = \"\"

[option]
lint          = false
format        = false
audits        = false
tests         = false
fuzzes        = false
benches       = false
examples      = false
comments      = false
doc_blocks    = false
doc_contracts = false
train         = true
clear         = true

[gate]
timeout = {GATE_TIMEOUT}
command = \"\"

[agent]
max_audits = {MAX_AUDITS}
max_rounds = {MAX_ROUNDS}
max_fixes  = {MAX_FIXES}
timeout    = {AGENT_TIMEOUT}
manager    = \"{MANAGER_MODEL}\"
requires   = [ \"{one}\", \"codex\", \"{one}\" ]
tasks      = [ \"{one}\" ]
audits     = [ \"{one}\" ]
tests      = [ \"{one}\" ]
fuzzes     = [ \"{one}\" ]
benches    = [ \"{one}\" ]
examples   = [ \"{one}\" ]

[claude]
model  = \"{CLAUDE_MODEL}\"
effort = \"{CLAUDE_EFFORT}\"

[codex]
model  = \"{CODEX_MODEL}\"
effort = \"{CODEX_EFFORT}\"
"
        )

    }

    pub fn load ( config_file: &StdPath ) -> AppResult<Self> {

        Ok(Self::document(config_file)?.project)

    }

    pub fn document ( config_file: &StdPath ) -> AppResult<Document> {

        let body = File::read(config_file);

        if body.trim().is_empty() { return Ok(Document::default()); }

        let mut document: Document = Toml::parse(&body)?;
        document.project = document.project.sanitized();
        document.gate = document.gate.sanitized();
        document.agent = document.agent.sanitized();

        Ok(document)

    }

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        let body = File::read(config_file);

        let mut document: Document = if body.trim().is_empty() { Document::default() } else { Toml::parse(&body)? };
        document.project = self.clone();

        File::write_atomic(config_file, &Toml::to_string_pretty(&document)?)

    }

    fn sanitized ( mut self ) -> Self {

        for paths in [&mut self.ignore, &mut self.include] {

            let mut seen = HashMap::new();
            paths.retain(|path| !path.trim().is_empty() && seen.insert(path.trim().to_string(), ()).is_none());

        }

        self

    }

}

impl Gate {

    fn sanitized ( mut self ) -> Self {

        self.command = self.command.trim().to_string();

        self

    }

}

impl Agent {

    fn sanitized ( mut self ) -> Self {

        self.max_audits = self.max_audits.max(1);
        self.max_rounds = self.max_rounds.max(1);
        self.manager = self.manager.trim().to_string();

        if self.manager.is_empty() { self.manager = MANAGER_MODEL.to_string(); }

        for models in [&mut self.requires, &mut self.tasks, &mut self.audits, &mut self.tests, &mut self.fuzzes, &mut self.benches, &mut self.examples] {

            models.retain(|model| !model.trim().is_empty());

            if models.is_empty() { models.push(DEFAULT_MODEL.to_string()); }

        }

        self

    }

}

impl Document {

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        File::write_atomic(config_file, &Toml::to_string_pretty(self)?)

    }

}
