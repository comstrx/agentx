use std::fs;
use std::path::Path;

use agentx::config::{Agent, Context, Gate, Options, Paths, Spec};

#[test]
fn project_and_option_defaults () {

    let spec = Spec::default();

    assert!(spec.inspire.is_empty());
    assert!(spec.description.is_empty());

    let opt = Options::default();

    assert!(!opt.lint);
    assert!(!opt.format);
    assert!(!opt.audits);
    assert!(!opt.tests);
    assert!(!opt.fuzzes);
    assert!(!opt.benches);
    assert!(!opt.examples);
    assert!(!opt.comments);
    assert!(!opt.doc_blocks);
    assert!(!opt.doc_contracts);

}

#[test]
fn gate_and_agent_defaults () {

    let gate = Gate::default();

    assert_eq!(gate.timeout, 1000);
    assert!(gate.command.is_empty());

    let agent = Agent::default();

    assert_eq!(agent.timeout, 10000);
    assert_eq!(agent.max_audits, 3);
    assert_eq!(agent.max_rounds, 3);
    assert_eq!(agent.max_fixes, 3);
    assert_eq!(agent.manager, "claude");
    assert_eq!(agent.requires, ["claude"]);
    assert_eq!(agent.audits, ["claude"]);

}

#[test]
fn roster_numbers_duplicate_models () {

    let agent = Agent {
        requires: vec!["claude".into(), "claude".into(), "codex".into()],
        ..Agent::default()
    };

    assert_eq!(agent.roster("requires"), ["claude_1", "claude_2", "codex_1"]);

}

#[test]
fn roster_selects_the_phase_and_is_empty_for_unknown () {

    let agent = Agent {
        tasks: vec!["codex".into()],
        benches: vec!["claude".into(), "codex".into()],
        ..Agent::default()
    };

    assert_eq!(agent.roster("tasks"), ["codex_1"]);
    assert_eq!(agent.roster("benches"), ["claude_1", "codex_1"]);
    assert!(agent.roster("nope").is_empty());

}

#[test]
fn paths_resolve_under_the_cache () {

    let paths = Paths::new(Path::new("/tmp/proj"));

    assert!(paths.cache.ends_with(".agentx"));
    assert!(paths.state.ends_with(".agentx/configs/state.json"));
    assert!(paths.config_file.ends_with("Agentx.toml"));

}

#[test]
fn stem_classification () {

    assert_eq!(Context::buckets_of_stem("agentx"), ["overview"]);
    assert_eq!(Context::buckets_of_stem("claude"), ["overview"]);
    assert_eq!(Context::buckets_of_stem("codex"), ["overview"]);
    assert_eq!(Context::buckets_of_stem("contracts"), ["contracts"]);
    assert_eq!(Context::buckets_of_stem("instructions"), ["contracts"]);
    assert_eq!(Context::buckets_of_stem("references"), ["references"]);

}

#[test]
fn spec_round_trips_through_toml () {

    let dir = std::env::temp_dir().join(format!("agentx-cfg-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let file = dir.join("Agentx.toml");

    let spec = Spec {
        inspire: "demo".into(),
        description: "a demo project".into(),
        ..Spec::default()
    };

    spec.save(&file).unwrap();

    let loaded = Spec::load(&file).unwrap();

    assert_eq!(loaded.inspire, "demo");
    assert_eq!(loaded.description, "a demo project");

    fs::remove_dir_all(&dir).ok();

}
