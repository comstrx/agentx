use std::collections::HashMap;
use std::path::{Path as StdPath, PathBuf};

use crate::config::Config;
use crate::config::names::CONVERGENCE;
use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::parse::Json;
use crate::core::support::text::Text;
use super::state::State;
use super::{archive, base, compose, gate, workers};

/// Agents run unbounded — a stuck turn is interrupted via `stop`/`drain`.
const WORKER_TIMEOUT: u64 = 0;

/// A control-flow halt: a clean drain, or a real failure to bubble up.
pub enum Halt {

    Drained,
    Failed(AppError),
}

impl From<AppError> for Halt {

    fn from ( error: AppError ) -> Self {

        Self::Failed(error)

    }

}

type Flow<T> = Result<T, Halt>;

/// Drives one full cycle: brief the manager, run each step to convergence under
/// review, then record and archive the run.
pub struct Orchestrator<'a> {

    cfg: &'a Config,
    cwd: PathBuf,
}

impl<'a> Orchestrator<'a> {

    pub fn new ( cfg: &'a Config ) -> Self {

        let body = File::read(&cfg.paths.sessions);

        if !body.trim().is_empty()
            && let Ok(sessions) = Json::parse::<HashMap<String, String>>(&body)
        {
            State::load_sessions(sessions);
        }

        State::set_frozen(task_names(&cfg.paths.tasks));

        Self { cwd: cfg.root.clone(), cfg }

    }

    /// Run the whole cycle, translating control-flow halts into a clean result.
    pub fn run ( &self ) -> AppResult<()> {

        let steps = base::resolve_steps(self.cfg);

        if steps.is_empty() {
            return Err(AppError::message("nothing to do: add a requirement under agents/requires/ or a task under agents/tasks/"));
        }

        match self.cycle(&steps) {

            Ok(()) => {

                let blocked = State::blocked();

                if blocked.is_empty() {
                    println!("[agentx] cycle finished clean");
                } else {
                    println!("[agentx] CYCLE FINISHED WITH OPEN ISSUES in: {} - review the decision record", blocked.join(", "));
                }

                Ok(())

            }

            Err(Halt::Drained) => {
                println!("[agentx] drained - stopped after the current turn, state left intact");
                Ok(())
            }

            Err(Halt::Failed(error)) => Err(error),
        }

    }

    fn cycle ( &self, steps: &[String] ) -> Flow<()> {

        self.brief_manager()?;

        for step in steps {
            println!("[agentx] step: {step}");
            self.run_phase(step)?;
        }

        println!("[agentx] writing decision record");
        self.write_decision()?;
        archive::run(&self.cfg.paths)?;

        Ok(())

    }

    fn brief_manager ( &self ) -> Flow<()> {

        self.call("manager", self.cfg.manager(), &compose::manager_brief(self.cfg))?;

        Ok(())

    }

    fn run_phase ( &self, step: &str ) -> Flow<()> {

        File::remove(&self.cfg.paths.review);

        let mut converged = self.run_step(step, false)?;
        let mut rounds = 0;

        loop {

            self.call("manager", self.cfg.manager(), &compose::manager_review(self.cfg, step, rounds + 1))?;
            self.check_drain()?;

            let ( action, _ ) = Text::parse_control(&File::read(&self.cfg.paths.control));

            if action == "ship" {
                break;
            }

            rounds += 1;

            if rounds >= self.cfg.spec.max_rounds {
                println!("[agentx] {step}: max_rounds reached");
                break;
            }

            converged = self.run_step(step, true)?;
        }

        if !converged {
            State::add_blocked(step);
            println!("[agentx] {step}: NOT converged - open issues recorded");
        }

        File::remove(&self.cfg.paths.review);

        Ok(())

    }

    fn run_step ( &self, step: &str, has_review: bool ) -> Flow<bool> {

        match step {
            "arch" => self.run_arch(has_review),
            "work" => self.run_work(has_review),
            "test" => self.run_test(has_review),
            _ => Ok(true),
        }

    }

    fn run_arch ( &self, has_review: bool ) -> Flow<bool> {

        let frozen = State::frozen();
        let mut rounds = 0;

        while rounds < self.cfg.spec.max_rounds {

            rounds += 1;
            let review = has_review && rounds == 1;

            for agent in self.cfg.spec.roster("arch") {

                let init = State::take_init(&format!("arch-{agent}"));
                let critique = !Dir::markdown(&self.cfg.paths.reports_of("arch")).is_empty();
                let prompt = compose::architect(self.cfg, &agent, init, critique, review, &frozen);

                self.worker_turn("arch", &agent, &prompt)?;
            }

            if self.shipped("arch") {
                return Ok(true);
            }
        }

        Ok(self.shipped("arch"))

    }

    fn run_work ( &self, has_review: bool ) -> Flow<bool> {

        let mut rounds = 0;
        let mut gate_ok = true;

        while rounds < self.cfg.spec.max_rounds {

            rounds += 1;
            let review = has_review && rounds == 1;

            for agent in self.cfg.spec.roster("work") {

                let init = State::take_init(&format!("work-{agent}"));
                let prompt = compose::executor(self.cfg, &agent, init, !gate_ok, review);

                self.worker_turn("work", &agent, &prompt)?;
                gate_ok = self.run_gate()?;

                let mut fixes = 0;

                while !gate_ok && fixes < self.cfg.spec.max_fixes {

                    fixes += 1;
                    let prompt = compose::executor(self.cfg, &agent, false, true, false);

                    self.worker_turn("work", &agent, &prompt)?;
                    gate_ok = self.run_gate()?;
                }
            }

            if gate_ok && self.shipped("work") {
                return Ok(true);
            }
        }

        Ok(gate_ok && self.shipped("work"))

    }

    fn run_test ( &self, has_review: bool ) -> Flow<bool> {

        let mut rounds = 0;

        while rounds < self.cfg.spec.max_rounds {

            rounds += 1;
            let review = has_review && rounds == 1;

            for agent in self.cfg.spec.roster("test") {

                let init = State::take_init(&format!("test-{agent}"));
                let prompt = compose::verifier(self.cfg, &agent, init, review);

                self.worker_turn("test", &agent, &prompt)?;
            }

            if self.shipped("test") {
                return Ok(true);
            }
        }

        Ok(self.shipped("test"))

    }

    fn run_gate ( &self ) -> Flow<bool> {

        let ok = gate::run(&self.cfg.spec.gate_cmd, &self.cwd, self.cfg.spec.gate_timeout, &self.cfg.paths.gate_log)?;

        Ok(ok)

    }

    fn write_decision ( &self ) -> Flow<()> {

        let history = &self.cfg.paths.history;
        Dir::ensure(history)?;

        let decision = history.join(format!("{}.md", Dir::next_stamp(history)));
        self.call("manager", self.cfg.manager(), &compose::manager_decision(self.cfg, &decision))?;

        Ok(())

    }

    fn worker_turn ( &self, step: &str, agent: &str, prompt: &str ) -> Flow<()> {

        let key = format!("{step}-{agent}");
        self.call(&key, agent, prompt)?;

        let report = self.cfg.paths.reports_of(step).join(format!("{agent}.md"));
        snapshot_one(&report, &self.cfg.paths.rounds_of(step))?;

        self.check_drain()

    }

    fn call ( &self, key: &str, agent: &str, prompt: &str ) -> Flow<String> {

        self.dump_prompt(key, prompt)?;

        let session = State::session(key);
        let reply = workers::run(agent, prompt, session.as_deref(), &self.cwd, WORKER_TIMEOUT, &self.cfg.paths.active)?;

        State::set_session(key, &reply.session);
        self.persist_sessions()?;

        Ok(reply.text)

    }

    fn shipped ( &self, step: &str ) -> bool {

        let reports = self.cfg.paths.reports_of(step);

        self.cfg.spec.roster(step).iter().all(|agent| {

            let report = reports.join(format!("{agent}.md"));
            report.exists() && Text::last_line_is(&File::read(&report), CONVERGENCE)

        })

    }

    fn check_drain ( &self ) -> Flow<()> {

        if self.cfg.paths.drain.exists() { Err(Halt::Drained) } else { Ok(()) }

    }

    fn dump_prompt ( &self, label: &str, prompt: &str ) -> AppResult<()> {

        let dir = &self.cfg.paths.prompts;
        Dir::ensure(dir)?;

        let target = dir.join(format!("{}-{label}.md", Dir::next_sequence(dir)));
        File::write(&target, prompt)

    }

    fn persist_sessions ( &self ) -> AppResult<()> {

        let body = Json::to_string_pretty(&State::sessions())?;
        File::write(&self.cfg.paths.sessions, &body)

    }

}

fn task_names ( tasks: &StdPath ) -> Vec<String> {

    Dir::markdown(tasks).iter().map(|path| Path::name_of(path)).collect()

}

fn snapshot_one ( report: &StdPath, rounds: &StdPath ) -> AppResult<()> {

    if !report.exists() {
        return Ok(());
    }

    Dir::ensure(rounds)?;
    let target = rounds.join(format!("{}-{}", Dir::next_sequence(rounds), Path::name_of(report)));
    File::copy(report, &target)

}
