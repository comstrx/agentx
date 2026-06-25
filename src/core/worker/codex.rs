use std::path::Path as StdPath;
use std::process::Command;
use std::time::{Duration, Instant};
use serde_json::{Value, json};

use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::File;
use crate::core::support::proc::{Proc, Recv, Stream};
use super::arch::{Backend, Codex};

const STEP: Duration = Duration::from_millis(100);
const HANDSHAKE: u64 = 60;

impl Codex {

    pub fn new () -> Self {

        Self { session: None, model: String::new(), effort: String::new(), stream: None, seq: 0 }

    }

    fn open ( &mut self, cwd: &StdPath, pid_file: Option<&StdPath> ) -> AppResult<Stream> {

        let mut command = Command::new("codex");
        command.current_dir(cwd).arg("mcp-server");

        let mut stream = match pid_file {
            Some(path) => {

                let record = |pid: i32| { let _ = File::write(path, &pid.to_string()); };
                Proc::stream(command, Some(&record as &dyn Fn(i32)))?

            }
            None => Proc::stream(command, None)?,
        };

        let id = self.tick();
        let init = json!({ "jsonrpc": "2.0", "id": id, "method": "initialize", "params": { "protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": { "name": "agentx", "version": "0.2.0" } } });

        stream.send(&init.to_string())?;
        Self::read_reply(&stream, id, HANDSHAKE)?;
        stream.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }).to_string())?;

        Ok(stream)

    }

    fn request ( &mut self, prompt: &str, cwd: &StdPath ) -> Value {

        let id = self.tick();

        if let Some(thread) = &self.session && !thread.is_empty() {

            return json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call", "params": { "name": "codex-reply", "arguments": { "threadId": thread, "prompt": prompt } } });

        }

        let mut arguments = json!({ "prompt": prompt, "cwd": cwd.display().to_string(), "sandbox": "danger-full-access", "approval-policy": "never" });

        if !self.model.is_empty() { arguments["model"] = json!(self.model); }

        if !self.effort.is_empty() { arguments["config"] = json!({ "model_reasoning_effort": self.effort }); }

        json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call", "params": { "name": "codex", "arguments": arguments } })

    }

    fn roundtrip ( stream: &mut Stream, call: Value, timeout: u64 ) -> AppResult<String> {

        let id = call.get("id").and_then(Value::as_i64).unwrap_or_default();

        stream.send(&call.to_string())?;

        let result = Self::read_reply(stream, id, timeout)?;

        if result.get("isError").and_then(Value::as_bool) == Some(true) {

            let detail = Self::content_text(&result);

            return Err(AppError::command("codex", 0, if detail.is_empty() { "codex reported an error".to_string() } else { detail }));

        }

        let thread = result.get("structuredContent").and_then(|value| value.get("threadId")).and_then(Value::as_str).unwrap_or_default();

        Ok(thread.to_string())

    }

    fn read_reply ( stream: &Stream, id: i64, timeout: u64 ) -> AppResult<Value> {

        let started = Instant::now();

        loop {

            if Proc::aborted() {

                stream.kill();
                return Err(AppError::message("interrupted by signal"));

            }

            if timeout > 0 && started.elapsed().as_secs() >= timeout {

                stream.kill();
                return Err(AppError::timeout("codex", timeout));

            }

            match stream.recv(STEP) {
                Recv::Line(line) => {

                    let event: Value = match serde_json::from_str(line.trim()) {
                        Ok(value) => value,
                        Err(_) => continue,
                    };

                    if event.get("id").and_then(Value::as_i64) != Some(id) { continue; }

                    if let Some(error) = event.get("error") {

                        let detail = error.get("message").and_then(Value::as_str).unwrap_or("codex error");
                        return Err(AppError::command("codex", -1, detail));

                    }

                    return Ok(event.get("result").cloned().unwrap_or(Value::Null));

                }
                Recv::Idle => continue,
                Recv::Closed => {

                    let tail = stream.stderr();
                    let detail = if tail.trim().is_empty() { "codex stream closed unexpectedly".to_string() } else { tail.trim().to_string() };

                    return Err(AppError::command("codex", -1, detail));

                }
            }

        }

    }

    fn content_text ( result: &Value ) -> String {

        if let Some(text) = result.get("structuredContent").and_then(|value| value.get("content")).and_then(Value::as_str) {

            return text.to_string();

        }

        result.get("content").and_then(Value::as_array).and_then(|items| items.first())
            .and_then(|item| item.get("text")).and_then(Value::as_str).unwrap_or_default().to_string()

    }

    fn tick ( &mut self ) -> i64 {

        self.seq += 1;
        self.seq

    }

}

impl Backend for Codex {

    fn configure ( &mut self, model: &str, effort: &str ) {

        self.model = model.trim().to_string();
        self.effort = effort.trim().to_string();

    }

    fn turn ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String> {

        if self.stream.is_none() {

            let stream = self.open(cwd, pid_file)?;
            self.stream = Some(stream);

        }

        let call = self.request(prompt, cwd);

        let outcome = match self.stream.as_mut() {
            Some(stream) => Self::roundtrip(stream, call, timeout),
            None => Err(AppError::message("codex stream not initialised")),
        };

        match outcome {
            Ok(thread) => {

                if !thread.is_empty() { self.session = Some(thread.clone()); }

                Ok(thread)

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
