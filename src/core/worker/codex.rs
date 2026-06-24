use std::path::Path as StdPath;
use std::process::Command;
use serde_json::Value;

use crate::core::error::{AppError, AppResult};
use super::arch::{Worker, Codex};

impl Codex {

    pub fn new () -> Self {

        Self { session: None, model: String::new(), effort: String::new() }

    }

    pub fn configure ( &mut self, model: &str, effort: &str ) {

        self.model = model.trim().to_string();
        self.effort = effort.trim().to_string();

    }

    pub fn start ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String> {

        let mut command = Command::new("codex");

        command.current_dir(cwd);
        command.arg("exec");

        if !self.model.is_empty() { command.arg("-m").arg(&self.model); }

        if !self.effort.is_empty() { command.arg("-c").arg(format!("model_reasoning_effort={}", self.effort)); }

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

            let kind = event.get("type").and_then(Value::as_str).unwrap_or_default();

            if kind == "error" || kind.ends_with(".error") || kind.ends_with(".failed") {

                let detail = event.get("message").and_then(Value::as_str)
                    .or_else(|| event.get("error").and_then(Value::as_str))
                    .filter(|text| !text.trim().is_empty())
                    .unwrap_or("codex reported an error");

                return Err(AppError::command("codex", 0, detail));

            }

            if kind == "thread.started" && let Some(id) = event.get("thread_id").and_then(Value::as_str) && !id.is_empty() {

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
