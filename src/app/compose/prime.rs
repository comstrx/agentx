use std::path::PathBuf;

use crate::config::{Config, Train, base::prompts as P};
use crate::config::base::consts::{CONTRACTS, DESIGNS, HISTORY, OVERVIEW, REFERENCES, SKILLS};
use crate::core::fs::{Dir, Path};
use crate::app::{Compose, Journey, Phase};

impl Compose {

    pub(crate) fn prime ( cfg: &Config, journey: &Journey, phase: &str, agent: &str ) -> String {

        let parts: Vec<String> = match phase {
            "requires" => vec![
                P::PRIME.to_string(),
                Self::setup(cfg),
                Self::stage(cfg, journey),
                P::REQUIRES_ROLE.to_string(),
                P::REQUIRES_MISSION.to_string(),
                P::REQUIRES_FLAG.to_string(),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            "tasks" => vec![
                P::PRIME.to_string(),
                Self::setup(cfg),
                Self::stage(cfg, journey),
                P::TASKS_ROLE.to_string(),
                P::TASKS_IMPLEMENT.to_string(),
                P::TASKS_REMEDIATION.to_string(),
                Self::author_policy(cfg),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            "audits" => vec![
                P::PRIME.to_string(),
                Self::setup(cfg),
                Self::stage(cfg, journey),
                P::AUDITS_ROLE.to_string(),
                P::AUDITS_REVIEW.to_string(),
                P::AUDITS_WRITE.to_string(),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            "tests" | "benches" | "examples" | "fuzzes" => vec![
                P::PRIME.to_string(),
                Self::setup(cfg),
                Self::stage(cfg, journey),
                Self::mission_of(phase).to_string(),
                P::PRODUCE_SCOPE.to_string(),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            _ => Vec::new(),
        };

        Self::render(&parts, &Self::priming_pairs(cfg, phase, agent))

    }

    pub(crate) fn reaffirm ( cfg: &Config, agent: &str ) -> String {

        Self::render(&[P::REAFFIRM.to_string()], &Self::values(cfg, "requires", agent, None))

    }

    pub(crate) fn manager_brief ( cfg: &Config, journey: &Journey ) -> String {

        let parts = [
            P::PRIME.to_string(),
            Self::setup(cfg),
            Self::stage(cfg, journey),
            P::MANAGER_ROLE.to_string(),
            P::MANAGER_INIT.to_string(),
            P::TOLERANCE.to_string(),
            P::PRIME_READY.to_string(),
        ];

        Self::render(&parts, &Self::priming_pairs(cfg, "requires", "manager"))

    }

    fn stage ( cfg: &Config, journey: &Journey ) -> String {

        if !journey.intake_done && journey.task_status.is_empty() && journey.phase <= Phase::Requires {

            return P::STARTUP.to_string();

        }

        let shipped = journey.task_status.values().filter(|value| value.as_str() == "shipped").count();
        let total = Dir::markdown(&cfg.paths.tasks).len();
        let phase = format!("{:?}", journey.phase).to_lowercase();

        format!(
            "WHERE THIS RUN STANDS - RESUMING an unfinished journey: it reached the `{phase}` phase, with {shipped} \
            of {total} task(s) already shipped.\n\n{}",
            P::RESUME,
        )

    }

    fn priming_pairs ( cfg: &Config, phase: &str, agent: &str ) -> Vec<(&'static str, String)> {

        let mut pairs = Self::values(cfg, phase, agent, None);

        pairs.push(( SKILLS,     Self::study_block(cfg, SKILLS) ));
        pairs.push(( OVERVIEW,   Self::study_block(cfg, OVERVIEW) ));
        pairs.push(( CONTRACTS,  Self::study_block(cfg, CONTRACTS) ));
        pairs.push(( DESIGNS,    Self::study_block(cfg, DESIGNS) ));
        pairs.push(( REFERENCES, Self::study_block(cfg, REFERENCES) ));
        pairs.push(( HISTORY,    Self::history_block(cfg) ));

        pairs

    }

    fn study_block ( cfg: &Config, name: &str ) -> String {

        let files = Path::relative(cfg.context.bucket(name), &cfg.root);

        if files.is_empty() { return "  (none provided for this project)".to_string(); }

        let listed = files.into_iter().map(|file| format!("    {file}")).collect::<Vec<_>>().join("\n");

        format!("  OPEN and READ each file below IN FULL now - study it, do not skim and do not skip one:\n{listed}")

    }

    fn history_block ( cfg: &Config ) -> String {

        let name = cfg.spec.inspire.trim();

        if name.is_empty() {

            return "  (no archetype bound yet — no prior history; lean on the contracts and skills above)".to_string();

        }

        let group = |label: &str, files: Vec<PathBuf>| {

            match files.is_empty() {
                true => format!("{label}\n    (none yet — no prior run of this kind has reached here)"),
                false => {

                    let listed = Path::relative(&files, &cfg.root).into_iter().map(|file| format!("    {file}")).collect::<Vec<_>>().join("\n");

                    format!("{} These are the {} file(s) — open and study EVERY one of them, closely:\n{listed}", label, files.len())

                }
            }

        };

        [
            group("FIRST, the REQUIREMENTS that past projects of this exact kind delivered — they tell you what this kind of project actually needs.", Train::past_requires(name)),
            group("THEN, the TASK PLANS those requirements were decomposed into — how the work was cut, ordered, and contracted.", Train::past_tasks(name)),
            group("LAST, the manager's DECISION REPORTS — your deepest source: the key decisions, trade-offs, and the WHY behind every call. Study these hardest of all.", Train::history(name)),
        ].join("\n\n")

    }

    fn setup ( cfg: &Config ) -> String {

        let opt = &cfg.option;
        let onoff = |on: bool| if on { "on" } else { "off" };

        let archetype = match cfg.spec.inspire.trim().is_empty() {
            true => "unbound — the manager classifies it during this run".to_string(),
            false => cfg.spec.inspire.clone(),
        };

        let gate = match cfg.gate.command.trim().is_empty() {
            true => "to be detected after priming".to_string(),
            false => cfg.gate.command.clone(),
        };

        let mut team = vec![format!("    {:<12} {}", "manager:", cfg.manager())];

        for ( phase, on ) in [( "requires", true ), ( "tasks", true ), ( "audits", opt.audits ), ( "tests", opt.tests ), ( "benches", opt.benches ), ( "examples", opt.examples ), ( "fuzzes", opt.fuzzes )] {

            if !on { continue; }

            let roster = cfg.roster(phase);
            let label = format!("{}s:", Self::role_label(phase));

            team.push(format!("    {label:<12} {} — {}", roster.len(), roster.join(", ")));

        }

        format!(
            "THIS RUN — the concrete setup you are part of right now; read it and assume NO defaults:\n\
            - Project root: {}\n\
            - Archetype (the training kind this run learns from and feeds): {archetype}\n\
            - Quality gate (this tool runs it after every task turn): {gate}\n\
            - Phases after `tasks` — only the ON ones run, the rest are skipped entirely: audit {} · tests {} · benches {} · examples {} · fuzzes {}\n\
            - The team on this run — each name is one independent, separately-briefed model instance:\n{}\n\
            - Limits: up to {} manager review rounds per phase, {} gate-repair attempts per task, {} audit rounds.",
            Path::display(&cfg.root),
            onoff(opt.audits), onoff(opt.tests), onoff(opt.benches), onoff(opt.examples), onoff(opt.fuzzes),
            team.join("\n"),
            cfg.agent.max_rounds, cfg.agent.max_fixes, cfg.agent.max_audits,
        )

    }

}
