use std::path::PathBuf;
use std::time::Instant;

use crate::config::base::consts::PHASES;
use crate::config::Train;
use crate::core::error::AppError;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Halt, Mark, Orchestrator, Project, Ui};

impl Orchestrator {

    pub(super) fn prime ( &mut self ) -> Flow<()> {

        if self.journey.primed { return Ok(()); }

        let model = self.cfg.manager().to_string();
        let opened = Instant::now();

        Ui::rule("priming · training the team before any work");

        Ui::arrow(0, "lap 1 — teaching the project, the contracts, and each role");
        Ui::blank();

        let fresh = !self.sessions.contains_key("manager");

        if fresh { Ui::working(1, Mark::Study, "training the manager"); }

        let started = Instant::now();
        let brief = Compose::manager_brief(&self.cfg, &self.journey);
        self.prime_turn("manager", &model, &brief)?;

        if fresh { Ui::done(1, Mark::Ok, "manager trained", started); }

        Ui::blank();

        self.discover_and_compose(&model)?;

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);
            let role = Compose::role_label(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if self.dropped.contains(&key) { continue; }

                let fresh = !self.sessions.contains_key(&key);

                if fresh { Ui::working(1, Mark::Study, &format!("training {agent} · {role}")); }

                let started = Instant::now();
                let prompt = Compose::prime(&self.cfg, &self.journey, phase, agent);
                let turn = self.prime_turn(&key, agent, &prompt);

                if self.survive(phase, agent, 1, turn)? && fresh { Ui::done(1, Mark::Ok, &format!("{agent} · {role} trained"), started); }

                Ui::blank();

            }

        }

        Ui::blank();
        Ui::arrow(0, "lap 2 — active-recall confirmation of the invariants");
        Ui::blank();

        Ui::working(1, Mark::Study, "confirming the manager");

        let started = Instant::now();
        let confirm = Compose::reaffirm(&self.cfg, &model);
        self.call("manager", &model, &confirm)?;
        self.check_drain()?;

        Ui::done(1, Mark::Ok, "manager confirmed", started);
        Ui::blank();

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if self.dropped.contains(&key) { continue; }

                let prompt = Compose::reaffirm(&self.cfg, agent);

                Ui::working(1, Mark::Study, &format!("confirming {agent}"));

                let started = Instant::now();
                let turn = self.call(&key, agent, &prompt);

                if self.survive(phase, agent, 1, turn)? { Ui::done(1, Mark::Ok, &format!("{agent} confirmed"), started); }

                Ui::blank();

                self.check_drain()?;

            }

        }

        self.journey.primed = true;
        self.save("primed")?;

        Ui::done(0, Mark::Cool, "team primed — opening the pipeline", opened);

        Ok(())

    }

    pub(super) fn discover_and_compose ( &mut self, model: &str ) -> Flow<()> {

        let before = self.cfg.spec.inspire.trim().to_string();

        self.discover()?;

        let after = self.cfg.spec.inspire.trim().to_string();

        if after == before || after.is_empty() { return Ok(()); }

        self.cfg.context = Project::discover(&self.cfg.paths, &self.cfg.spec);

        if Train::trace(&after).0.is_empty() && Train::history_reports(&after).is_empty() { return Ok(()); }

        Ui::arrow(1, &format!("composed knowledge for {after} — re-briefing the manager"));

        let addendum = Compose::manager_addendum(&self.cfg);
        self.call("manager", model, &addendum)?;

        self.check_drain()

    }

    pub(super) fn prime_turn ( &mut self, key: &str, agent: &str, prompt: &str ) -> Flow<()> {

        if self.sessions.contains_key(key) { return Ok(()); }

        self.call(key, agent, prompt)?;

        self.check_drain()

    }

    pub(super) fn intake ( &mut self ) -> Flow<()> {

        if self.journey.intake_done { return Ok(()); }

        Ui::rule("intake · the manager turns the discovered requirements into an ordered backlog");

        Dir::ensure(&self.cfg.paths.inbox)?;

        Ui::working(0, Mark::Think, "the manager is analysing the discovered requirements");

        let started = Instant::now();
        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_intake(&self.cfg, &self.journey);
        self.deliver("manager", &model, "", 0, &prompt)?;
        self.check_drain()?;

        let authored = Dir::markdown(&self.cfg.paths.inbox);

        if authored.is_empty() {

            let mut copied = 0;

            for source in self.sources() {

                match File::copy(&source, &self.cfg.paths.inbox.join(Path::name_of(&source))) {
                    Ok(_) => copied += 1,
                    Err(error) => Ui::cross(0, &format!("could not copy {} into the backlog — {error}", Path::name_of(&source))),
                }

            }

            if copied == 0 {

                return Err(Halt::Failed(AppError::message("intake produced no backlog — the manager wrote no requirement files and the sources could not be copied; fix the errors above, then run `start` to resume")));

            }

            Ui::bang(0, &format!("manager wrote no files — using {copied} discovered requirement file(s) as-is"));

        }
        else {

            Ui::done(0, Mark::Ok, &format!("{} ordered requirement file(s) ready", authored.len()), started);

        }

        self.journey.intake_done = true;
        self.save("intake")?;

        Ok(())

    }

    pub(super) fn sources ( &self ) -> Vec<PathBuf> {

        self.cfg.context.requires.clone()

    }

}
