use std::collections::HashMap;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::fs::File;
use crate::core::parse::Toml;
use super::arch::{Agent, Document, Gate, Member, Options, Seats, Spec};
use crate::config::base::consts::{
    AGENT_TIMEOUT, CLAUDE_EFFORT, CLAUDE_MODEL, CODEX_EFFORT, CODEX_MODEL, DEFAULT_MODEL,
    GATE_TIMEOUT, MANAGER_MODEL, MAX_AUDITS, MAX_FIXES, MAX_ROUNDS,
};

impl Spec {

    pub(crate) fn default_toml () -> String {

        let o = Options::default();
        let a = Agent::default();
        let b = |value: bool| if value { "true" } else { "false" };

        let mut engines = Document::default();
        engines.fill_defaults();

        let member = |seat: &Member| {

            let ( model, effort ) = engines.resolve_member(seat);

            format!("{{ agent = \"{}\", model = \"{model}\", effort = \"{effort}\" }}", seat.agent)

        };

        let roster = |seats: &Seats| seats.members.iter().map(&member).collect::<Vec<_>>().join(", ");

        format!(
"[project]
inspire       = \"\"
description   = \"\"

[option]
lint          = {lint}
format        = {format}
audits        = {audits}
tests         = {tests}
fuzzes        = {fuzzes}
benches       = {benches}
examples      = {examples}
comments      = {comments}
doc_blocks    = {doc_blocks}
doc_contracts = {doc_contracts}
train         = {train}
clear         = {clear}

[gate]
timeout = {GATE_TIMEOUT}
command = \"\"

[claude]
model  = \"{CLAUDE_MODEL}\"
effort = \"{CLAUDE_EFFORT}\"

[codex]
model  = \"{CODEX_MODEL}\"
effort = \"{CODEX_EFFORT}\"

[agent]
max_audits = {MAX_AUDITS}
max_rounds = {MAX_ROUNDS}
max_fixes  = {MAX_FIXES}
timeout    = {AGENT_TIMEOUT}
manager    = {manager}
requires   = [ {r_requires} ]
tasks      = [ {r_tasks} ]
audits     = [ {r_audits} ]
tests      = [ {r_tests} ]
fuzzes     = [ {r_fuzzes} ]
benches    = [ {r_benches} ]
examples   = [ {r_examples} ]
",
            lint = b(o.lint),
            format = b(o.format),
            audits = b(o.audits),
            tests = b(o.tests),
            fuzzes = b(o.fuzzes),
            benches = b(o.benches),
            examples = b(o.examples),
            comments = b(o.comments),
            doc_blocks = b(o.doc_blocks),
            doc_contracts = b(o.doc_contracts),
            train = b(o.train),
            clear = b(o.clear),
            manager = member(&a.manager),
            r_requires = roster(&a.requires),
            r_tasks = roster(&a.tasks),
            r_audits = roster(&a.audits),
            r_tests = roster(&a.tests),
            r_fuzzes = roster(&a.fuzzes),
            r_benches = roster(&a.benches),
            r_examples = roster(&a.examples),
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
        self.manager.agent = self.manager.agent.trim().to_string();

        if self.manager.agent.is_empty() { self.manager = Member::backend(MANAGER_MODEL); }

        for seats in [&mut self.requires, &mut self.tasks, &mut self.audits, &mut self.tests, &mut self.fuzzes, &mut self.benches, &mut self.examples] {

            seats.members.retain(|member| !member.agent.trim().is_empty());

            if seats.members.is_empty() { seats.members.push(Member::backend(DEFAULT_MODEL)); }

        }

        self

    }

}

impl Document {

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        File::write_atomic(config_file, &Toml::to_string_pretty(self)?)

    }

}
