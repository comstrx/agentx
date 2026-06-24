use std::path::Path as StdPath;
use std::process::Command;
use serde_json::Value;

use crate::core::error::{AppError, AppResult};
use super::arch::{Worker, Claude};

impl Claude {

    pub fn new () -> Self {

        Self { session: None, model: String::new(), effort: String::new() }

    }

    pub fn configure ( &mut self, model: &str, effort: &str ) {

        self.model = model.trim().to_string();
        self.effort = effort.trim().to_string();

    }

    pub fn start ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String> {

        let mut command = Command::new("claude");

        command.current_dir(cwd);
        command.arg("-p").arg(prompt);
        command.args(["--output-format", "json", "--permission-mode", "bypassPermissions"]);

        if !self.model.is_empty() { command.arg("--model").arg(&self.model); }

        if !self.effort.is_empty() { command.arg("--effort").arg(&self.effort); }

        if let Some(id) = &self.session && !id.is_empty() {

            command.arg("--resume").arg(id);

        }

        let stdout = Worker::capture(command, timeout, pid_file, "claude")?;
        let parsed: Value = serde_json::from_str(stdout.trim()).unwrap_or(Value::Null);

        if parsed.get("is_error").and_then(Value::as_bool) == Some(true) {

            let detail = parsed.get("result").and_then(Value::as_str).filter(|text| !text.trim().is_empty()).unwrap_or("claude reported an error");

            return Err(AppError::command("claude", 0, detail));

        }

        let next = parsed
            .get("session_id")
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
            .map(str::to_string)
            .or_else(|| self.session.clone())
            .unwrap_or_default();

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
