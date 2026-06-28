use std::path::Path as StdPath;

use crate::config::base::consts::CONVERGENCE;
use crate::config::{Config, base::prompts as P};
use crate::core::fs::Path;
use crate::core::str::Str;
use crate::app::Compose;

impl Compose {

    pub(super) fn values ( cfg: &Config, phase: &str, agent: &str, task: Option<&StdPath> ) -> Vec<(&'static str, String)> {

        let paths = &cfg.paths;
        let root = &cfg.root;

        let rounds = match task {
            Some(path) => paths.task_rounds(&Path::stem_of(path)),
            None => paths.rounds_of(phase),
        };

        let current = task.map(|path| Self::rel(path, root)).unwrap_or_default();

        vec![
            ( "token", CONVERGENCE.to_string() ),
            ( "config", Self::rel(&paths.config_file, root) ),
            ( "cache", Self::rel(&paths.cache, root) ),
            ( "requires", Self::rel(&paths.inbox, root) ),
            ( "tasks", Self::rel(&paths.tasks, root) ),
            ( "audit", Self::rel(&paths.audit, root) ),
            ( "task", current ),
            ( "reports", Self::rel(&paths.reports_of(phase), root) ),
            ( "rounds", Self::rel(&rounds, root) ),
            ( "report", Self::rel(&paths.report_of(phase, agent), root) ),
            ( "review", Self::rel(&paths.review_of(phase), root) ),
            ( "gate_log", Self::rel(&paths.gate_log, root) ),
            ( "agent", agent.to_string() ),
        ]

    }

    pub(super) fn rel ( path: &StdPath, root: &StdPath ) -> String {

        Path::relative_one(path, root)

    }

    fn flag ( on: bool, yes: &str, no: &str ) -> String {

        if on { yes.to_string() } else { no.to_string() }

    }

    pub(super) fn author_policy ( cfg: &Config ) -> String {

        let opt = &cfg.option;

        [
            Self::flag(opt.comments, P::COMMENTS_ON, P::COMMENTS_OFF),
            Self::flag(opt.format, P::FORMATS_ON, P::FORMATS_OFF),
            Self::docs_policy(cfg),
        ].join("\n\n")

    }

    pub(super) fn mission_of ( phase: &str ) -> &'static str {

        match phase {
            "tests"    => P::TESTS_MISSION,
            "benches"  => P::BENCHES_MISSION,
            "examples" => P::EXAMPLES_MISSION,
            "fuzzes"   => P::FUZZES_MISSION,
            _          => "",
        }

    }

    pub(super) fn duty_of ( phase: &str ) -> String {

        match phase {
            "audits"   => "remediation tasks",
            "tests"    => "test suite",
            "benches"  => "benchmarks",
            "examples" => "examples",
            "fuzzes"   => "fuzzing",
            _          => "deliverable",
        }.to_string()

    }

    pub(super) fn docs_policy ( cfg: &Config ) -> String {

        match ( cfg.option.doc_blocks, cfg.option.doc_contracts ) {
            ( true, _ )      => P::DOC_BLOCKS_ON.to_string(),
            ( false, true )  => format!("{}\n\n{}", P::DOC_BLOCKS_OFF, P::DOC_CONTRACTS_ON),
            ( false, false ) => format!("{}\n\n{}", P::DOC_BLOCKS_OFF, P::DOC_CONTRACTS_OFF),
        }

    }

    pub(super) fn gate_pillars ( cfg: &Config ) -> String {

        let opt = &cfg.option;

        let mut pillars = vec!["CHECK — prove the code is syntactically valid, every reference resolves, and all dependencies are present and compatible. Use the single lightest command the toolchain offers for this and nothing heavier; it MUST NOT run tests, lint, format, generate code, migrate, deploy, or write any file."];

        if opt.lint   { pillars.push("LINT — run the project's static-analysis / linter in REPORT-ONLY mode: no --fix, no autocorrect, no file writes. A reported violation fails the gate."); }

        if opt.format { pillars.push("FORMAT — run the project's formatter in CHECK mode ONLY (--check / --dry-run / --diff): it fails on any deviation and NEVER rewrites a file."); }

        if opt.tests  { pillars.push("TEST — run the project's automated test suite to completion. Any failing or errored test fails the gate."); }

        pillars.iter().enumerate().map(|( index, pillar )| format!("{}. {pillar}", index + 1)).collect::<Vec<_>>().join("\n")

    }

    pub(super) fn description_block ( cfg: &Config ) -> String {

        let note = cfg.spec.description.trim();

        match note.is_empty() {
            true => String::new(),
            false => format!("\nThe operator describes this project, in their own words — weigh it as intent, not as a contract:\n{note}\n"),
        }

    }

    pub(super) fn render ( parts: &[String], pairs: &[(&str, String)] ) -> String {

        Str::template(&parts.join("\n\n"), pairs)

    }

}
