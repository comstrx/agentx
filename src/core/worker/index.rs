use std::path::{Path as StdPath, PathBuf};
use std::process::Command;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;

use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::File;
use crate::core::support::proc::Proc;
use crate::core::support::str::Str;
use crate::core::support::text::Text;
use super::arch::{Worker, Backend, Fault};

impl Worker {

    pub fn fault ( error: &AppError ) -> Fault {

        let text = match error {
            AppError::Timeout { .. } => return Fault::Transient,
            AppError::Command { stderr, .. } => stderr.to_lowercase(),
            other => other.to_string().to_lowercase(),
        };

        if Self::names(&text, &["usage limit", "quota", "credit", "billing", "insufficient", "payment", "out of credits", "exceeded your"]) { return Fault::Exhausted; }

        if Self::names(&text, &["no conversation", "conversation not found", "session not found", "no session", "unknown session", "invalid session", "could not resume", "no such session", "resume"]) { return Fault::Session; }

        if Self::names(&text, &["api key", "unauthorized", "authentication", "not logged in", " 401", " 403", "invalid model", "model not found", "unknown model", "no such model", "permission denied", "forbidden", "no such file", "executable file not found", "command not found", "cannot find"]) { return Fault::Fatal; }

        Fault::Transient

    }

    fn names ( text: &str, needles: &[&str] ) -> bool {

        needles.iter().any(|needle| text.contains(needle))

    }

    pub fn new ( model: &str ) -> Self {

        Self { backend: Backend::select(model), cwd: PathBuf::from("."), timeout: 0, pid_file: None }

    }

    pub fn set_model ( &mut self, model: &str ) -> &mut Self {

        self.backend = Backend::select(model);

        self

    }

    pub fn engine ( &mut self, model: &str, effort: &str ) -> &mut Self {

        match &mut self.backend {
            Backend::Claude(agent) => agent.configure(model, effort),
            Backend::Codex(agent) => agent.configure(model, effort),
        }

        self

    }

    pub fn cwd ( &mut self, path: &StdPath ) -> &mut Self {

        self.cwd = path.to_path_buf();

        self

    }

    pub fn timeout ( &mut self, seconds: u64 ) -> &mut Self {

        self.timeout = seconds;

        self

    }

    pub fn pid_file ( &mut self, path: &StdPath ) -> &mut Self {

        self.pid_file = Some(path.to_path_buf());

        self

    }

    pub fn set_session ( &mut self, id: &str ) -> &mut Self {

        match &mut self.backend {
            Backend::Claude(agent) => agent.set_session(id),
            Backend::Codex(agent) => agent.set_session(id),
        }

        self

    }

    pub fn clear ( &mut self ) -> &mut Self {

        match &mut self.backend {
            Backend::Claude(agent) => agent.clear(),
            Backend::Codex(agent) => agent.clear(),
        }

        self

    }

    pub fn session ( &self ) -> Option<&str> {

        match &self.backend {
            Backend::Claude(agent) => agent.session(),
            Backend::Codex(agent) => agent.session(),
        }

    }

    pub fn start ( &mut self, prompt: &str ) -> AppResult<String> {

        let cwd = self.cwd.clone();
        let timeout = self.timeout;
        let pid_file = self.pid_file.clone();

        match &mut self.backend {
            Backend::Claude(agent) => agent.start(prompt, &cwd, timeout, pid_file.as_deref()),
            Backend::Codex(agent) => agent.start(prompt, &cwd, timeout, pid_file.as_deref()),
        }

    }

    pub fn stop ( &self ) {

        if let Some(path) = &self.pid_file && let Some(pid) = Proc::read_pid(path) {

            let _ = killpg(Pid::from_raw(pid), Signal::SIGTERM);
            File::remove(path);

        }

    }

    pub(crate) fn capture ( command: Command, timeout: u64, pid_file: Option<&StdPath>, name: &str ) -> AppResult<String> {

        let output = match pid_file {
            Some(path) => {

                let record = |pid: i32| { let _ = File::write(path, &pid.to_string()); };
                let result = Proc::run(command, timeout, Some(&record as &dyn Fn(i32)));
                File::remove(path);

                result?

            }
            None => Proc::run(command, timeout, None)?,
        };

        if output.timed_out { return Err(AppError::timeout(name, timeout)); }

        if output.code != 0 { return Err(AppError::command(name, output.code, Str::take(Text::last_line(&output.stderr), 200))); }

        Ok(output.stdout)

    }

}
