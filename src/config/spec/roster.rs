use std::collections::HashMap;

use crate::config::base::consts::PHASES;
use crate::config::worker::Worker;
use super::arch::{Agent, Member, Seats};

impl Member {

    pub fn backend ( name: &str ) -> Member {

        Member { agent: name.trim().to_string(), model: String::new(), effort: String::new() }

    }

    pub fn label ( &self ) -> String {

        let model = self.model.trim();
        let effort = self.effort.trim();

        match model.is_empty() && effort.is_empty() {
            true => self.agent.trim().to_string(),
            false => format!("{} ({}/{})", self.agent.trim(), if model.is_empty() { "·" } else { model }, if effort.is_empty() { "·" } else { effort }),
        }

    }

}

impl Seats {

    pub fn label ( &self ) -> String {

        match self.members.is_empty() {
            true => "(none)".to_string(),
            false => self.members.iter().map(Member::label).collect::<Vec<_>>().join(", "),
        }

    }

}

impl Agent {

    pub fn backends ( &self ) -> Vec<&'static str> {

        let mut agents = vec![self.manager.agent.clone()];

        for phase in PHASES {

            agents.extend(self.members(phase).iter().map(|member| member.agent.clone()));

        }

        let mut out = Vec::new();

        if agents.iter().any(|name| Worker::resolve(name) == Some("claude")) { out.push("claude"); }

        if agents.iter().any(|name| Worker::resolve(name) == Some("codex")) { out.push("codex"); }

        out

    }

    pub fn members ( &self, phase: &str ) -> &[Member] {

        match phase {
            "requires" => &self.requires.members,
            "tasks" => &self.tasks.members,
            "audits" => &self.audits.members,
            "tests" => &self.tests.members,
            "benches" => &self.benches.members,
            "examples" => &self.examples.members,
            "fuzzes" => &self.fuzzes.members,
            _ => &[],
        }

    }

    pub fn seats ( &self, phase: &str ) -> Vec<(String, Member)> {

        let mut out = Vec::new();
        let mut seen: HashMap<&str, u32> = HashMap::new();

        for member in self.members(phase) {

            let agent = member.agent.trim();

            if agent.is_empty() { continue; }

            let count = seen.entry(agent).or_insert(0);
            *count += 1;

            out.push(( format!("{agent}_{count}"), member.clone() ));

        }

        out

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        self.seats(phase).into_iter().map(|( key, _ )| key).collect()

    }

    pub fn member ( &self, phase: &str, seat: &str ) -> Option<Member> {

        self.seats(phase).into_iter().find(|( key, _ )| key == seat).map(|( _, member )| member)

    }

}
