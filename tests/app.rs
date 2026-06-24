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
