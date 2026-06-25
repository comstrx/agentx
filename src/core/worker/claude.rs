use std::path::Path as StdPath;
use std::process::Command;
use std::time::{Duration, Instant};
use serde_json::{Value, json};

use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::File;
use crate::core::support::proc::{Proc, Recv, Stream};
use super::arch::{Backend, Claude};

const STEP: Duration = Duration::from_millis(100);

impl Claude {

    pub fn new () -> Self {

        Self { session: None, model: String::new(), effort: String::new(), stream: None }

    }

    fn open ( &self, cwd: &StdPath, pid_file: Option<&StdPath> ) -> AppResult<Stream> {

        let mut command = Command::new("claude");

        command.current_dir(cwd);
        command.args(["-p", "--input-format", "stream-json", "--output-format", "stream-json", "--verbose", "--permission-mode", "bypassPermissions"]);

        if !self.model.is_empty() { command.arg("--model").arg(&self.model); }

        if !self.effort.is_empty() { command.arg("--effort").arg(&self.effort); }

        if let Some(id) = &self.session && !id.is_empty() {

            command.arg("--resume").arg(id);

        }

        match pid_file {
            Some(path) => {

                let record = |pid: i32| { let _ = File::write(path, &pid.to_string()); };
                Proc::stream(command, Some(&record as &dyn Fn(i32)))

            }
            None => Proc::stream(command, None),
        }

    }

    fn exchange ( stream: &mut Stream, prompt: &str, timeout: u64, previous: Option<&str> ) -> AppResult<String> {

        let message = json!({ "type": "user", "message": { "role": "user", "content": prompt } });
        stream.send(&message.to_string())?;

        let started = Instant::now();
        let mut session = previous.unwrap_or_default().to_string();

        loop {

            if Proc::aborted() {

                stream.kill();
                return Err(AppError::message("interrupted by signal"));

            }

            if timeout > 0 && started.elapsed().as_secs() >= timeout {

                stream.kill();
                return Err(AppError::timeout("claude", timeout));

            }

            match stream.recv(STEP) {
                Recv::Line(line) => {

                    let event: Value = match serde_json::from_str(line.trim()) {
                        Ok(value) => value,
                        Err(_) => continue,
                    };

                    if let Some(id) = event.get("session_id").and_then(Value::as_str).filter(|value| !value.is_empty()) {

                        session = id.to_string();

                    }

                    if event.get("type").and_then(Value::as_str) == Some("result") {

                        if event.get("is_error").and_then(Value::as_bool) == Some(true) {

                            let detail = event.get("result").and_then(Value::as_str).filter(|text| !text.trim().is_empty()).unwrap_or("claude reported an error");
                            return Err(AppError::command("claude", 0, detail));

                        }

                        return Ok(session);

                    }

                }
                Recv::Idle => continue,
                Recv::Closed => {

                    let tail = stream.stderr();
                    let detail = if tail.trim().is_empty() { "claude stream closed unexpectedly".to_string() } else { tail.trim().to_string() };

                    return Err(AppError::command("claude", -1, detail));

                }
            }

        }

    }

}

impl Backend for Claude {

    fn configure ( &mut self, model: &str, effort: &str ) {

        self.model = model.trim().to_string();
        self.effort = effort.trim().to_string();

    }

    fn turn ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String> {

        if self.stream.is_none() {

            self.stream = Some(self.open(cwd, pid_file)?);

        }

        let previous = self.session.clone();

        let outcome = match self.stream.as_mut() {
            Some(stream) => Self::exchange(stream, prompt, timeout, previous.as_deref()),
            None => Err(AppError::message("claude stream not initialised")),
        };

        match outcome {
            Ok(session) => {

                if !session.is_empty() { self.session = Some(session.clone()); }

                Ok(session)

            }
            Err(error) => {

                self.stream = None;

                Err(error)

            }
        }

    }

    fn set_session ( &mut self, id: &str ) {

        self.session = Some(id.to_string());

    }

    fn clear ( &mut self ) {

        self.session = None;
        self.stream = None;

    }

    fn session ( &self ) -> Option<&str> {

        self.session.as_deref()

    }

}
