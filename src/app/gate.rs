use std::path::Path as StdPath;
use std::process::Command;

use crate::core::error::AppResult;
use crate::core::support::fs::File;
use crate::core::support::proc::Proc;

/// Run the gate command under `sh -c`, writing combined output to `log`.
/// An empty command is treated as a pass. Returns whether the gate is green.
pub fn run ( gate_cmd: &str, cwd: &StdPath, timeout: u64, log: &StdPath ) -> AppResult<bool> {

    if gate_cmd.trim().is_empty() {
        File::write(log, "no gate command set; gate skipped")?;
        return Ok(true);
    }

    let mut command = Command::new("sh");
    command.arg("-c").arg(gate_cmd).current_dir(cwd);

    let output = Proc::run(command, timeout, None)?;
    let body = format!("{}{}", output.stdout, output.stderr);
    File::write(log, &body)?;

    Ok(output.code == 0 && !output.timed_out)

}
