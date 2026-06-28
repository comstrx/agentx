use crate::config::Train;
use crate::config::base::consts::{REPORTS_DIR, REQUIRES, TASKS_DIR, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Halt, Orchestrator, Ui};

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

        Ui::arrow(0, "the manager is writing a decision report per requirement");

        let prompt = Compose::manager_finalize(&self.cfg);
        self.deliver("manager", &model, "", 0, &prompt)?;

        let kind = self.cfg.spec.inspire.clone();

        if kind.is_empty() {

            Ui::dot(0, "project is unbound — reports written, nothing copied to the training center");

            return Ok(());

        }

        let count = self.archive(&kind);

        Ui::tick(0, &format!("recorded {count} requirement(s) to the training center · {kind}"));

        Ok(())

    }

    fn archive ( &self, kind: &str ) -> usize {

        let mut count = 0;

        for req in Dir::markdown(&self.cfg.paths.inbox) {

            let stem = Self::clean_name(&req);
            let _ = Train::record(kind, REQUIRES, &stem, &File::read(&req));

            let report = self.cfg.paths.manager.join(Path::name_of(&req));

            if report.exists() { let _ = Train::record(kind, REPORTS_DIR, &stem, &File::read(&report)); }

            count += 1;

        }

        for task in Dir::markdown(&self.cfg.paths.tasks) {

            let _ = Train::record(kind, TASKS_DIR, &Self::clean_name(&task), &File::read(&task));

        }

        count

    }

}
