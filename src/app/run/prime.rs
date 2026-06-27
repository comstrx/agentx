use std::path::PathBuf;

use crate::config::base::consts::PHASES;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn prime ( &mut self ) -> Flow<()> {

        if self.journey.primed { return Ok(()); }

        let model = self.cfg.manager().to_string();

        Ui::rule("priming · training the team before any work");

        Ui::arrow(0, "lap 1 — teaching the project, the contracts, and each role");

        if !self.sessions.contains_key("manager") { Ui::arrow(1, "training the manager"); }

        let brief = Compose::manager_brief(&self.cfg, &self.journey);
        self.prime_turn("manager", &model, &brief)?;

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);
            let role = Compose::role_label(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if !self.sessions.contains_key(&key) { Ui::arrow(1, &format!("training {agent} · {role}")); }

                let prompt = Compose::prime(&self.cfg, &self.journey, phase, agent);
                self.prime_turn(&key, agent, &prompt)?;

            }

        }

        Ui::arrow(0, "lap 2 — active-recall confirmation of the invariants");

        Ui::arrow(1, "confirming the manager");
        let confirm = Compose::reaffirm(&self.cfg, &model);
        self.call("manager", &model, &confirm)?;
        self.check_drain()?;

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);
                let prompt = Compose::reaffirm(&self.cfg, agent);

                Ui::arrow(1, &format!("confirming {agent}"));
                self.call(&key, agent, &prompt)?;
                self.check_drain()?;

            }

        }

        self.journey.primed = true;
        self.save("primed")?;

        Ui::tick(0, "team primed — opening the pipeline");

        Ok(())

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

        Ui::arrow(0, "the manager is analysing the discovered requirements");

        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_intake(&self.cfg, &self.journey);
        self.deliver("manager", &model, "", 0, &prompt)?;
        self.check_drain()?;

        let authored = Dir::markdown(&self.cfg.paths.inbox);

        if authored.is_empty() {

            for source in self.sources() {

                let _ = File::copy(&source, &self.cfg.paths.inbox.join(Path::name_of(&source)));

            }

            Ui::bang(0, "manager wrote no files — using the discovered requirements as-is");

        }
        else {

            Ui::tick(0, &format!("{} ordered requirement file(s) ready", authored.len()));

        }

        self.journey.intake_done = true;
        self.save("intake")?;

        Ok(())

    }

    pub(super) fn sources ( &self ) -> Vec<PathBuf> {

        self.cfg.context.requires.clone()

    }

}
