use std::collections::HashMap;

use crate::config::base::consts::PHASES;
use crate::config::worker::Worker;
use super::arch::Agent;

impl Agent {

    pub fn backends ( &self ) -> Vec<&'static str> {

        let mut names = vec![self.manager.clone()];

        for phase in PHASES {

            names.extend(self.roster(phase));

        }

        let mut out = Vec::new();

        if names.iter().any(|name| Worker::resolve(name) == Some("claude")) { out.push("claude"); }

        if names.iter().any(|name| Worker::resolve(name) == Some("codex")) { out.push("codex"); }

        out

    }

    pub fn models ( &self, phase: &str ) -> &[String] {

        match phase {
            "requires" => &self.requires,
            "tasks" => &self.tasks,
            "audits" => &self.audits,
            "tests" => &self.tests,
            "benches" => &self.benches,
            "examples" => &self.examples,
            "fuzzes" => &self.fuzzes,
            _ => &[],
        }

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        Self::expand_roster(self.models(phase))

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
