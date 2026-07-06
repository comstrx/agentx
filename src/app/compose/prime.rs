use crate::config::{Config, base::prompts as P};
use crate::config::base::consts::{CONTRACTS, DESIGNS, HISTORY, OVERVIEW, REFERENCES, SKILLS};
use crate::core::fs::{Dir, Path};
use crate::app::{Compose, Journey, Phase};

impl Compose {

    pub(crate) fn prime ( cfg: &Config, journey: &Journey, phase: &str, agent: &str ) -> String {

        let parts: Vec<String> = match phase {
            "requires" => vec![
                P::PRIME.to_string(),
                P::CRAFT.to_string(),
                Self::stage_policy(cfg).to_string(),
                Self::setup(cfg, phase, agent),
                Self::stage(cfg, journey),
                P::REQUIRES_ROLE.to_string(),
                P::REQUIRES_MISSION.to_string(),
                P::REQUIRES_FLAG.to_string(),
                P::DOORS.to_string(),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            "tasks" => {

                let mut parts = vec![
                    P::PRIME.to_string(),
                    P::CRAFT.to_string(),
                    Self::stage_policy(cfg).to_string(),
                    Self::setup(cfg, phase, agent),
                    Self::stage(cfg, journey),
                    P::TASKS_ROLE.to_string(),
                    P::TASKS_IMPLEMENT.to_string(),
                ];

                if cfg.option.audits { parts.push(P::TASKS_REMEDIATION.to_string()); }

                parts.push(P::DOORS.to_string());
                parts.push(P::DEPENDENCIES.to_string());
                parts.push(Self::author_policy(cfg));
                parts.push(P::TOLERANCE.to_string());
                parts.push(P::PRIME_READY.to_string());

                parts

            },
            "audits" => vec![
                P::PRIME.to_string(),
                P::CRAFT.to_string(),
                Self::stage_policy(cfg).to_string(),
                Self::setup(cfg, phase, agent),
                Self::stage(cfg, journey),
                P::AUDITS_ROLE.to_string(),
                P::AUDITS_REVIEW.to_string(),
                P::AUDITS_WRITE.to_string(),
                P::DOORS.to_string(),
                P::TOLERANCE.to_string(),
                P::PRIME_READY.to_string(),
            ],
            "tests" | "benches" | "examples" | "fuzzes" => vec![
                P::PRIME.to_string(),
                P::CRAFT.to_string(),
                Self::stage_policy(cfg).to_string(),
                Self::setup(cfg, phase, agent),
                Self::stage(cfg, journey),
                Self::mission_of(phase).to_string(),
                P::PRODUCE_SCOPE.to_string(),
                P::DOORS.to_string(),
                P::DEPENDENCIES.to_string(),
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

    pub(crate) fn manager_addendum ( cfg: &Config ) -> String {

        Self::render(&[P::MANAGER_ADDENDUM.to_string()], &Self::priming_pairs(cfg, "requires", "manager"))

    }

    pub(crate) fn manager_brief ( cfg: &Config, journey: &Journey ) -> String {

        let parts = [
            P::PRIME.to_string(),
            P::CRAFT.to_string(),
            Self::stage_policy(cfg).to_string(),
            Self::setup(cfg, "requires", "manager"),
            Self::stage(cfg, journey),
            P::MANAGER_ROLE.to_string(),
            P::MANAGER_INIT.to_string(),
            P::TOLERANCE.to_string(),
            P::PRIME_READY.to_string(),
        ];

        Self::render(&parts, &Self::priming_pairs(cfg, "requires", "manager"))

    }

    fn stage ( cfg: &Config, journey: &Journey ) -> String {

        if journey.task_status.is_empty() && journey.phase <= Phase::Requires && Dir::markdown(&cfg.paths.tasks).is_empty() {

            return P::STARTUP.to_string();

        }

        let shipped = journey.task_status.values().filter(|value| value.as_str() == "shipped").count();
        let total = Dir::markdown(&cfg.paths.tasks).len();
        let phase = journey.phase.slug();

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

        let files = Path::relative(&cfg.context.history, &cfg.root);

        if files.is_empty() {

            return "  (no prior work yet — this is the first project of its kind; lean on the contracts and skills above)".to_string();

        }

        let listed = files.into_iter().map(|file| format!("    {file}")).collect::<Vec<_>>().join("\n");

        format!("  This is the accumulated knowledge of this project and the prior work of its kind - the decision reports from earlier runs, oldest to newest. OPEN and study EVERY one closely; it is your deepest source of what already works and the calls not to reopen. Master them as if you had written them yourself: continuing these decisions is the fastest correct path, and rediscovering them is pure waste:\n{listed}")

    }

    fn setup ( cfg: &Config, phase: &str, agent: &str ) -> String {

        let opt = &cfg.option;
        let onoff = |on: bool| if on { "on" } else { "off" };

        let archetype = match cfg.spec.inspire.trim().is_empty() {
            true => "unbound — ruled on at intake: classified by the manager on a fix ruling, or runs without inherited knowledge".to_string(),
            false => cfg.spec.inspire.clone(),
        };

        let gate = match cfg.gate.command.trim().is_empty() {
            true => "none yet — verified at intake; the manager composes one if the operator rules fix".to_string(),
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
            - Phases after `tasks` — only the ON ones run, the rest are skipped entirely: audits {} · tests {} · benches {} · examples {} · fuzzes {}\n\
            - The team on this run — each name is one independent, separately-briefed model instance:\n{}\n\
            {}\n\
            - Limits: up to {} manager review rounds per phase, {} gate-repair attempts per task, {} audit rounds.",
            Path::display(&cfg.root),
            onoff(opt.audits), onoff(opt.tests), onoff(opt.benches), onoff(opt.examples), onoff(opt.fuzzes),
            team.join("\n"),
            Self::seat(cfg, phase, agent),
            cfg.agent.max_rounds, cfg.agent.max_fixes, cfg.agent.max_audits,
        )

    }

    fn seat ( cfg: &Config, phase: &str, agent: &str ) -> String {

        if agent == "manager" || agent == cfg.manager() {

            return format!(
                "- YOUR seat: the MANAGER ({}) — the single authority on quality; every roster above ships to YOUR review, \
                and this tool hands you each step when it is time.",
                cfg.manager(),
            );

        }

        let roster = cfg.roster(phase);
        let role = Self::role_label(phase);
        let judged = format!("The manager ({}) reads your report and the real code after every round and rules ship or revise.", cfg.manager());

        let consumer = Self::consumer_of(phase);

        if roster.len() <= 1 {

            return format!(
                "- YOUR seat: {agent}, the ONLY {role} this run — the whole relay is yours: no earlier report to \
                inherit, no one refining behind you. {judged} {consumer}",
            );

        }

        let relay = roster.iter()
            .map(|name| if name == agent { format!("{name} (YOU)") } else { name.clone() })
            .collect::<Vec<_>>()
            .join(" -> ");

        format!(
            "- YOUR seat: {agent}, one of the {role}s above. The {role} relay runs in this EXACT order every \
            round: {relay} — each seat opens the reports of the seats before it and sharpens the shared work, \
            and the seats after you inherit yours. {judged} {consumer}",
        )

    }

    fn consumer_of ( phase: &str ) -> &'static str {

        match phase {
            "requires" => "Your task files are the ONLY spec the executors build from — a vague line in them becomes an executor's guess.",
            "tasks"    => "Your code is what the operator ships and what every later active phase exercises — the whole run stands on what you leave behind.",
            "audits"   => "Your remediation tasks are executed VERBATIM by the executors — an imprecise Fix becomes wrong code.",
            _          => "Your captured evidence is what the manager's verdict stands on — an unproven claim misleads the whole run.",
        }

    }

}
