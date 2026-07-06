use std::fs;
use std::path::Path;

use agentx::config::{Agent, Context, Gate, Member, Options, Paths, Seats, Spec};

fn write_agent ( tag: &str, body: &str ) -> ( std::path::PathBuf, std::path::PathBuf ) {

    let dir = std::env::temp_dir().join(format!("agentx-agent-{tag}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let file = dir.join("Agentx.toml");
    fs::write(&file, body).unwrap();

    ( dir, file )

}

#[test]
fn project_and_option_defaults () {

    let spec = Spec::default();

    assert!(spec.inspire.is_empty());
    assert_eq!(spec.stage, "dev");
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
    assert_eq!(agent.manager.agent, "claude");
    assert!(agent.manager.model.is_empty() && agent.manager.effort.is_empty());
    assert_eq!(agent.roster("requires"), ["claude_1"]);
    assert_eq!(agent.roster("tasks"), ["claude_1"]);

}

fn seats ( names: &[&str] ) -> Seats {

    Seats { members: names.iter().map(|name| Member::backend(name)).collect(), seq: true }

}

#[test]
fn roster_numbers_duplicate_models () {

    let agent = Agent {
        requires: seats(&["claude", "claude", "codex"]),
        ..Agent::default()
    };

    assert_eq!(agent.roster("requires"), ["claude_1", "claude_2", "codex_1"]);

}

#[test]
fn roster_selects_the_phase_and_is_empty_for_unknown () {

    let agent = Agent {
        tasks: seats(&["codex"]),
        benches: seats(&["claude", "codex"]),
        ..Agent::default()
    };

    assert_eq!(agent.roster("tasks"), ["codex_1"]);
    assert_eq!(agent.roster("benches"), ["claude_1", "codex_1"]);
    assert!(agent.roster("nope").is_empty());

}

#[test]
fn member_parse_forms_bare_table_and_lists () {

    let ( dir, file ) = write_agent("forms", "\
[agent]
manager  = { agent = \"claude\", model = \"fable-5\", effort = \"ultra\" }
requires = [ { agent = \"claude\", model = \"opus-4.8\", effort = \"high\" }, { agent = \"claude\", model = \"fable-5\", effort = \"max\" } ]
tasks    = \"claude\"
audits   = { agent = \"claude\", effort = \"max\" }
tests    = [ \"claude\", { agent = \"codex\", model = \"gpt-5.5-codex\" } ]
");

    let doc = Spec::document(&file).unwrap();

    assert_eq!(doc.agent.manager.agent, "claude");
    assert_eq!(doc.agent.manager.model, "fable-5");
    assert_eq!(doc.agent.manager.effort, "ultra");

    assert_eq!(doc.agent.requires.members.len(), 2);
    assert_eq!(doc.agent.requires.members[0].model, "opus-4.8");
    assert_eq!(doc.agent.requires.members[1].effort, "max");
    assert!(doc.agent.requires.seq);

    assert_eq!(doc.agent.tasks.members.len(), 1);
    assert!(doc.agent.tasks.members[0].model.is_empty());
    assert!(!doc.agent.tasks.seq);

    assert_eq!(doc.agent.audits.members.len(), 1);
    assert_eq!(doc.agent.audits.members[0].effort, "max");
    assert!(!doc.agent.audits.seq);

    assert_eq!(doc.agent.tests.members.len(), 2);
    assert_eq!(doc.agent.tests.members[1].agent, "codex");
    assert_eq!(doc.agent.tests.members[1].model, "gpt-5.5-codex");

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn manager_cardinality_and_missing_agent_error () {

    let ( d1, f1 ) = write_agent("mgr2", "[agent]\nmanager = [ \"claude\", \"codex\" ]\n");
    assert!(Spec::document(&f1).is_err());
    fs::remove_dir_all(&d1).ok();

    let ( d2, f2 ) = write_agent("noagent", "[agent]\nrequires = [ { model = \"opus\" } ]\n");
    assert!(Spec::document(&f2).is_err());
    fs::remove_dir_all(&d2).ok();

    let ( d3, f3 ) = write_agent("mgr1list", "[agent]\nmanager = [ \"claude\" ]\n");
    assert_eq!(Spec::document(&f3).unwrap().agent.manager.agent, "claude");
    fs::remove_dir_all(&d3).ok();

}

#[test]
fn engine_precedence_member_then_section_then_const () {

    let ( d1, f1 ) = write_agent("prec", "\
[agent]
requires = [ { agent = \"claude\", effort = \"max\" } ]
[claude]
model = \"opus-4.8\"
");

    let doc = Spec::document(&f1).unwrap();
    let ( model, effort ) = doc.resolve_member(&doc.agent.requires.members[0]);

    assert_eq!(model, "opus-4.8");
    assert_eq!(effort, "max");
    fs::remove_dir_all(&d1).ok();

    let ( d2, f2 ) = write_agent("prec2", "[agent]\ntasks = \"claude\"\n");
    let doc2 = Spec::document(&f2).unwrap();
    let ( model2, effort2 ) = doc2.resolve_member(&doc2.agent.tasks.members[0]);

    assert_eq!(model2, agentx::config::base::consts::CLAUDE_MODEL);
    assert_eq!(effort2, agentx::config::base::consts::CLAUDE_EFFORT);
    fs::remove_dir_all(&d2).ok();

}

#[test]
fn agent_section_round_trips_without_data_loss () {

    let ( dir, file ) = write_agent("rt", "\
[agent]
manager  = { agent = \"claude\", model = \"fable-5\", effort = \"ultra\" }
requires = [ \"claude\", { agent = \"claude\", model = \"opus-4.8\" } ]
tests    = [ \"claude\", { agent = \"codex\", model = \"gpt-5.5-codex\" } ]
");

    Spec::document(&file).unwrap().save(&file).unwrap();
    let after = Spec::document(&file).unwrap();

    assert_eq!(after.agent.manager.model, "fable-5");
    assert_eq!(after.agent.manager.effort, "ultra");
    assert_eq!(after.agent.requires.members.len(), 2);
    assert_eq!(after.agent.requires.members[1].model, "opus-4.8");
    assert_eq!(after.agent.tests.members[1].agent, "codex");
    assert_eq!(after.agent.tests.members[1].model, "gpt-5.5-codex");

    let bytes1 = fs::read_to_string(&file).unwrap();
    after.save(&file).unwrap();
    let bytes2 = fs::read_to_string(&file).unwrap();

    assert_eq!(bytes1, bytes2);

    fs::remove_dir_all(&dir).ok();

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
fn classification_skips_direct_bucket_placeholders () {

    let dir = std::env::temp_dir().join(format!("agentx-buckets-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();

    fs::create_dir_all(dir.join("designs/deep")).unwrap();
    fs::create_dir_all(dir.join("contracts")).unwrap();
    fs::create_dir_all(dir.join("references")).unwrap();

    fs::write(dir.join("designs/.gitkeep"), "").unwrap();
    fs::write(dir.join("designs/deep/.gitkeep"), "").unwrap();
    fs::write(dir.join("designs/home.png"), "").unwrap();
    fs::write(dir.join("contracts/.gitkeep"), "").unwrap();
    fs::write(dir.join("contracts/law.md"), "law").unwrap();
    fs::write(dir.join("references/.gitkeep"), "").unwrap();
    fs::write(dir.join("references/prior.txt"), "prior").unwrap();

    let mut context = Context::default();
    context.collect(&dir, true);

    let designs = context.bucket("designs");

    assert!(!designs.contains(&dir.join("designs/.gitkeep")));
    assert!(designs.contains(&dir.join("designs/deep/.gitkeep")));
    assert!(designs.contains(&dir.join("designs/home.png")));

    assert_eq!(context.bucket("contracts"), [dir.join("contracts/law.md")]);
    assert_eq!(context.bucket("references"), [dir.join("references/prior.txt")]);

    fs::remove_dir_all(&dir).ok();

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

#[test]
fn saves_keep_the_house_shape () {

    let ( dir, file ) = write_agent("shape", "\
[project]
inspire = \"laravel-saas\"
[gate]
command = \"composer verify\"
[agent]
requires = [ \"claude\", { agent = \"codex\", model = \"gpt-5.5\" } ]
");

    Spec::document(&file).unwrap().save(&file).unwrap();

    let body = fs::read_to_string(&file).unwrap();

    assert!(body.contains("inspire       = \"laravel-saas\""));
    assert!(body.contains("stage         = \"dev\""));
    assert!(body.contains("command = \"composer verify\""));
    assert!(body.contains("requires   = [ \"claude\", { agent = \"codex\", model = \"gpt-5.5\" } ]"));
    assert!(!body.contains("[[agent.requires]]"));
    assert!(!body.contains("[claude]"));
    assert!(!body.contains("[codex]"));

    fs::remove_dir_all(&dir).ok();

}
