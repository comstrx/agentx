use std::collections::HashMap;
use std::path::Path as StdPath;
use serde::Deserialize;

use crate::core::error::AppResult;
use crate::core::support::fs::File;
use crate::core::support::parse::Toml;
use super::arch::Spec;
use super::defaults::{DEFAULT_MODEL, GATE_TIMEOUT, MANAGER_MODEL, MAX_FIXES, MAX_ROUNDS};
use super::names::STEPS;

impl Default for Spec {

    fn default () -> Self {

        Self {
            max_rounds: MAX_ROUNDS,
            max_fixes: MAX_FIXES,
            gate_cmd: String::new(),
            gate_timeout: GATE_TIMEOUT,
            manager_model: MANAGER_MODEL.to_string(),
            steps: STEPS.iter().map(|step| step.to_string()).collect(),
            arch_models: vec![DEFAULT_MODEL.to_string()],
            work_models: vec![DEFAULT_MODEL.to_string()],
            test_models: vec![DEFAULT_MODEL.to_string()],
        }

    }

}

impl Spec {

    /// The raw model list for a step.
    pub fn models ( &self, step: &str ) -> &[String] {

        match step {
            "arch" => &self.arch_models,
            "work" => &self.work_models,
            "test" => &self.test_models,
            _ => &[],
        }

    }

    /// The expanded roster: `[claude, claude, codex]` -> `[claude_1, claude_2, codex_1]`.
    pub fn roster ( &self, step: &str ) -> Vec<String> {

        expand_roster(self.models(step))

    }

    /// Drop unknown steps, and restore defaults for any list left empty.
    fn sanitized ( mut self ) -> Self {

        self.steps.retain(|step| STEPS.contains(&step.as_str()));

        if self.steps.is_empty() {
            self.steps = STEPS.iter().map(|step| step.to_string()).collect();
        }

        for models in [&mut self.arch_models, &mut self.work_models, &mut self.test_models] {

            models.retain(|model| !model.trim().is_empty());

            if models.is_empty() {
                models.push(DEFAULT_MODEL.to_string());
            }
        }

        self

    }

}

#[derive(Deserialize)]
struct Document {

    #[serde(default)]
    project: Spec,
}

/// Load `[project]` from `Agentx.toml`, falling back to defaults if absent.
pub fn load ( config_file: &StdPath ) -> AppResult<Spec> {

    let body = File::read(config_file);

    if body.trim().is_empty() {
        return Ok(Spec::default());
    }

    let document: Document = Toml::parse(&body)?;

    Ok(document.project.sanitized())

}

/// Expand a model list into uniquely-suffixed agent ids.
pub fn expand_roster ( models: &[String] ) -> Vec<String> {

    let mut roster = Vec::with_capacity(models.len());
    let mut seen: HashMap<&str, u32> = HashMap::new();

    for raw in models {

        let model = raw.trim();

        if model.is_empty() {
            continue;
        }

        let count = seen.entry(model).or_insert(0);
        *count += 1;

        roster.push(format!("{model}_{count}"));
    }

    roster

}
