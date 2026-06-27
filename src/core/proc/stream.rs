use std::io::{BufRead, BufReader, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;

use crate::core::error::{AppError, AppResult};
use super::arch::{Proc, Recv, Stream};

impl Proc {

    pub fn stream ( mut command: Command, on_spawn: Option<&dyn Fn(i32)> ) -> AppResult<Stream> {

        command.process_group(0);
        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = command.spawn()?;
        let pid = child.id() as i32;

        if let Some(callback) = on_spawn {

            callback(pid);

        }

        let Some(stdin) = child.stdin.take() else { return Err(AppError::message("child stdin was not piped")); };
        let Some(stdout) = child.stdout.take() else { return Err(AppError::message("child stdout was not piped")); };
        let Some(stderr) = child.stderr.take() else { return Err(AppError::message("child stderr was not piped")); };

        let ( sender, lines ) = mpsc::channel();

        thread::spawn(move || {

            let reader = BufReader::new(stdout);

            for line in reader.lines() {

                match line {
                    Ok(text) => if sender.send(text).is_err() { break; },
                    Err(_) => break,
                }

            }

        });

        let errors = Arc::new(Mutex::new(String::new()));
        let sink = Arc::clone(&errors);

        thread::spawn(move || {

            let reader = BufReader::new(stderr);

            for line in reader.lines() {

                let Ok(text) = line else { break; };

                if let Ok(mut guard) = sink.lock() {

                    if !guard.is_empty() { guard.push('\n'); }

                    guard.push_str(&text);

                    if guard.len() > 8192 {

                        let mut cut = guard.len() - 4096;

                        while cut < guard.len() && !guard.is_char_boundary(cut) { cut += 1; }

                        *guard = guard.split_off(cut);

                    }

                }

            }

        });

        Ok(Stream { child, stdin, lines, errors, pid })

    }

}

impl Stream {

    pub fn send ( &mut self, line: &str ) -> AppResult<()> {

        self.stdin.write_all(line.as_bytes())?;
        self.stdin.write_all(b"\n")?;
        self.stdin.flush()?;

        Ok(())

    }

    pub fn recv ( &self, wait: Duration ) -> Recv {

        match self.lines.recv_timeout(wait) {
            Ok(line) => Recv::Line(line),
            Err(RecvTimeoutError::Timeout) => Recv::Idle,
            Err(RecvTimeoutError::Disconnected) => Recv::Closed,
        }

    }

    pub fn stderr ( &self ) -> String {

        self.errors.lock().map(|guard| guard.clone()).unwrap_or_default()

    }

    pub fn pid ( &self ) -> i32 {

        self.pid

    }

    pub fn kill ( &self ) {

        let _ = killpg(Pid::from_raw(self.pid), Signal::SIGKILL);

    }

}

impl Drop for Stream {

    fn drop ( &mut self ) {

        let _ = killpg(Pid::from_raw(self.pid), Signal::SIGKILL);
        let _ = self.child.wait();

    }

}
