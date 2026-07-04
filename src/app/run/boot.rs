use std::collections::{HashMap, HashSet};

use crate::config::Config;
use crate::config::base::consts::{PHASES, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, Path};
use crate::core::proc::Proc;
use crate::app::{Compose, Halt, Journey, Mark, Orchestrator, Status, Ui};

impl Orchestrator {

    pub(crate) fn new ( cfg: Config ) -> Self {

        let journey = Journey::load(&cfg.paths.state);
        let sessions = Self::load_sessions(&cfg.paths.sessions);

        Self { cfg, journey, sessions, live: HashMap::new(), dropped: HashSet::new() }

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

        Ui::blank();
        Ui::title(&format!("{TOOL} · orchestration server"));
        Ui::blank();
        Ui::step("starting up — readying the team and the pipeline");
        Ui::blank();
        Ui::field("project", &Path::display(&self.cfg.root));
        Ui::field("inspire", &kind);

        if !self.cfg.gate.command.is_empty() { Ui::field("gate", &self.cfg.gate.command); }

        Ui::field("team", "");
        Ui::role("manager", self.cfg.manager());

        for phase in PHASES {

            if !self.active(phase) { continue; }

            Ui::role(&format!("{}s", Compose::role_label(phase)), &self.cfg.roster(phase).join(" "));

        }

        Ui::field("engines", "");

        for ( name, model, effort ) in self.engines() {

            Ui::role(&name, &format!("model {model} · effort {effort}"));

        }

        Ui::blank();

    }

    pub(super) fn engines ( &self ) -> Vec<( String, String, String )> {

        let mut out = Vec::new();

        let ( model, effort ) = self.cfg.engine_of_key("manager");
        out.push(( "manager".to_string(), model, effort ));

        for phase in PHASES {

            if !self.active(phase) { continue; }

            for seat in self.cfg.roster(phase) {

                let ( model, effort ) = self.cfg.engine_of_key(&Self::key(phase, &seat));
                out.push(( format!("{phase} {seat}"), model, effort ));

            }

        }

        out

    }

    pub(super) fn report_outcome ( &self ) {

        let shipped = self.journey.task_status.values().filter(|status| status.as_str() == "shipped").count();
        let total = Dir::markdown(&self.cfg.paths.tasks).len();

        let ran: Vec<&str> = PHASES.into_iter().filter(|phase| self.active(phase)).collect();

        Ui::blank();

        if self.journey.blocked.is_empty() {

            Ui::mark(0, Mark::Party, "journey complete — every phase shipped");
            Ui::field("delivered", &format!("{shipped}/{total} task(s)"));
            Ui::field("phases", &ran.join(" → "));

        }
        else {

            Ui::warn(&format!("journey complete with {} open issue(s)", self.journey.blocked.len()));
            Ui::field("delivered", &format!("{shipped}/{total} task(s) shipped"));
            Ui::field("blocked", &self.journey.blocked.join(", "));

        }

        Ui::blank();

    }

}
