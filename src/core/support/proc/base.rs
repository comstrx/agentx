use std::io::Read;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;

use crate::core::error::AppResult;
use super::arch::{Output, Proc};

const POLL: Duration = Duration::from_millis(50);

impl Proc {

    /// Run `command` to completion with a wall-clock `timeout` in seconds
    /// (`0` = unbounded). The child leads its own process group, so a timeout
    /// kills the whole subtree — never the orchestrator. `on_spawn` is handed
    /// the child pid the moment it starts (e.g. to record it for `stop`).
    pub fn run ( mut command: Command, timeout: u64, on_spawn: Option<&dyn Fn(i32)> ) -> AppResult<Output> {

        command.process_group(0);

        let mut child = command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let pid = child.id() as i32;

        if let Some(callback) = on_spawn {
            callback(pid);
        }

        // Drain both pipes on their own threads — a full pipe would otherwise
        // block the child and deadlock the poll loop below.
        let mut out = child.stdout.take().expect("stdout is piped");
        let mut err = child.stderr.take().expect("stderr is piped");

        let out_reader = thread::spawn(move || {
            let mut buffer = String::new();
            let _ = out.read_to_string(&mut buffer);
            buffer
        });

        let err_reader = thread::spawn(move || {
            let mut buffer = String::new();
            let _ = err.read_to_string(&mut buffer);
            buffer
        });

        let started = Instant::now();
        let mut code = -1;
        let mut timed_out = false;

        loop {

            match child.try_wait()? {

                Some(status) => {
                    code = status.code().unwrap_or(-1);
                    break;
                }

                None => {

                    if timeout > 0 && started.elapsed().as_secs() >= timeout {
                        let _ = killpg(Pid::from_raw(pid), Signal::SIGKILL);
                        let _ = child.wait();
                        timed_out = true;
                        break;
                    }

                    thread::sleep(POLL);
                }
            }
        }

        let stdout = out_reader.join().unwrap_or_default();
        let stderr = err_reader.join().unwrap_or_default();

        Ok(Output { code, stdout, stderr, timed_out })

    }

}
