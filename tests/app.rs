use std::fs;
use std::path::PathBuf;

use agentx::App;

fn temp ( tag: &str ) -> PathBuf {

    let dir = std::env::temp_dir().join(format!("agentx-app-{tag}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    dir

}

#[test]
fn clear_is_a_noop_without_a_cache () {

    let dir = temp("noop");

    assert!(App::clear(&dir).is_ok());

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn clear_removes_files_but_keeps_dirs () {

    let dir = temp("clear");
    let configs = dir.join(".agentx").join("configs");
    fs::create_dir_all(&configs).unwrap();
    fs::write(configs.join("state.json"), "{}").unwrap();

    App::clear(&dir).unwrap();

    assert!(configs.exists());
    assert!(!configs.join("state.json").exists());

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn stop_is_a_noop_when_idle () {

    let dir = temp("stop");

    assert!(App::stop(&dir).is_ok());

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn drain_is_a_noop_when_idle () {

    let dir = temp("drain");

    assert!(App::drain(&dir).is_ok());

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn compose_requires_a_config () {

    let dir = temp("compose");

    let error = App::compose(&dir, None, None, None, false).unwrap_err().to_string();

    assert!(error.contains("Agentx.toml"), "unexpected error: {error}");

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn compose_rejects_an_unknown_role () {

    let dir = temp("compose-role");
    fs::write(dir.join("Agentx.toml"), "").unwrap();

    let error = App::compose(&dir, Some("wizard"), None, None, false).unwrap_err().to_string();

    assert!(error.contains("unknown role"), "unexpected error: {error}");

    fs::remove_dir_all(&dir).ok();

}

#[test]
fn compose_rejects_manager_turns_for_workers () {

    let dir = temp("compose-guard");
    fs::write(dir.join("Agentx.toml"), "").unwrap();

    let error = App::compose(&dir, Some("arch"), Some("discover"), None, false).unwrap_err().to_string();

    assert!(error.contains("manager"), "unexpected error: {error}");

    fs::remove_dir_all(&dir).ok();

}
