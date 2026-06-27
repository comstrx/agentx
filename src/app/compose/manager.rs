use std::path::Path as StdPath;

use crate::config::{Config, Train, base::prompts as P};
use crate::core::fs::{Dir, Path};
use crate::app::{Compose, Journey};

impl Compose {

    pub(crate) fn manager_intake ( cfg: &Config, journey: &Journey ) -> String {

        let sources = &cfg.context.requires;

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
            Self::situation(cfg, phase, task, round),
            P::MANAGER_INTEGRATION.to_string(),
            body.to_string(),
        ];

        if phase == "tasks" {

            parts.push(P::MANAGER_POLICY.to_string());
            parts.push(Self::author_policy(cfg));

        }

        parts.push(P::MANAGER_FLAG.to_string());
        parts.push(P::MANAGER_VERDICT.to_string());

        Self::render(&parts, &Self::values(cfg, phase, "manager", task))

    }

    fn situation ( cfg: &Config, phase: &str, task: Option<&StdPath>, round: u32 ) -> String {

        let max = cfg.agent.max_rounds;
        let roster = cfg.roster(phase);
        let count = roster.len();

        let team = match roster.is_empty() {
            true => "(none)".to_string(),
            false => roster.join(", "),
        };

        let role = Compose::role_label(phase);

        let plural = if count == 1 { role.to_string() } else { format!("{role}s") };

        let reports = Self::rel(&cfg.paths.reports_of(phase), &cfg.root);
        let requires = Self::rel(&cfg.paths.inbox, &cfg.root);
        let tasks = Self::rel(&cfg.paths.tasks, &cfg.root);

        let rounds = match task {
            Some(path) => Self::rel(&cfg.paths.task_rounds(&Path::stem_of(path)), &cfg.root),
            None => Self::rel(&cfg.paths.rounds_of(phase), &cfg.root),
        };

        match phase {
            "requires" => format!(
                "SITUATION — PHASE 1 of 3: REQUIRES (architecture). This is review round {round} of at most {max} for \
                this phase; the architects have just finished a full round among themselves and handed you the plan to \
                judge.\n\
                The team this round: {count} {plural} — {team}. Each name encodes its backend and instance (claude_1 \
                runs on claude, codex_1 on codex, claude_2 a second claude, and so on), and each authored its OWN report. \
                They worked FROM the requirements backlog in {requires}/ and PRODUCED the ordered task plan in {tasks}/.\n\
                Before you rule: read every architect's report for THIS round in {reports}/, and the full discussion trail \
                across ALL prior rounds in {rounds}/."
            ),
            "tasks" => {

                let current = task.map(|path| Self::rel(path, &cfg.root)).unwrap_or_default();

                format!(
                    "SITUATION — PHASE 2 of 3: TASKS (execution). This is review round {round} of at most {max} for THIS \
                    one task; the executors have just run a full round on it — the quality gate ran GREEN after every \
                    executor turn — and handed it to you.\n\
                    The team this round: {count} {plural} — {team}. Each name encodes its backend and instance (claude_1 \
                    runs on claude, codex_1 on codex, claude_2 a second claude, and so on), and each authored its OWN \
                    report. They implemented exactly ONE task contract this round: {current}, drawn from the ordered plan \
                    in {tasks}/.\n\
                    Before you rule: read the actual code this task touched, every executor's report for THIS round in \
                    {reports}/, and the full trail across ALL prior rounds for this task in {rounds}/."
                )

            },
            _ => {

                let duty = Self::duty_of(phase);

                format!(
                    "SITUATION — THE {phase} PHASE (after tasks). This is review round {round} of at most {max} for \
                    this phase; the {plural} have just finished a full round producing the {duty} for the executed tasks and \
                    handed it to you.\n\
                    The team this round: {count} {plural} — {team}. Each name encodes its backend and instance (claude_1 runs \
                    on claude, codex_1 on codex, claude_2 a second claude, and so on), and each authored its OWN report. They \
                    worked ONLY on the executed tasks in {tasks}/ — not the project at large.\n\
                    Before you rule: read the {duty} they actually produced and ran, every {role}'s report for THIS round in \
                    {reports}/, and the full trail across ALL prior rounds in {rounds}/."
                )

            },
        }

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

        Self::render(&parts, &[( "description", Self::description_block(cfg) ), ( "types", Train::catalogue() ), ( "answer", answer.to_string() )])

    }

    pub(crate) fn manager_gate ( cfg: &Config, answer: &str ) -> String {

        let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_GATE.to_string()];

        Self::render(&parts, &[( "pillars", Self::gate_pillars(cfg) ), ( "answer", answer.to_string() )])

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
