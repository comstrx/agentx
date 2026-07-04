use std::time::Instant;

use crate::config::Train;
use crate::config::base::consts::TOOL;
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Halt, Mark, Orchestrator, Ui};

impl Orchestrator {

    pub(crate) fn run_train ( &mut self, primed: bool ) -> AppResult<()> {

        match self.train(primed) {
            Ok(()) => Ok(()),
            Err(Halt::Drained) | Err(Halt::Stopped) => {

                Ui::blank();
                Ui::warn(&format!("training interrupted — run `{TOOL} train` again to finish, then `{TOOL} clear`"));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Failed(error)) => Err(error),
        }

    }

    fn train ( &mut self, primed: bool ) -> Flow<()> {

        let model = self.cfg.manager().to_string();

        Ui::rule("train · recording the journey into the training center");

        if Dir::markdown(&self.cfg.paths.inbox).is_empty() && Dir::markdown(&self.cfg.paths.tasks).is_empty() {

            Ui::dot(0, "nothing to record — this run has no requirements or tasks yet");

            return Ok(());

        }

        if !primed {

            Ui::arrow(0, "training the manager for the closing report");

            let brief = Compose::manager_brief(&self.cfg, &self.journey);
            self.call("manager", &model, &brief)?;

            let confirm = Compose::reaffirm(&self.cfg, &model);
            self.call("manager", &model, &confirm)?;
            self.check_drain()?;

        }

        Ui::working(0, Mark::Think, "the manager is writing a decision report per requirement");

        let started = Instant::now();
        let prompt = Compose::manager_finalize(&self.cfg);
        self.deliver("manager", &model, "", 0, &prompt)?;

        let kind = self.cfg.spec.inspire.clone();

        if kind.is_empty() {

            Ui::dot(0, "project is unbound — reports written, nothing copied to the training center");

            return Ok(());

        }

        let ( count, failed ) = self.archive(&kind);

        match ( count, failed ) {
            ( 0, 0 ) => Ui::bang(0, "no manager report matched a requirement — nothing recorded to the training center"),
            ( _, 0 ) => Ui::done(0, Mark::Cool, &format!("recorded {count} requirement(s) to the training center · {kind}"), started),
            _        => Ui::bang(0, &format!("recorded {count} requirement(s), {failed} FAILED — see the errors above")),
        }

        Ok(())

    }

    fn archive ( &self, kind: &str ) -> ( usize, usize ) {

        let mut count = 0;
        let mut failed = 0;

        for req in Dir::markdown(&self.cfg.paths.inbox) {

            let report = self.cfg.paths.manager.join(Path::name_of(&req));

            if !report.exists() { continue; }

            match Train::record(kind, &Self::clean_name(&req), &File::read(&report)) {
                Ok(()) => count += 1,
                Err(error) => {

                    failed += 1;
                    Ui::cross(0, &format!("could not record {} — {error}", Path::name_of(&req)));

                }
            }

        }

        ( count, failed )

    }

}
