use std::fs;
use std::path::Path;

use agentx::config::{Agent, Context, Gate, Paths, Spec};

#[test]
fn spec_defaults () {

    let spec = Spec::default();

    assert!(spec.inspire.is_empty());
    assert!(spec.tests);
    assert_eq!(spec.max_rounds, 5);
    assert_eq!(spec.max_fixes, 5);

}

#[test]
fn gate_and_agent_defaults () {

    let gate = Gate::default();

    assert_eq!(gate.timeout, 900);
    assert!(gate.command.is_empty());

    let agent = Agent::default();

    assert_eq!(agent.manager, "claude");
    assert_eq!(agent.architects, ["claude"]);

}

#[test]
fn roster_numbers_duplicate_models () {

    let agent = Agent {
        architects: vec!["claude".into(), "claude".into(), "codex".into()],
        ..Agent::default()
    };

    assert_eq!(agent.roster("requires"), ["claude_1", "claude_2", "codex_1"]);

}

#[test]
fn roster_selects_the_phase_and_is_empty_for_unknown () {

    let agent = Agent {
        executors: vec!["codex".into()],
        ..Agent::default()
    };

    assert_eq!(agent.roster("tasks"), ["codex_1"]);
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
fn stem_classification_is_multi_bucket () {

    let agentx = Context::buckets_of_stem("agentx");

    assert!(agentx.contains(&"overview"));
    assert!(agentx.contains(&"contracts"));
    assert_eq!(Context::buckets_of_stem("claude"), ["overview"]);

}

#[test]
fn spec_round_trips_through_toml () {

    let dir = std::env::temp_dir().join(format!("agentx-cfg-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let file = dir.join("Agentx.toml");

    let spec = Spec {
        inspire: "demo".into(),
        tests: false,
        ..Spec::default()
    };

    spec.save(&file).unwrap();

    let loaded = Spec::load(&file).unwrap();

    assert_eq!(loaded.inspire, "demo");
    assert!(!loaded.tests);

    fs::remove_dir_all(&dir).ok();

}
