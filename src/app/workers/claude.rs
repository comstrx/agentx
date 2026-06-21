use std::path::Path as StdPath;
use std::process::Command;
use serde_json::Value;

use crate::core::error::AppResult;
use super::arch::{Reply, Worker, capture};

/// The `claude` CLI in non-interactive print mode with JSON output.
pub struct Claude;

impl Worker for Claude {

    fn invoke ( &self, prompt: &str, session: Option<&str>, cwd: &StdPath, timeout: u64, active: &StdPath ) -> AppResult<Reply> {

        let mut command = Command::new("claude");

        command.current_dir(cwd);
        command.arg("-p").arg(prompt);
        command.args(["--output-format", "json", "--permission-mode", "bypassPermissions"]);

        if let Some(id) = session {
            command.arg("--resume").arg(id);
        }

        let output = capture(command, timeout, active, "claude")?;
        let parsed: Value = serde_json::from_str(output.stdout.trim()).unwrap_or(Value::Null);

        let text = parsed.get("result").and_then(Value::as_str).map(str::to_string).unwrap_or(output.stdout);
        let next = parsed
            .get("session_id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| session.unwrap_or_default().to_string());

        Ok(Reply { text, session: next })

    }

}
