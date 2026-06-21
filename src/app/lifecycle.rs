use std::path::Path as StdPath;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::{Pid, setpgid};

use crate::config::Paths;
use crate::config::names::CACHE_DIR;
use crate::core::error::AppResult;
use crate::core::support::fs::{Dir, File};
use super::base::{assemble, read_pid, write_pid};
use super::discovery::resolve_root;
use super::orchestrator::Orchestrator;
use super::scaffold;

/// Scaffold the layout in the current directory.
pub fn init ( cwd: &StdPath ) -> AppResult<()> {

    let paths = Paths::new(cwd);
    scaffold::run(&paths)?;
    println!("[agentx] initialised at {}", cwd.display());

    Ok(())

}

/// Resolve the root and run a full cycle, recording the process group so `stop`
/// can reach it.
pub fn start ( cwd: &StdPath ) -> AppResult<()> {

    let root = resolve_root(cwd);
    let paths = Paths::new(&root);
    scaffold::run(&paths)?;

    let config = assemble(&root)?;

    // Lead our own process group so `stop` signals the whole cycle, not the shell.
    let _ = setpgid(Pid::from_raw(0), Pid::from_raw(0));
    write_pid(&paths.pid)?;

    let result = Orchestrator::new(&config).run();

    File::remove(&paths.pid);
    File::remove(&paths.drain);
    File::remove(&paths.active);

    result

}

/// Kill the running cycle and its in-flight worker immediately.
pub fn stop ( cwd: &StdPath ) -> AppResult<()> {

    let paths = Paths::new(&resolve_root(cwd));
    let mut hit = false;

    for pid_file in [&paths.active, &paths.pid] {

        if let Some(pid) = read_pid(pid_file) {
            let _ = killpg(Pid::from_raw(pid), Signal::SIGTERM);
            hit = true;
        }
    }

    File::remove(&paths.active);
    File::remove(&paths.pid);

    if hit {
        println!("[agentx] stopped the running cycle");
    } else {
        println!("[agentx] no running cycle found");
    }

    Ok(())

}

/// Request a clean stop after the current turn.
pub fn drain ( cwd: &StdPath ) -> AppResult<()> {

    let paths = Paths::new(&resolve_root(cwd));
    File::write(&paths.drain, "true\n")?;
    println!("[agentx] drain requested - the run will stop cleanly after the current turn");

    Ok(())

}

/// Delete the `.agentx` cache entirely.
pub fn clean ( cwd: &StdPath ) -> AppResult<()> {

    let cache = resolve_root(cwd).join(CACHE_DIR);
    Dir::remove(&cache);
    println!("[agentx] removed {}", cache.display());

    Ok(())

}
