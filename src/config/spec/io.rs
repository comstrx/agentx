use std::collections::HashMap;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::fs::File;
use crate::core::parse::Toml;
use super::arch::{Agent, Document, Engine, Gate, Member, Seats, Spec};
use crate::config::base::consts::{DEFAULT_MODEL, DEFAULT_STAGE, MANAGER_MODEL};

impl Spec {

    pub(crate) fn default_toml () -> String {

        let mut document = Document::default();

        let mut engines = Document::default();
        engines.fill_defaults();

        let ( model, effort ) = engines.resolve_member(&document.agent.manager);
        document.agent.manager.model = model;
        document.agent.manager.effort = effort;

        for seats in [
            &mut document.agent.requires, &mut document.agent.tasks, &mut document.agent.audits,
            &mut document.agent.tests, &mut document.agent.fuzzes, &mut document.agent.benches,
            &mut document.agent.examples,
        ] {

            for member in &mut seats.members {

                let ( model, effort ) = engines.resolve_member(member);
                member.model = model;
                member.effort = effort;

            }

        }

        document.render()

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

        File::write_atomic(config_file, &document.render())

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

        File::write_atomic(config_file, &self.render())

    }

    pub fn render ( &self ) -> String {

        let b = |value: bool| if value { "true" } else { "false" };

        let member = |seat: &Member| {

            let agent = seat.agent.trim();
            let model = seat.model.trim();
            let effort = seat.effort.trim();

            if model.is_empty() && effort.is_empty() { return Toml::quote(agent); }

            let mut parts = vec![format!("agent = {}", Toml::quote(agent))];

            if !model.is_empty() { parts.push(format!("model = {}", Toml::quote(model))); }

            if !effort.is_empty() { parts.push(format!("effort = {}", Toml::quote(effort))); }

            format!("{{ {} }}", parts.join(", "))

        };

        let roster = |seats: &Seats| format!("[ {} ]", seats.members.iter().map(&member).collect::<Vec<_>>().join(", "));

        let list = |name: &str, values: &[String]| {

            if values.is_empty() { return String::new(); }

            format!("{name:<13} = [ {} ]\n", values.iter().map(|value| Toml::quote(value)).collect::<Vec<_>>().join(", "))

        };

        let engine = |name: &str, engine: &Engine| {

            if engine.model.trim().is_empty() && engine.effort.trim().is_empty() { return String::new(); }

            format!("\n[{name}]\nmodel  = {}\neffort = {}\n", Toml::quote(engine.model.trim()), Toml::quote(engine.effort.trim()))

        };

        let o = &self.option;

        format!(
"[project]
inspire       = {inspire}
stage         = {stage}
description   = {description}
{ignore}{include}
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
timeout = {gate_timeout}
command = {gate_command}
{claude}{codex}
[agent]
max_audits = {max_audits}
max_rounds = {max_rounds}
max_fixes  = {max_fixes}
timeout    = {agent_timeout}
manager    = {manager}
requires   = {requires}
tasks      = {tasks}
audits     = {r_audits}
tests      = {r_tests}
fuzzes     = {fuzzes_r}
benches    = {benches_r}
examples   = {examples_r}
",
            inspire = Toml::quote(self.project.inspire.trim()),
            stage = Toml::quote(match self.project.stage.trim() { "" => DEFAULT_STAGE, other => other }),
            description = Toml::quote(self.project.description.trim()),
            ignore = list("ignore", &self.project.ignore),
            include = list("include", &self.project.include),
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
            gate_timeout = self.gate.timeout,
            gate_command = Toml::quote(self.gate.command.trim()),
            claude = engine("claude", &self.claude),
            codex = engine("codex", &self.codex),
            max_audits = self.agent.max_audits,
            max_rounds = self.agent.max_rounds,
            max_fixes = self.agent.max_fixes,
            agent_timeout = self.agent.timeout,
            manager = member(&self.agent.manager),
            requires = roster(&self.agent.requires),
            tasks = roster(&self.agent.tasks),
            r_audits = roster(&self.agent.audits),
            r_tests = roster(&self.agent.tests),
            fuzzes_r = roster(&self.agent.fuzzes),
            benches_r = roster(&self.agent.benches),
            examples_r = roster(&self.agent.examples),
        )

    }

}
