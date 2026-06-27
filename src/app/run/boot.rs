use std::collections::HashMap;

use crate::config::Config;
use crate::config::base::consts::{PHASES, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, Path};
use crate::core::proc::Proc;
use crate::config::worker::Worker;
use crate::app::{Halt, Journey, Orchestrator, Status, Ui};

impl Orchestrator {

    pub(crate) fn new ( cfg: Config ) -> Self {

        let journey = Journey::load(&cfg.paths.state);
        let sessions = Self::load_sessions(&cfg.paths.sessions);

        Self { cfg, journey, sessions, live: HashMap::new() }

    }

    pub(crate) fn run ( &mut self ) -> AppResult<()> {

        self.boot();

        match self.cycle() {
            Ok(()) => {

                self.report_outcome();

                Ok(())

            }
            Err(Halt::Drained) => {

                Ui::blank();
                Ui::ok(&format!("drained at phase {:?}, round {} — state saved; `start` resumes", self.journey.phase, self.journey.current_round));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Stopped) => {

                Ui::blank();
                Ui::warn(&format!("interrupted — stopped at phase {:?}, round {}; state saved, `start` resumes", self.journey.phase, self.journey.current_round));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Failed(error)) => {

                if Proc::aborted() {

                    if !self.journey.journey_id.is_empty() {

                        self.journey.status = Status::Stopped;
                        let _ = self.journey.save(&self.cfg.paths.state);

                    }

                    Ui::blank();
                    Ui::warn(&format!("interrupted — stopped at phase {:?}, round {}; state saved, `start` resumes", self.journey.phase, self.journey.current_round));
                    Ui::blank();

                    return Ok(());

                }

                if !self.journey.journey_id.is_empty() {

                    self.journey.status = Status::Failed;
                    let _ = self.journey.save(&self.cfg.paths.state);

                }

                Err(error)

            }
        }

    }

    pub(super) fn boot ( &self ) {

        let kind = if self.cfg.spec.inspire.is_empty() { "(unbound)".to_string() } else { self.cfg.spec.inspire.clone() };
        let gate = if self.cfg.gate.command.is_empty() { "(none — gate skipped)".to_string() } else { self.cfg.gate.command.clone() };

        Ui::blank();
        Ui::title(&format!("{TOOL} · orchestration server"));
        Ui::blank();
        Ui::step("starting up — readying the team and the pipeline");
        Ui::field("project", &Path::display(&self.cfg.root));
        Ui::field("type", &kind);
        Ui::field("gate", &gate);
        Ui::field("team", "");
        Ui::role("manager", self.cfg.manager());
        Ui::role("architects", &self.cfg.roster("requires").join(" "));
        Ui::role("executors", &self.cfg.roster("tasks").join(" "));

        for phase in ["audits", "tests", "benches", "examples", "fuzzes"] {

            if self.active(phase) { Ui::role(phase, &self.cfg.roster(phase).join(" ")); }

        }

        Ui::field("engines", "");

        for ( name, model, effort ) in self.engines() {

            Ui::role(name, &format!("model {model} · effort {effort}"));

        }

        if self.cfg.gate.command.trim().is_empty() {

            Ui::blank();
            Ui::warn("no quality gate is set — the team's code will NOT be gate-verified this run; set [gate].command or pass --gate");

        }

        Ui::blank();

    }

    pub(super) fn engines ( &self ) -> Vec<( &'static str, String, String )> {

        let claude = self.cfg.engine("claude");
        let codex = self.cfg.engine("codex");

        let mut team: Vec<String> = vec![self.cfg.manager().to_string()];

        for phase in PHASES {

            team.extend(self.cfg.roster(phase));

        }

        let mut out = Vec::new();

        if team.iter().any(|model| Worker::resolve(model) == Some("claude")) { out.push(("claude", Self::dash(&claude.0), Self::dash(&claude.1))); }

        if team.iter().any(|model| Worker::resolve(model) == Some("codex")) { out.push(("codex", Self::dash(&codex.0), Self::dash(&codex.1))); }

        out

    }

    pub(super) fn dash ( value: &str ) -> String {

        if value.trim().is_empty() { "default".to_string() } else { value.trim().to_string() }

    }

    pub(super) fn report_outcome ( &self ) {

        let shipped = self.journey.task_status.values().filter(|status| status.as_str() == "shipped").count();
        let total = Dir::markdown(&self.cfg.paths.tasks).len();

        let ran: Vec<&str> = ["requires", "tasks", "audits", "tests", "benches", "examples", "fuzzes"]
            .into_iter()
            .filter(|phase| matches!(*phase, "requires" | "tasks") || self.active(phase))
            .collect();

        Ui::blank();

        if self.journey.blocked.is_empty() {

            Ui::ok("journey complete — every phase shipped");
            Ui::field("delivered", &format!("{shipped}/{total} task(s)"));
            Ui::field("phases", &ran.join(" → "));
            Ui::field("next", "recording the run into the training center, then clearing the runtime");

        }
        else {

            Ui::warn(&format!("journey complete with {} open issue(s)", self.journey.blocked.len()));
            Ui::field("delivered", &format!("{shipped}/{total} task(s) shipped"));
            Ui::field("blocked", &self.journey.blocked.join(", "));
            Ui::field("next", &format!("runtime kept for inspection — fix it, then `{TOOL} start` to resume (or `{TOOL} train` + `{TOOL} clear`)"));

        }

        Ui::blank();

    }

}
