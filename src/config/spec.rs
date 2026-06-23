use std::collections::HashMap;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::support::fs::File;
use crate::core::support::parse::Toml;
use super::arch::{Document, Spec};
use super::consts::{DEFAULT_MODEL, GATE_TIMEOUT, MANAGER_MODEL, MAX_FIXES, MAX_ROUNDS};

impl Default for Spec {

    fn default () -> Self {

        Self {
            project_type: String::new(),
            max_rounds: MAX_ROUNDS,
            max_fixes: MAX_FIXES,
            gate_cmd: String::new(),
            gate_timeout: GATE_TIMEOUT,
            manager_model: MANAGER_MODEL.to_string(),
            architect_models: vec![DEFAULT_MODEL.to_string()],
            executor_models: vec![DEFAULT_MODEL.to_string()],
            tester_models: vec![DEFAULT_MODEL.to_string()],
        }

    }

}

impl Spec {

    pub fn load ( config_file: &StdPath ) -> AppResult<Self> {

        let body = File::read(config_file);

        if body.trim().is_empty() { return Ok(Self::default()); }

        let document: Document = Toml::parse(&body)?;

        Ok(document.project.sanitized())

    }

    pub fn save ( &self, config_file: &StdPath ) -> AppResult<()> {

        let document = Document { project: self.clone() };

        File::write_atomic(config_file, &Toml::to_string_pretty(&document)?)

    }

    pub fn models ( &self, phase: &str ) -> &[String] {

        match phase {
            "requires" => &self.architect_models,
            "tasks" => &self.executor_models,
            "tests" => &self.tester_models,
            _ => &[],
        }

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        Self::expand_roster(self.models(phase))

    }

    fn sanitized ( mut self ) -> Self {

        self.max_rounds = self.max_rounds.max(1);

        self.manager_model = self.manager_model.trim().to_string();

        if self.manager_model.is_empty() { self.manager_model = MANAGER_MODEL.to_string(); }

        for models in [&mut self.architect_models, &mut self.executor_models, &mut self.tester_models] {

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
