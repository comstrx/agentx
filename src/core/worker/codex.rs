use std::path::Path as StdPath;
use std::process::Command;
use serde_json::Value;

use crate::core::error::AppResult;
use super::arch::{Worker, Codex};

impl Codex {

    pub fn new () -> Self {

        Self { session: None }

    }

    pub fn start ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String> {

        let mut command = Command::new("codex");

        command.current_dir(cwd);
        command.arg("exec");

        if let Some(id) = &self.session && !id.is_empty() {

            command.arg("resume").arg(id);

        }

        command.arg("--json").arg(prompt);

        let stdout = Worker::capture(command, timeout, pid_file, "codex")?;
        let mut next = self.session.clone().unwrap_or_default();

        for line in stdout.lines() {

            let event: Value = match serde_json::from_str(line) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if event.get("type").and_then(Value::as_str) == Some("thread.started") && let Some(id) = event.get("thread_id").and_then(Value::as_str) && !id.is_empty() {

                next = id.to_string();

            }

        }

        if !next.is_empty() { self.session = Some(next.clone()); }

        Ok(next)

    }

    pub fn set_session ( &mut self, id: &str ) {

        self.session = Some(id.to_string());

    }

    pub fn clear ( &mut self ) {

        self.session = None;

    }

    pub fn session ( &self ) -> Option<&str> {

        self.session.as_deref()

    }

}
