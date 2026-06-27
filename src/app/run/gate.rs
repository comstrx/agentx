use std::path::Path as StdPath;

use crate::config::base::consts::TOOL;
use crate::core::error::AppError;
use crate::core::fs::{File, Path};
use crate::core::proc::Proc;
use crate::app::{Flow, Gate, Halt, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn gate_failure ( &self, agent: &str, task: Option<&StdPath> ) -> Halt {

        let max = self.cfg.agent.max_fixes;
        let round = self.journey.current_round;
        let log = Path::relative_one(&self.cfg.paths.gate_log, &self.cfg.root);

        let context = match self.journey.current_audit {
            0 => "the tasks phase".to_string(),
            n => format!("the audit phase (audit round {n})"),
        };

        let unit = match task {
            Some(path) => format!("task {}", Path::stem_of(path)),
            None => "the current task".to_string(),
        };

        Halt::Failed(AppError::message(format!(
            "the quality gate is still failing after {max} repair attempts — {context}, manager round {round}, {unit} ({agent}). \
             Fix the gate manually (see {log}), confirm it passes, then run `{TOOL} start` to resume exactly here — nothing is lost."
        )))

    }

    pub(super) fn gate_timeout ( &self, agent: &str, task: Option<&StdPath> ) -> Halt {

        let secs = self.cfg.gate.timeout;
        let unit = match task {
            Some(path) => format!("task {}", Path::stem_of(path)),
            None => "the current task".to_string(),
        };

        Halt::Failed(AppError::message(format!(
            "the quality gate TIMED OUT after {secs}s on {unit} ({agent}) — this is environment slowness, NOT a code defect, \
             so no amount of repair will clear it. Raise [gate].timeout in Agentx.toml (or make the gate command faster), \
             then run `{TOOL} start` to resume exactly here — nothing is lost."
        )))

    }

    pub(super) fn run_gate ( &self ) -> Flow<Gate> {

        let gate = &self.cfg.gate;
        let log = &self.cfg.paths.gate_log;

        if gate.command.is_empty() {

            File::write(log, "no gate command set; gate skipped")?;
            return Ok(Gate::Green);

        }

        let output = Proc::shell_in(&gate.command, &self.cfg.root, gate.timeout)?;
        File::write(log, &format!("{}{}", output.stdout, output.stderr))?;

        if output.timed_out { return Ok(Gate::Timeout); }

        Ok(if output.code == 0 { Gate::Green } else { Gate::Red })

    }

    pub(super) fn gate_step ( &self, depth: usize ) -> Flow<Gate> {

        if self.cfg.gate.command.is_empty() {

            self.run_gate()?;
            Ui::dot(depth, "gate skipped — no gate command set, the work is UNVERIFIED");

            return Ok(Gate::Green);

        }

        Ui::arrow(depth, &format!("running gate · {}", self.cfg.gate.command));

        let result = self.run_gate()?;
        let log = Path::relative_one(&self.cfg.paths.gate_log, &self.cfg.root);

        match result {
            Gate::Green   => Ui::tick(depth, "gate green"),
            Gate::Red     => Ui::cross(depth, &format!("gate red — see {log}")),
            Gate::Timeout => Ui::bang(depth, &format!("gate timed out after {}s — environment slowness, not a code defect", self.cfg.gate.timeout)),
        }

        Ok(result)

    }

}
