use crate::config::base::consts::{DOCS_DIR, REQUIRES};
use crate::core::error::AppError;
use crate::app::{Flow, Journey, Orchestrator, Phase, Project, Status, Ui};

impl Orchestrator {

    pub(super) fn cycle ( &mut self ) -> Flow<()> {

        if self.journey.is_resumable() {

            Ui::blank();
            Ui::step(&format!("resuming journey {} — fresh sessions, re-priming the team, continuing from phase {:?} round {}", self.journey.journey_id, self.journey.phase, self.journey.current_round));

            self.sessions.clear();
            self.live.clear();
            self.persist_sessions()?;

            self.journey.primed = false;
            self.journey.agents_done.clear();
            self.journey.current_agent.clear();
            self.journey.current_round = 0;
            self.journey.status = Status::Running;
            self.save("resume")?;

        }
        else {

            if self.cfg.context.requires.is_empty() {

                return Err(AppError::message(format!("nothing to do — add a requirement (a Requirements.md at the project root, or a file under {DOCS_DIR}/{REQUIRES}/) then run start")).into());

            }

            self.start_fresh()?;

        }

        self.prime()?;

        self.discover()?;

        self.intake()?;

        if self.journey.phase <= Phase::Requires { self.phase_requires()?; }

        if self.journey.phase <= Phase::Tasks { self.phase_tasks()?; }

        self.phase_audit()?;

        self.phase_produce("tests", Phase::Tests, self.cfg.option.tests)?;

        self.phase_produce("benches", Phase::Benches, self.cfg.option.benches)?;

        self.phase_produce("examples", Phase::Examples, self.cfg.option.examples)?;

        self.phase_produce("fuzzes", Phase::Fuzzes, self.cfg.option.fuzzes)?;

        self.journey.phase = Phase::Completed;
        self.journey.status = Status::Completed;
        self.save("completed")?;

        Ok(())

    }

    pub(super) fn start_fresh ( &mut self ) -> Flow<()> {

        self.journey = Journey::fresh();
        self.sessions.clear();

        Project::reset_runtime(&self.cfg.paths);

        self.save("start")?;

        Ok(())

    }

    pub(super) fn enter ( &mut self, phase: Phase ) -> Flow<()> {

        if self.journey.phase != phase {

            self.journey.phase = phase;
            self.journey.current_round = 0;
            self.journey.current_task.clear();
            self.journey.agents_done.clear();

        }

        self.save("phase")?;

        Ok(())

    }

}
