use std::path::Path as StdPath;

use crate::config::{Config, Train, base::prompts as P};
use crate::config::base::consts::PHASES;
use crate::core::fs::{Dir, Path};
use crate::app::{Compose, Journey, Ruling};

impl Compose {

    fn source_list ( cfg: &Config ) -> String {

        let sources = &cfg.context.requires;

        match sources.is_empty() {
            true => "  (none discovered)".to_string(),
            false => sources.iter().map(|path| format!("  {}", Self::rel(path, &cfg.root))).collect::<Vec<_>>().join("\n"),
        }

    }

    pub(crate) fn manager_requires_check ( cfg: &Config ) -> String {

        Self::render(&[P::INTAKE_REQUIRES_CHECK.to_string()], &[
            ( "sources", Self::source_list(cfg) ),
            ( "conflict", Self::rel(&cfg.paths.conflict, &cfg.root) ),
        ])

    }

    pub(crate) fn manager_convert ( cfg: &Config, journey: &Journey, ruling: Option<Ruling> ) -> String {

        let mut parts = vec![P::MANAGER_INTAKE.to_string()];

        if cfg.force { parts.push(P::INTAKE_FORCED.to_string()); }
        else {

            match ruling {
                Some(Ruling::Proceed) => parts.push(P::INTAKE_PROCEED.to_string()),
                Some(Ruling::Fix)     => parts.push(P::INTAKE_REQUIRES_FIX.to_string()),
                _                     => {}
            }

        }

        parts.push(P::INTAKE_CLOSE.to_string());

        let pairs = vec![
            ( "sources", Self::source_list(cfg) ),
            ( "requires", Self::rel(&cfg.paths.inbox, &cfg.root) ),
            ( "state", Self::intake_state(cfg, journey) ),
            ( "conflict", Self::rel(&cfg.paths.conflict, &cfg.root) ),
        ];

        Self::render(&parts, &pairs)

    }

    pub(crate) fn manager_project_check ( cfg: &Config ) -> String {

        Self::render(&[P::INTAKE_PROJECT.to_string()], &[
            ( "root", Path::display(&cfg.root) ),
            ( "conflict", Self::rel(&cfg.paths.conflict, &cfg.root) ),
        ])

    }

    pub(crate) fn manager_project_fix ( _cfg: &Config ) -> String {

        P::INTAKE_PROJECT_FIX.to_string()

    }

    pub(crate) fn manager_requires_fix ( cfg: &Config ) -> String {

        Self::render(&[P::INTAKE_REQUIRES_FIX.to_string()], &[( "requires", Self::rel(&cfg.paths.inbox, &cfg.root) )])

    }

    pub(crate) fn manager_gate_fix ( cfg: &Config, answer: &str ) -> String {

        Self::render(&[P::INTAKE_GATE_FIX.to_string()], &[
            ( "pillars", Self::gate_pillars(cfg) ),
            ( "answer", answer.to_string() ),
        ])

    }

    pub(crate) fn manager_gate_check ( cfg: &Config ) -> String {

        let gate = match cfg.gate.command.trim().is_empty() {
            true => "(empty - no gate is configured)".to_string(),
            false => cfg.gate.command.trim().to_string(),
        };

        Self::render(&[P::INTAKE_GATE.to_string()], &[
            ( "gate", gate ),
            ( "pillars", Self::gate_pillars(cfg) ),
            ( "conflict", Self::rel(&cfg.paths.conflict, &cfg.root) ),
            ( "config", Self::rel(&cfg.paths.config_file, &cfg.root) ),
        ])

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

    pub(crate) fn manager_review ( cfg: &Config, phase: &str, task: Option<&StdPath>, round: u32, gate_ok: bool ) -> String {

        let body = match phase {
            "requires" => P::MANAGER_REVIEW_REQUIRES,
            "tasks"    => P::MANAGER_REVIEW_TASKS,
            "audits"   => P::MANAGER_REVIEW_AUDITS,
            "tests"    => P::MANAGER_REVIEW_TESTS,
            "benches"  => P::MANAGER_REVIEW_BENCHES,
            "examples" => P::MANAGER_REVIEW_EXAMPLES,
            "fuzzes"   => P::MANAGER_REVIEW_FUZZES,
            _          => "",
        };

        let mut parts = vec![
            P::MANAGER_ROLE.to_string(),
            Self::situation(cfg, phase, task, round, gate_ok),
            P::MANAGER_INTEGRATION.to_string(),
            body.to_string(),
        ];

        if phase == "tasks" {

            parts.push(P::MANAGER_POLICY.to_string());
            parts.push(Self::author_policy(cfg));

        }

        if matches!(phase, "tasks" | "audits") {

            parts.push(P::MANAGER_STAGE.to_string());
            parts.push(Self::stage_policy(cfg).to_string());

        }

        parts.push(P::MANAGER_FLAG.to_string());
        parts.push(P::MANAGER_VERDICT.to_string());

        Self::render(&parts, &Self::values(cfg, phase, "manager", task))

    }

    fn situation ( cfg: &Config, phase: &str, task: Option<&StdPath>, round: u32, gate_ok: bool ) -> String {

        let max = cfg.agent.max_rounds;
        let roster = cfg.roster(phase);
        let count = roster.len();

        let team = match roster.is_empty() {
            true => "(none)".to_string(),
            false => roster.join(", "),
        };

        let role = Compose::role_label(phase);

        let plural = if count == 1 { role.to_string() } else { format!("{role}s") };
        let ladder = Self::ladder(cfg, phase);
        let encodes = "Each name encodes its backend and instance (claude_1 runs on claude, codex_1 on codex, claude_2 a second claude, and so on), and each authored its OWN report.";

        let reports = Self::rel(&cfg.paths.reports_of(phase), &cfg.root);
        let requires = Self::rel(&cfg.paths.inbox, &cfg.root);
        let tasks = Self::rel(&cfg.paths.tasks, &cfg.root);
        let audit = Self::rel(&cfg.paths.audit, &cfg.root);

        let rounds = match task {
            Some(path) => Self::rel(&cfg.paths.task_rounds(&Path::stem_of(path)), &cfg.root),
            None => Self::rel(&cfg.paths.rounds_of(phase), &cfg.root),
        };

        match phase {
            "requires" => format!(
                "SITUATION — PHASE REQUIRES (architecture) — {ladder}. This is review round {round} of at most {max} for \
                this phase; the architects have just finished a full round among themselves and handed you the plan to \
                judge.\n\
                The team this round: {count} {plural} — {team}. {encodes} They worked FROM the requirements backlog in \
                {requires}/ and PRODUCED the ordered task plan in {tasks}/.\n\
                Before you rule: read every architect's report for THIS round in {reports}/, and the full discussion trail \
                across ALL prior rounds in {rounds}/."
            ),
            "tasks" => {

                let current = task.map(|path| Self::rel(path, &cfg.root)).unwrap_or_default();

                let gate = match gate_ok {
                    true => "the quality gate ran GREEN after every executor turn",
                    false => "the quality gate is RED — a red gate cannot ship",
                };

                format!(
                    "SITUATION — PHASE TASKS (execution) — {ladder}. This is review round {round} of at most {max} for THIS \
                    one task; the executors have just run a full round on it — {gate} — and handed it to you.\n\
                    The team this round: {count} {plural} — {team}. {encodes} They implemented exactly ONE task contract \
                    this round: {current}, drawn from the ordered plan in {tasks}/.\n\
                    Before you rule: read the actual code this task touched, every executor's report for THIS round in \
                    {reports}/, and the full trail across ALL prior rounds for this task in {rounds}/."
                )

            },
            "audits" => format!(
                "SITUATION — PHASE AUDITS (whole-system review) — {ladder}. This is review round {round} of at most {max} \
                for this phase; the auditors have just finished a full round judging the ENTIRE system and handed you \
                their findings.\n\
                The team this round: {count} {plural} — {team}. {encodes} They reviewed the WHOLE project — integration, \
                layering, abstraction and duplication, provider→adaptor boundaries, dangerous dependencies, performance, \
                and committed secrets — NOT only the executed tasks, and they EXECUTE nothing: their deliverable is \
                explained remediation task files (Problem · Why · Fix) under {audit}/.\n\
                Before you rule: read every auditor's report for THIS round in {reports}/, the remediation tasks they \
                propose in {audit}/, and the full trail across ALL prior rounds in {rounds}/."
            ),
            _ => {

                let duty = Self::duty_of(phase);
                let head = phase.to_uppercase();

                let gate = match gate_ok {
                    true => "The quality gate ran GREEN after this round.".to_string(),
                    false => "The quality gate is RED after this round — a red gate cannot ship: either the producers' own artifacts are broken (revise), or a valid check exposed a real defect in the executed code (record the defect as a DEFECT, revise).".to_string(),
                };

                format!(
                    "SITUATION — PHASE {head} (after tasks) — {ladder}. This is review round {round} of at most {max} for \
                    this phase; the {plural} have just finished a full round producing the {duty} for the executed tasks \
                    and handed it to you.\n\
                    The team this round: {count} {plural} — {team}. {encodes} They worked ONLY on the executed tasks in \
                    {tasks}/ — not the project at large — and ran the {duty} for real. {gate}\n\
                    Before you rule: read the {duty} they actually produced and ran, every {role}'s report for THIS round \
                    in {reports}/, and the full trail across ALL prior rounds in {rounds}/."
                )

            },
        }

    }

    fn ladder ( cfg: &Config, phase: &str ) -> String {

        let active: Vec<&str> = PHASES.into_iter().filter(|name| cfg.option.active(name)).collect();
        let position = active.iter().position(|name| *name == phase).map(|index| index + 1).unwrap_or(1);

        format!("{} of the {} active phase(s) this run: {}", Self::ordinal(position), active.len(), active.join(" → "))

    }

    fn ordinal ( n: usize ) -> String {

        let suffix = match ( n % 10, n % 100 ) {
            ( 1, 11 ) | ( 2, 12 ) | ( 3, 13 ) => "th",
            ( 1, _ ) => "st",
            ( 2, _ ) => "nd",
            ( 3, _ ) => "rd",
            _        => "th",
        };

        format!("{n}{suffix}")

    }

    pub(crate) fn role_label ( phase: &str ) -> &'static str {

        match phase {
            "requires" => "architect",
            "tasks"    => "executor",
            "audits"   => "auditor",
            "tests"    => "tester",
            "benches"  => "bencher",
            "examples" => "exampler",
            "fuzzes"   => "fuzzer",
            _          => "agent",
        }

    }

    pub(crate) fn manager_discover ( cfg: &Config, answer: &str ) -> String {

        let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_DISCOVER.to_string()];

        Self::render(&parts, &[
            ( "description", Self::description_block(cfg) ),
            ( "types", Train::catalogue() ),
            ( "base", Path::display(&Train::base()) ),
            ( "center", Path::display(&Train::projects()) ),
            ( "archive", Path::display(&Train::histories()) ),
            ( "answer", answer.to_string() ),
        ])

    }

    pub(crate) fn manager_create ( cfg: &Config ) -> String {

        let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_CREATE.to_string()];

        Self::render(&parts, &[( "description", Self::description_block(cfg) )])

    }

    pub(crate) fn manager_finalize ( cfg: &Config ) -> String {

        let pairs = vec![
            ( "requires", Self::rel(&cfg.paths.inbox, &cfg.root) ),
            ( "manager", Self::rel(&cfg.paths.manager, &cfg.root) ),
            ( "rounds", Self::rel(&cfg.paths.rounds, &cfg.root) ),
        ];

        let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_FINALIZE.to_string()];

        Self::render(&parts, &pairs)

    }

}
