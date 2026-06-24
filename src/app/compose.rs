use std::path::Path as StdPath;

use crate::config::consts::{BUCKET_TITLES, CONTEXT_BUCKETS, CONVERGENCE};
use crate::config::{Config, Train, prompts as P};
use crate::core::support::fs::{Dir, Path};
use crate::core::support::str::Str;
use super::arch::{Compose, Journey};

impl Compose {

    pub(crate) fn prime ( cfg: &Config, phase: &str, agent: &str ) -> String {

        let parts: Vec<String> = match phase {
            "requires" => vec![
                P::ARCH_ROLE.to_string(),
                P::TEAM.to_string(),
                P::STAKES.to_string(),
                Self::briefing(cfg),
                P::ARCH_MISSION.to_string(),
                P::ARCH_FLAG.to_string(),
                P::LAW.to_string(),
                P::PRIME_STUDY.to_string(),
            ],
            "tasks" => vec![
                P::EXEC_ROLE.to_string(),
                P::TEAM.to_string(),
                P::STAKES.to_string(),
                Self::briefing(cfg),
                P::EXEC_IMPLEMENT.to_string(),
                P::LAW.to_string(),
                P::PRIME_STUDY.to_string(),
            ],
            "tests" => vec![
                P::VERIFY_ROLE.to_string(),
                P::TEAM.to_string(),
                P::STAKES.to_string(),
                Self::briefing(cfg),
                P::VERIFY_WORKSPACE.to_string(),
                P::VERIFY_STRATEGY.to_string(),
                Self::tests_policy(cfg),
                P::LAW.to_string(),
                P::PRIME_STUDY.to_string(),
            ],
            _ => Vec::new(),
        };

        Self::render(&parts, &Self::values(cfg, phase, agent, None))

    }

    pub(crate) fn reaffirm ( agent: &str ) -> String {

        P::REAFFIRM.replace("{agent}", agent)

    }

    pub(crate) fn architect ( cfg: &Config, agent: &str, has_review: bool ) -> String {

        let mut parts = vec![P::ARCH_WORK.to_string()];

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::ARCH_REPORT.to_string());

        Self::render(&parts, &Self::values(cfg, "requires", agent, None))

    }

    pub(crate) fn executor ( cfg: &Config, agent: &str, task: &StdPath, gate_failed: bool, has_review: bool ) -> String {

        let mut parts = vec![P::EXEC_TASK.to_string()];

        if gate_failed { parts.push(P::EXEC_GATE_FAIL.to_string()); }

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::EXEC_REPORT.to_string());

        Self::render(&parts, &Self::values(cfg, "tasks", agent, Some(task)))

    }

    pub(crate) fn verifier ( cfg: &Config, agent: &str, has_review: bool ) -> String {

        let mut parts = vec![P::VERIFY_WORK.to_string(), Self::tests_policy(cfg)];

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::VERIFY_REPORT.to_string());

        Self::render(&parts, &Self::values(cfg, "tests", agent, None))

    }

    pub(crate) fn manager_brief ( cfg: &Config ) -> String {

        let parts = [
            P::MANAGER_ROLE.to_string(),
            P::STAKES.to_string(),
            P::MANAGER_INIT.replace("{context}", &Self::context_block(cfg)),
        ];

        parts.join("\n\n")

    }

    pub(crate) fn manager_intake ( cfg: &Config, journey: &Journey ) -> String {

        let kind = &cfg.spec.inspire;

        let mut sources = if kind.is_empty() { Vec::new() } else { Train::requires(kind) };
        sources.extend(cfg.context.requires.iter().cloned());

        let list = match sources.is_empty() {
            true => "  (none discovered)".to_string(),
            false => sources.iter().map(|path| format!("  {}", Self::rel(path, &cfg.root))).collect::<Vec<_>>().join("\n"),
        };

        let pairs = vec![
            ( "sources", list ),
            ( "requires", Self::rel(&cfg.paths.inbox, &cfg.root) ),
            ( "state", Self::intake_state(cfg, journey) ),
        ];

        Self::render(&[P::MANAGER_INTAKE.to_string()], &pairs)

    }

    fn intake_state ( cfg: &Config, journey: &Journey ) -> String {

        let backlog = Dir::markdown(&cfg.paths.inbox).len();
        let tasks = Dir::markdown(&cfg.paths.tasks).len();
        let requires = Self::rel(&cfg.paths.inbox, &cfg.root);
        let tasks_dir = Self::rel(&cfg.paths.tasks, &cfg.root);

        let mode = match backlog > 0 || tasks > 0 {
            true => format!("RESUME — an earlier run already advanced this journey (phase {:?}, status {:?}). It will continue from where it stopped; you are NOT starting over.", journey.phase, journey.status),
            false => "FRESH — no backlog or tasks exist yet; you are creating the backlog for the first time.".to_string(),
        };

        format!(
            "RUN STATE — read before writing anything:\n\
            - Mode: {mode}\n\
            - Requirement files already in {requires}/: {backlog}.\n\
            - Task files already in {tasks_dir}/: {tasks}.\n\
            If a backlog already exists, intake has run before: READ every existing file first, CONTINUE the numbering, and do NOT recreate, renumber, or rewrite a requirement already captured — the architects may already have built tasks from it, so changing it now would break the run. Add ONLY genuinely-missing requirements; if the backlog is already complete and correct for the sources, change nothing and stop."
        )

    }

    pub(crate) fn manager_review ( cfg: &Config, phase: &str, task: Option<&StdPath>, round: u32 ) -> String {

        let body = match phase {
            "requires" => P::MANAGER_REVIEW_ARCH,
            "tasks" => P::MANAGER_REVIEW_WORK,
            "tests" => P::MANAGER_REVIEW_TEST,
            _ => "",
        };

        let counter = format!("Review round {round} of at most {}.", cfg.spec.max_rounds);

        let mut parts = vec![
            P::MANAGER_ROLE.to_string(),
            counter,
            P::MANAGER_INTEGRATION.to_string(),
            body.to_string(),
        ];

        if phase == "tests" { parts.push(Self::tests_policy(cfg)); }

        parts.push(P::MANAGER_FLAG.to_string());
        parts.push(P::MANAGER_VERDICT.to_string());

        Self::render(&parts, &Self::values(cfg, phase, "manager", task))

    }

    pub(crate) fn manager_summary ( cfg: &Config ) -> String {

        let pairs = vec![
            ( "rounds", Self::rel(&cfg.paths.rounds, &cfg.root) ),
            ( "summary", Self::rel(&cfg.paths.summary(), &cfg.root) ),
        ];

        let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_SUMMARY.to_string()];

        Self::render(&parts, &pairs)

    }

    fn values ( cfg: &Config, phase: &str, agent: &str, task: Option<&StdPath> ) -> Vec<(&'static str, String)> {

        let paths = &cfg.paths;
        let root = &cfg.root;

        let rounds = match task {
            Some(path) => paths.task_rounds(&Path::stem_of(path)),
            None => paths.rounds_of(phase),
        };

        let current = task.map(|path| Self::rel(path, root)).unwrap_or_default();

        vec![
            ( "token", CONVERGENCE.to_string() ),
            ( "cache", Self::rel(&paths.cache, root) ),
            ( "requires", Self::rel(&paths.inbox, root) ),
            ( "tasks", Self::rel(&paths.tasks, root) ),
            ( "task", current ),
            ( "reports", Self::rel(&paths.reports_of(phase), root) ),
            ( "rounds", Self::rel(&rounds, root) ),
            ( "report", Self::rel(&paths.report_of(phase, agent), root) ),
            ( "review", Self::rel(&paths.review_of(phase), root) ),
            ( "summary", Self::rel(&paths.summary(), root) ),
            ( "gate_log", Self::rel(&paths.gate_log, root) ),
            ( "tests", Self::rel(&paths.tests, root) ),
            ( "probes", Self::rel(&paths.probes, root) ),
            ( "agent", agent.to_string() ),
        ]

    }

    fn rel ( path: &StdPath, root: &StdPath ) -> String {

        Path::relative_one(path, root)

    }

    fn tests_policy ( cfg: &Config ) -> String {

        match cfg.spec.tests {
            true => P::TESTS_REAL.to_string(),
            false => P::TESTS_SCRATCH.to_string(),
        }

    }

    fn title_of ( name: &str ) -> &str {

        BUCKET_TITLES.iter().find(|( bucket, _ )| *bucket == name).map_or(name, |( _, title )| *title)

    }

    fn context_block ( cfg: &Config ) -> String {

        let mut lines = vec!["Project context (read once, internalise, comply). Within each section, files are ordered shared-first then project-specific; when two files conflict, the LATER file in the list wins:".to_string()];

        for name in CONTEXT_BUCKETS {

            lines.push(String::new());
            lines.push(format!("{}:", Self::title_of(name)));

            let files = Path::relative(cfg.context.bucket(name), &cfg.root);

            if files.is_empty() {

                lines.push("  (none)".to_string());

            }
            else {

                lines.extend(files.into_iter().map(|file| format!("  {file}")));

            }

        }

        let requires = Path::relative(&Dir::markdown(&cfg.paths.inbox), &cfg.root);

        lines.push(String::new());
        lines.push(format!("{}:", Self::title_of("requires")));

        if requires.is_empty() {

            lines.push("  (none)".to_string());

        }
        else {

            lines.extend(requires.into_iter().map(|file| format!("  {file}")));

        }

        lines.join("\n")

    }

    fn briefing ( cfg: &Config ) -> String {

        P::BRIEFING.replace("{context}", &Self::context_block(cfg))

    }

    fn render ( parts: &[String], pairs: &[(&str, String)] ) -> String {

        Str::template(&parts.join("\n\n"), pairs)

    }

}
