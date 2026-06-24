use std::collections::HashMap;
use std::fmt;
use std::path::Path as StdPath;
use serde::Deserializer;
use serde::de::{Error, Visitor};

use crate::core::error::AppResult;
use crate::core::support::fs::File;
use crate::core::support::parse::Toml;
use super::arch::{Agent, Document, Gate, Spec, TestsFlag};
use super::consts::{AGENT_TIMEOUT, DEFAULT_MODEL, GATE_TIMEOUT, MANAGER_MODEL, MAX_FIXES, MAX_ROUNDS};

impl Default for Spec {

    fn default () -> Self {

        Self {
            inspire: String::new(),
            tests: true,
            max_rounds: MAX_ROUNDS,
            max_fixes: MAX_FIXES,
            ignore: Vec::new(),
            include: Vec::new(),
        }

    }

}

impl Default for Gate {

    fn default () -> Self {

        Self { timeout: GATE_TIMEOUT, command: String::new() }

    }

}

impl Default for Agent {

    fn default () -> Self {

        Self {
            timeout: AGENT_TIMEOUT,
            manager: MANAGER_MODEL.to_string(),
            architects: vec![DEFAULT_MODEL.to_string()],
            executors: vec![DEFAULT_MODEL.to_string()],
            testers: vec![DEFAULT_MODEL.to_string()],
        }

    }

}

impl Visitor<'_> for TestsFlag {

    type Value = bool;

    fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

        formatter.write_str("a boolean, 0 or 1, or \"true\"/\"false\"")

    }

    fn visit_bool <E> ( self, value: bool ) -> Result<bool, E> where E: Error {

        Ok(value)

    }

    fn visit_i64 <E> ( self, value: i64 ) -> Result<bool, E> where E: Error {

        Ok(value != 0)

    }

    fn visit_u64 <E> ( self, value: u64 ) -> Result<bool, E> where E: Error {

        Ok(value != 0)

    }

    fn visit_str <E> ( self, value: &str ) -> Result<bool, E> where E: Error {

        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" | "" => Ok(false),
            other => Err(E::custom(format!("invalid tests value: {other:?}"))),
        }

    }

}

impl Spec {

    pub fn load ( config_file: &StdPath ) -> AppResult<Self> {

        Ok(Self::document(config_file)?.project)

    }

    pub fn document ( config_file: &StdPath ) -> AppResult<Document> {

        let body = File::read(config_file);

        if body.trim().is_empty() { return Ok(Document::default()); }

        let mut document: Document = Toml::parse(&body)?;
        document.project = document.project.sanitized();
        document.gate = document.gate.sanitized();
        document.agent = document.agent.sanitized();

        Ok(document)

    }

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        let body = File::read(config_file);

        let mut document: Document = if body.trim().is_empty() { Document::default() } else { Toml::parse(&body)? };
        document.project = self.clone();

        File::write_atomic(config_file, &Toml::to_string_pretty(&document)?)

    }

    pub(crate) fn de_tests <'de, D> ( deserializer: D ) -> Result<bool, D::Error> where D: Deserializer<'de> {

        deserializer.deserialize_any(TestsFlag)

    }

    pub(crate) fn parse_tests ( value: &str ) -> Option<bool> {

        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        }

    }

    fn sanitized ( mut self ) -> Self {

        self.max_rounds = self.max_rounds.max(1);

        for paths in [&mut self.ignore, &mut self.include] {

            let mut seen = HashMap::new();
            paths.retain(|path| !path.trim().is_empty() && seen.insert(path.trim().to_string(), ()).is_none());

        }

        self

    }

}

impl Gate {

    fn sanitized ( mut self ) -> Self {

        self.command = self.command.trim().to_string();

        self

    }

}

impl Agent {

    pub fn models ( &self, phase: &str ) -> &[String] {

        match phase {
            "requires" => &self.architects,
            "tasks" => &self.executors,
            "tests" => &self.testers,
            _ => &[],
        }

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        Self::expand_roster(self.models(phase))

    }

    fn sanitized ( mut self ) -> Self {

        self.manager = self.manager.trim().to_string();

        if self.manager.is_empty() { self.manager = MANAGER_MODEL.to_string(); }

        for models in [&mut self.architects, &mut self.executors, &mut self.testers] {

            models.retain(|model| !model.trim().is_empty());

            if models.is_empty() { models.push(DEFAULT_MODEL.to_string()); }

        }

        self

    }

    fn expand_roster ( models: &[String] ) -> Vec<String> {

        let mut roster = Vec::with_capacity(models.len());
        let mut seen: HashMap<&str, u32> = HashMap::new();

        for raw in models {

            let model = raw.trim();

            if model.is_empty() { continue; }

            let count = seen.entry(model).or_insert(0);
            *count += 1;

            roster.push(format!("{model}_{count}"));

        }

        roster

    }

}

impl Document {

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        File::write_atomic(config_file, &Toml::to_string_pretty(self)?)

    }

}
