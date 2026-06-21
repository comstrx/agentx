use std::path::Path as StdPath;

use crate::config::names::{BUCKETS, CONVERGENCE};
use crate::config::{Config, prompts as P};
use crate::core::support::fs::Path;

const BUCKET_TITLES: [(&str, &str); 5] = [
    ( "overview",  "Overview / workflow" ),
    ( "contracts", "Contracts (LAW - they override everything)" ),
    ( "history",   "History / past decisions" ),
    ( "tasks",     "Existing tasks" ),
    ( "requires",  "Requirements to build" ),
];

fn title_of ( name: &str ) -> &str {

    BUCKET_TITLES.iter().find(|( bucket, _ )| *bucket == name).map_or(name, |( _, title )| *title)

}

fn rel ( path: &StdPath, root: &StdPath ) -> String {

    Path::relative_one(path, root)

}

fn context_block ( cfg: &Config ) -> String {

    let mut lines = vec!["Project context (read once, internalise, comply):".to_string()];

    for name in BUCKETS {

        lines.push(String::new());
        lines.push(format!("{}:", title_of(name)));

        let files = Path::relative(cfg.context.bucket(name), &cfg.root);

        if files.is_empty() {
            lines.push("  (none)".to_string());
        } else {
            lines.extend(files.into_iter().map(|file| format!("  {file}")));
        }
    }

    lines.join("\n")

}

fn briefing ( cfg: &Config ) -> String {

    P::BRIEFING.replace("{context}", &context_block(cfg))

}

fn values ( cfg: &Config, step: &str, agent: &str ) -> Vec<(&'static str, String)> {

    let paths = &cfg.paths;
    let root = &cfg.root;

    vec![
        ( "token", CONVERGENCE.to_string() ),
        ( "cache", rel(&paths.cache, root) ),
        ( "tasks", rel(&paths.tasks, root) ),
        ( "requires", rel(&paths.requires, root) ),
        ( "reports", rel(&paths.reports_of(step), root) ),
        ( "rounds", rel(&paths.rounds_of(step), root) ),
        ( "report", rel(&paths.reports_of(step).join(format!("{agent}.md")), root) ),
        ( "review", rel(&paths.review, root) ),
        ( "control", rel(&paths.control, root) ),
        ( "gate_log", rel(&paths.gate_log, root) ),
        ( "tests", rel(&paths.tests, root) ),
        ( "probes", rel(&paths.probes, root) ),
        ( "agent", agent.to_string() ),
    ]

}

fn render ( parts: &[String], pairs: &[(&str, String)] ) -> String {

    let mut text = parts.join("\n\n");

    for ( key, value ) in pairs {
        text = text.replace(&format!("{{{key}}}"), value);
    }

    text

}

pub fn architect ( cfg: &Config, agent: &str, init: bool, critique: bool, has_review: bool, frozen: &[String] ) -> String {

    let mut pairs = values(cfg, "arch", agent);
    let mut parts: Vec<String> = Vec::new();

    if init {

        parts.push(P::ARCH_ROLE.to_string());
        parts.push(P::TEAM.to_string());
        parts.push(briefing(cfg));

        if !frozen.is_empty() {
            pairs.push(( "frozen", frozen.iter().map(|name| format!("  {name}")).collect::<Vec<_>>().join("\n") ));
            parts.push(P::ARCH_FROZEN.to_string());
        }

        parts.push(P::ARCH_MISSION.to_string());
        parts.push(P::ARCH_FLAG.to_string());
        parts.push(P::LAW.to_string());

    } else {

        parts.push(format!("{agent}, your next architecture turn."));
        parts.push(P::ROUND.to_string());
    }

    if critique {
        parts.push(P::ARCH_CRITIQUE.to_string());
    }

    if has_review {
        parts.push(P::REVIEW_HANDOFF.to_string());
    }

    parts.push(P::ARCH_REPORT.to_string());

    render(&parts, &pairs)

}

pub fn executor ( cfg: &Config, agent: &str, init: bool, gate_failed: bool, has_review: bool ) -> String {

    let pairs = values(cfg, "work", agent);
    let mut parts: Vec<String> = Vec::new();

    if init {

        parts.push(P::EXEC_ROLE.to_string());
        parts.push(P::TEAM.to_string());
        parts.push(briefing(cfg));
        parts.push(P::EXEC_IMPLEMENT.to_string());
        parts.push(P::LAW.to_string());

    } else {

        parts.push(format!("{agent}, your next execution turn."));
        parts.push(P::ROUND.to_string());
    }

    if gate_failed {
        parts.push(P::EXEC_GATE_FAIL.to_string());
    }

    if has_review {
        parts.push(P::REVIEW_HANDOFF.to_string());
    }

    parts.push(P::EXEC_REPORT.to_string());

    render(&parts, &pairs)

}

pub fn verifier ( cfg: &Config, agent: &str, init: bool, has_review: bool ) -> String {

    let pairs = values(cfg, "test", agent);
    let mut parts: Vec<String> = Vec::new();

    if init {

        parts.push(P::VERIFY_ROLE.to_string());
        parts.push(P::TEAM.to_string());
        parts.push(briefing(cfg));
        parts.push(P::VERIFY_WORKSPACE.to_string());
        parts.push(P::VERIFY_STRATEGY.to_string());
        parts.push(P::LAW.to_string());

    } else {

        parts.push(format!("{agent}, your next verification turn."));
        parts.push(P::ROUND.to_string());
    }

    if has_review {
        parts.push(P::REVIEW_HANDOFF.to_string());
    }

    parts.push(P::VERIFY_REPORT.to_string());

    render(&parts, &pairs)

}

pub fn manager_brief ( cfg: &Config ) -> String {

    let parts = [P::MANAGER_ROLE.to_string(), P::MANAGER_INIT.replace("{context}", &context_block(cfg))];

    parts.join("\n\n")

}

pub fn manager_review ( cfg: &Config, step: &str, round_no: u32 ) -> String {

    let body = match step {
        "arch" => P::MANAGER_REVIEW_ARCH,
        "work" => P::MANAGER_REVIEW_WORK,
        "test" => P::MANAGER_REVIEW_TEST,
        _ => "",
    };

    let counter = format!("Review round {round_no} of at most {}.", cfg.spec.max_rounds);

    let parts = vec![
        P::MANAGER_ROLE.to_string(),
        counter,
        P::MANAGER_INTEGRATION.to_string(),
        body.to_string(),
        P::MANAGER_FLAG.to_string(),
        P::MANAGER_VERDICT.to_string(),
    ];

    render(&parts, &values(cfg, step, "manager"))

}

pub fn manager_decision ( cfg: &Config, decision: &StdPath ) -> String {

    let pairs = vec![
        ( "rounds", rel(&cfg.paths.rounds, &cfg.root) ),
        ( "decision", rel(decision, &cfg.root) ),
    ];

    let parts = vec![P::MANAGER_ROLE.to_string(), P::MANAGER_DECISION.to_string()];

    render(&parts, &pairs)

}
