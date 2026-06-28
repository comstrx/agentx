use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Gate, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn run_phase ( &mut self, phase: &str, roster: &[String], task: Option<&StdPath> ) -> Flow<bool> {

        let depth = if task.is_some() { 2 } else { 1 };
        let max = self.cfg.agent.max_rounds;
        let mut round = 0;

        self.journey.current_round = 0;

        loop {

            round += 1;
            self.journey.current_round = round;
            self.save("round")?;

            Ui::beat(depth, &format!("round {round}/{max}"));

            let gate_ok = self.roster_pass(phase, roster, task, round > 1)?;
            self.check_drain()?;

            let action = self.manager_review(phase, task, round)?;
            self.check_drain()?;

            if action == "ship" && ( !Self::gates(phase) || gate_ok ) {

                self.archive_final(phase, roster, task)?;

                return Ok(true);

            }

            if round >= max {

                Ui::bang(depth, &format!("reached the round limit ({max}) without convergence"));
                return Ok(false);

            }

        }

    }

    pub(super) fn roster_pass ( &mut self, phase: &str, roster: &[String], task: Option<&StdPath>, has_review: bool ) -> Flow<bool> {

        let mut gate_ok = true;

        let depth = if task.is_some() { 3 } else { 2 };
        let verb = Self::verb_of(phase);
        let max_fixes = self.cfg.agent.max_fixes;

        let pending: Vec<String> = roster.iter().filter(|name| !self.journey.agents_done.iter().any(|done| done == *name)).cloned().collect();
        self.journey.agents_pending = pending;
        self.save("round:agents")?;

        for agent in roster {

            if self.journey.agents_done.iter().any(|done| done == agent) { continue; }

            if self.dropped.contains(&Self::key(phase, agent)) { continue; }

            let activity = match task {
                Some(path) => format!("{agent} · {verb} {}", Path::stem_of(path)),
                None => format!("{agent} · {verb}"),
            };

            Ui::arrow(depth, &activity);

            let prompt = self.build_prompt(phase, agent, task, !gate_ok, has_review);
            let turn = self.worker_turn(phase, agent, task, &prompt);

            if !self.survive(phase, agent, depth, turn)? { continue; }

            Ui::tick(depth, &format!("{agent} wrote {}", Path::relative_one(&self.cfg.paths.report_of(phase, agent), &self.cfg.root)));

            if Self::gates(phase) {

                let mut gate = self.gate_step(depth)?;
                let mut fixes = 0;

                while gate == Gate::Red && fixes < max_fixes {

                    fixes += 1;
                    Ui::bang(depth, &format!("gate red — {agent} repairing (fix {fixes}/{max_fixes})"));

                    let repair = self.build_prompt(phase, agent, task, true, false);

                    self.worker_turn(phase, agent, task, &repair)?;
                    gate = self.gate_step(depth)?;

                }

                match gate {
                    Gate::Green                   => gate_ok = true,
                    Gate::Timeout                 => return Err(self.gate_timeout(agent, task)),
                    Gate::Red if phase == "tasks" => return Err(self.gate_failure(agent, task)),
                    Gate::Red => {

                        Ui::bang(depth, &format!("{agent} left the gate red after {max_fixes} fix(es) — blocking the {phase} phase for the manager"));
                        gate_ok = false;

                    }
                }

            }

            self.journey.agents_done.push(agent.clone());
            self.journey.agents_pending.retain(|pending| pending != agent);
            self.save("agent:done")?;

        }

        self.journey.agents_done.clear();
        self.save("round:done")?;

        Ok(gate_ok)

    }

    pub(super) fn build_prompt ( &self, phase: &str, agent: &str, task: Option<&StdPath>, gate_failed: bool, has_review: bool ) -> String {

        match phase {
            "requires" => Compose::architect(&self.cfg, agent, has_review),
            "tasks" => Compose::executor(&self.cfg, agent, task.unwrap_or_else(|| StdPath::new("")), gate_failed, has_review),
            "audits" => Compose::auditor(&self.cfg, agent, has_review),
            "tests" | "benches" | "examples" | "fuzzes" => Compose::producer(&self.cfg, phase, agent, gate_failed, has_review),
            _ => String::new(),
        }

    }

    pub(super) fn worker_turn ( &mut self, phase: &str, agent: &str, task: Option<&StdPath>, prompt: &str ) -> Flow<()> {

        self.journey.current_agent = agent.to_string();
        self.journey.last_action = format!("{phase}_turn");
        self.save("turn")?;

        self.archive_report(phase, agent, task)?;

        let depth = if task.is_some() { 3 } else { 2 };
        let key = Self::key(phase, agent);
        self.deliver(&key, agent, phase, depth, prompt)?;

        self.journey.last_action = format!("{phase}_report_written");
        self.save("report")?;

        self.check_drain()

    }

    pub(super) fn archive_report ( &self, phase: &str, agent: &str, task: Option<&StdPath> ) -> AppResult<()> {

        let report = self.cfg.paths.report_of(phase, agent);

        if !report.exists() { return Ok(()); }

        let dir = match task {
            Some(path) => self.cfg.paths.task_rounds(&Path::stem_of(path)),
            None => self.cfg.paths.rounds_of(phase),
        };

        Dir::ensure(&dir)?;

        let target = dir.join(format!("{agent}-{}.md", Dir::next_sequence(&dir)));
        File::rename(&report, &target)

    }

    pub(super) fn archive_final ( &self, phase: &str, roster: &[String], task: Option<&StdPath> ) -> AppResult<()> {

        for agent in roster {

            self.archive_report(phase, agent, task)?;

        }

        Ok(())

    }

}
