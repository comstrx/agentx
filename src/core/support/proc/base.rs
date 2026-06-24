use std::{thread, io::Read, path::Path as StdPath, time::Instant, process::{Command, Stdio}, os::unix::process::CommandExt};
use std::sync::atomic::{AtomicBool, Ordering};
use nix::{unistd::Pid, sys::signal::{kill, killpg, Signal}};

use crate::core::error::AppResult;
use crate::core::support::fs::File;
use super::arch::{Output, Proc, POLL};

static ABORT: AtomicBool = AtomicBool::new(false);

impl Proc {

    pub fn request_abort () {

        ABORT.store(true, Ordering::SeqCst);

    }

    pub fn aborted () -> bool {

        ABORT.load(Ordering::SeqCst)

    }

    pub fn run ( mut command: Command, timeout: u64, on_spawn: Option<&dyn Fn(i32)> ) -> AppResult<Output> {

        command.process_group(0);

        let mut child = command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let pid = child.id() as i32;

        if let Some(callback) = on_spawn {

            callback(pid);

        }

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

                    if Self::aborted() {

                        let _ = killpg(Pid::from_raw(pid), Signal::SIGKILL);
                        let _ = child.wait();

                        code = 130;
                        break;

                    }

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

    pub fn shell ( command: impl AsRef<str>, timeout: u64 ) -> AppResult<Output> {

        let mut shell = Command::new("sh");
        shell.arg("-c").arg(command.as_ref());

        Self::run(shell, timeout, None)

    }

    pub fn command ( program: impl AsRef<str>, args: &[&str], timeout: u64 ) -> AppResult<Output> {

        let mut command = Command::new(program.as_ref());
        command.args(args);

        Self::run(command, timeout, None)

    }

    pub fn capture ( program: impl AsRef<str>, args: &[&str] ) -> AppResult<Output> {

        Self::command(program, args, 0)

    }

    pub fn shell_in ( command: impl AsRef<str>, cwd: &StdPath, timeout: u64 ) -> AppResult<Output> {

        let mut shell = Command::new("sh");
        shell.arg("-c").arg(command.as_ref()).current_dir(cwd);

        Self::run(shell, timeout, None)

    }

    pub fn detach ( mut command: Command, log: &StdPath ) -> AppResult<i32> {

        let out = std::fs::File::create(log)?;
        let err = out.try_clone()?;

        command.stdin(Stdio::null()).stdout(out).stderr(err).process_group(0);

        let child = command.spawn()?;

        Ok(child.id() as i32)

    }

    pub fn pid () -> u32 {

        std::process::id()

    }

    pub fn write_pid ( path: &StdPath ) -> AppResult<()> {

        File::write(path, &Self::pid().to_string())

    }

    pub fn read_pid ( path: &StdPath ) -> Option<i32> {

        File::read_opt(path).and_then(|body| body.trim().parse().ok())

    }

    pub fn is_alive ( pid: i32 ) -> bool {

        kill(Pid::from_raw(pid), None).is_ok()

    }

}
