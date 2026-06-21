use std::path::Path as StdPath;
use std::process::Command;
use serde_json::Value;

use crate::core::error::AppResult;
use super::arch::{Reply, Worker, capture};

/// The `codex exec` CLI with JSONL event output.
pub struct Codex;

impl Worker for Codex {

    fn invoke ( &self, prompt: &str, session: Option<&str>, cwd: &StdPath, timeout: u64, active: &StdPath ) -> AppResult<Reply> {

        let mut command = Command::new("codex");

        command.current_dir(cwd);
        command.arg("exec");

        if let Some(id) = session {
            command.arg("resume").arg(id);
        }

        command.arg("--json").arg(prompt);

        let output = capture(command, timeout, active, "codex")?;

        let mut text: Vec<String> = Vec::new();
        let mut next = session.unwrap_or_default().to_string();

        for line in output.stdout.lines() {

            let event: Value = match serde_json::from_str(line) {
                Ok(value) => value,
                Err(_) => continue,
            };

            let kind = event.get("type").and_then(Value::as_str);

            if kind == Some("thread.started")
                && let Some(id) = event.get("thread_id").and_then(Value::as_str)
            {
                next = id.to_string();
            }

            if kind == Some("item.completed") {

                let item = event.get("item");

                if item.and_then(|value| value.get("type")).and_then(Value::as_str) == Some("agent_message")
                    && let Some(message) = item.and_then(|value| value.get("text")).and_then(Value::as_str)
                {
                    text.push(message.to_string());
                }
            }
        }

        Ok(Reply { text: text.join("\n"), session: next })

    }

}
