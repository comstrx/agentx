use std::collections::HashMap;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::fs::File;
use crate::core::parse::Json;
use crate::core::proc::Proc;
use crate::app::{Flow, Halt, Orchestrator, Status};

impl Orchestrator {

    pub(super) fn mark_blocked ( &mut self, name: &str ) {

        if !self.journey.blocked.iter().any(|item| item == name) {

            self.journey.blocked.push(name.to_string());

        }

    }

    pub(super) fn check_drain ( &mut self ) -> Flow<()> {

        if Proc::aborted() {

            self.journey.status = Status::Stopped;
            let _ = self.journey.save(&self.cfg.paths.state);

            return Err(Halt::Stopped);

        }

        if self.cfg.paths.drain.exists() {

            self.journey.status = Status::Drained;
            let _ = self.journey.save(&self.cfg.paths.state);

            return Err(Halt::Drained);

        }

        Ok(())

    }

    pub(super) fn save ( &mut self, action: &str ) -> Flow<()> {

        self.journey.last_action = action.to_string();
        self.journey.save(&self.cfg.paths.state)?;

        Ok(())

    }

    pub(super) fn persist_sessions ( &self ) -> AppResult<()> {

        let body = Json::to_string_pretty(&self.sessions)?;
        File::write_atomic(&self.cfg.paths.sessions, &body)

    }

    pub(crate) fn load_sessions ( path: &StdPath ) -> HashMap<String, String> {

        let body = File::read(path);

        if body.trim().is_empty() { return HashMap::new(); }

        let map: HashMap<String, String> = Json::parse(&body).unwrap_or_default();

        map.into_iter().filter(|( _, id )| !id.is_empty()).collect()

    }

    pub(super) fn key ( phase: &str, agent: &str ) -> String {

        format!("{phase}-{agent}")

    }

    pub(super) fn gates ( phase: &str ) -> bool {

        matches!(phase, "tasks" | "tests" | "benches" | "examples" | "fuzzes")

    }

    pub(super) fn active ( &self, phase: &str ) -> bool {

        match phase {
            "requires" | "tasks" => true,
            "audits"   => self.cfg.option.audits,
            "tests"    => self.cfg.option.tests,
            "benches"  => self.cfg.option.benches,
            "examples" => self.cfg.option.examples,
            "fuzzes"   => self.cfg.option.fuzzes,
            _          => false,
        }

    }

    pub(crate) fn verb_of ( phase: &str ) -> &'static str {

        match phase {
            "requires" => "architecting the task plan",
            "tasks"    => "implementing",
            "audits"   => "auditing the system",
            "tests"    => "testing the result",
            "benches"  => "benchmarking the result",
            "examples" => "writing examples",
            "fuzzes"   => "fuzzing the result",
            _          => "working",
        }

    }

}
