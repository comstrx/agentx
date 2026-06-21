use std::path::Path as StdPath;
use std::process::Command;

use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::File;
use crate::core::support::proc::{Output, Proc};

/// One agent turn's result: the text produced and the (possibly new) session id.
pub struct Reply {

    pub text: String,
    pub session: String,
}

/// A backend that runs one agent turn as a subprocess.
pub trait Worker {

    fn invoke ( &self, prompt: &str, session: Option<&str>, cwd: &StdPath, timeout: u64, active: &StdPath ) -> AppResult<Reply>;
}

/// Strip the roster suffix: `claude_2` -> `claude`.
pub fn model_of ( agent: &str ) -> &str {

    agent.rsplit_once('_').map_or(agent, |( model, _ )| model)

}

/// Run a worker command, recording the child pid to `active` for the turn so
/// `stop` can reach it, and mapping timeout / non-zero exit to typed errors.
pub fn capture ( command: Command, timeout: u64, active: &StdPath, name: &str ) -> AppResult<Output> {

    let record = |pid: i32| { let _ = File::write(active, &pid.to_string()); };
    let output = Proc::run(command, timeout, Some(&record as &dyn Fn(i32)))?;
    File::remove(active);

    if output.timed_out {
        return Err(AppError::timeout(name, timeout));
    }

    if output.code != 0 {
        return Err(AppError::command(name, output.code, brief(&output.stderr)));
    }

    Ok(output)

}

fn brief ( message: &str ) -> String {

    message.trim().lines().next_back().unwrap_or_default().chars().take(200).collect()

}
