use std::time::Duration;

use crate::config::base::consts::AGENT_RETRIES;
use crate::core::error::{AppError, AppResult};
use crate::core::proc::Proc;
use crate::core::text::Text;
use crate::config::worker::{Fault, Worker};
use crate::app::{Compose, Flow, Halt, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn dispatch ( &mut self, key: &str, agent: &str, prompt: &str ) -> AppResult<()> {

        if !self.live.contains_key(key) {

            let ( model, effort ) = self.cfg.engine(agent);

            let mut runner = Worker::new(agent);
            runner.cwd(&self.cfg.root).timeout(self.cfg.agent.timeout).pid_file(&self.cfg.paths.active);
            runner.engine(&model, &effort);

            if let Some(session) = self.sessions.get(key) && !session.is_empty() {

                runner.set_session(session);

            }

            self.live.insert(key.to_string(), runner);

        }

        let Some(runner) = self.live.get_mut(key) else {

            return Err(AppError::message("worker registry out of sync"));

        };

        let session = runner.turn(prompt)?;

        if !session.is_empty() {

            self.sessions.insert(key.to_string(), session);
            self.persist_sessions()?;

        }

        Ok(())

    }

    pub(super) fn call ( &mut self, key: &str, agent: &str, prompt: &str ) -> Flow<()> {

        let mut tries = 0;

        loop {

            self.check_drain()?;

            let error = match self.dispatch(key, agent, prompt) {
                Ok(()) => return Ok(()),
                Err(error) => error,
            };

            self.check_drain()?;

            tries += 1;

            if Worker::fault(&error) == Fault::Transient && tries <= AGENT_RETRIES {

                Ui::bang(1, &format!("{agent} — hiccup ({}); retrying {tries}/{AGENT_RETRIES}", Self::reason(&error)));
                self.backoff(tries)?;

                continue;

            }

            Ui::cross(1, &format!("{agent} failed to initialise — {}; stopping", Self::reason(&error)));

            return Err(Halt::Failed(error));

        }

    }

    pub(super) fn deliver ( &mut self, key: &str, agent: &str, phase: &str, depth: usize, prompt: &str ) -> Flow<()> {

        let mut tries = 0;
        let mut reprimed = false;

        loop {

            self.check_drain()?;

            let error = match self.dispatch(key, agent, prompt) {
                Ok(()) => return Ok(()),
                Err(error) => error,
            };

            self.check_drain()?;

            match Worker::fault(&error) {
                Fault::Exhausted => {

                    Ui::cross(depth, &format!("{agent} — provider usage/quota exhausted; stopping, `start` resumes once it resets"));

                    return Err(Halt::Failed(error));

                }
                Fault::Fatal => {

                    Ui::cross(depth, &format!("{agent} — unrecoverable: {}; stopping", Self::reason(&error)));

                    return Err(Halt::Failed(error));

                }
                Fault::Transient => {

                    tries += 1;

                    if tries <= AGENT_RETRIES {

                        Ui::bang(depth, &format!("{agent} — hiccup ({}); retrying {tries}/{AGENT_RETRIES}", Self::reason(&error)));
                        self.backoff(tries)?;

                        continue;

                    }

                }
                Fault::Session => {}
            }

            if reprimed {

                Ui::cross(depth, &format!("{agent} — did not recover after re-priming; stopping ({})", Self::reason(&error)));

                return Err(Halt::Failed(error));

            }

            Ui::bang(depth, &format!("{agent} — recovering on a fresh session: re-train, confirm, then resume"));

            self.reprime(key, agent, phase)?;

            reprimed = true;
            tries = 0;

        }

    }

    pub(super) fn reprime ( &mut self, key: &str, agent: &str, phase: &str ) -> Flow<()> {

        self.sessions.remove(key);
        self.live.remove(key);
        self.persist_sessions()?;

        let brief = if key == "manager" {

            Compose::manager_brief(&self.cfg, &self.journey)

        }
        else {

            Compose::prime(&self.cfg, &self.journey, phase, agent)

        };

        self.call(key, agent, &brief)?;

        let confirm = Compose::reaffirm(&self.cfg, agent);

        self.call(key, agent, &confirm)

    }

    pub(super) fn backoff ( &self, attempt: u32 ) -> Flow<()> {

        let seconds = ( 1u64 << attempt.min(4) ).min(15);

        for _ in 0..seconds {

            if Proc::aborted() { return Err(Halt::Stopped); }

            std::thread::sleep(Duration::from_secs(1));

        }

        Ok(())

    }

    pub(super) fn reason ( error: &AppError ) -> String {

        let raw = match error {
            AppError::Timeout { secs, .. } => return format!("timed out after {secs}s"),
            AppError::Command { stderr, .. } if !stderr.trim().is_empty() => Text::first_line(stderr).to_string(),
            other => other.to_string(),
        };

        raw.chars().take(90).collect()

    }

}
