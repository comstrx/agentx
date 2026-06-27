use std::path::{Path as StdPath, PathBuf};

use crate::core::error::{AppError, AppResult};
use super::arch::{Worker, Fault};

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

        Self { backend: Self::make(model), cwd: PathBuf::from("."), timeout: 0, pid_file: None }

    }

    pub fn set_model ( &mut self, model: &str ) -> &mut Self {

        self.backend = Self::make(model);

        self

    }

    pub fn engine ( &mut self, model: &str, effort: &str ) -> &mut Self {

        self.backend.configure(model, effort);

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

        self.backend.set_session(id);

        self

    }

    pub fn clear ( &mut self ) -> &mut Self {

        self.backend.clear();

        self

    }

    pub fn session ( &self ) -> Option<&str> {

        self.backend.session()

    }

    pub fn turn ( &mut self, prompt: &str ) -> AppResult<String> {

        let cwd = self.cwd.clone();
        let timeout = self.timeout;
        let pid_file = self.pid_file.clone();

        self.backend.turn(prompt, &cwd, timeout, pid_file.as_deref())

    }

}
