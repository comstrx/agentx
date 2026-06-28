use crate::config::base::consts::{DOCS_DIR, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::Path;
use crate::core::proc::Proc;
use crate::app::{Compose, Flow, Halt, Journey, Orchestrator, Status, Ui};

impl Orchestrator {

    pub(crate) fn create ( &mut self ) -> AppResult<()> {

        match self.run_create() {
            Ok(()) => {

                self.journey.status = Status::Completed;
                self.journey.note = format!("project created at {}", Path::display(&self.cfg.root));
                let _ = self.journey.save(&self.cfg.paths.state);

                Ui::blank();
                Ui::ok(&format!("project created — add your requirements under {DOCS_DIR}/ (or a Requirements.md at the root), then run `{TOOL} start` to build it"));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Drained) | Err(Halt::Stopped) => {

                self.mark_stopped();

                Ui::blank();
                Ui::warn("interrupted — project creation stopped");
                Ui::blank();

                Ok(())

            }
            Err(Halt::Failed(error)) => {

                if Proc::aborted() {

                    self.mark_stopped();

                    Ui::blank();
                    Ui::warn("interrupted — project creation stopped");
                    Ui::blank();

                    return Ok(());

                }

                self.journey.status = Status::Failed;
                self.journey.note = Self::reason(&error);
                let _ = self.journey.save(&self.cfg.paths.state);

                Err(error)

            }
        }

    }

    fn run_create ( &mut self ) -> Flow<()> {

        self.journey = Journey::create();
        self.save("create")?;

        let model = self.cfg.manager().to_string();

        Ui::blank();
        Ui::title(&format!("{TOOL} · create"));
        Ui::blank();
        Ui::field("project", &Path::display(&self.cfg.root));
        Ui::field("inspire", &self.cfg.spec.inspire);

        Ui::rule("priming · training the manager for this archetype");
        Ui::arrow(0, "training the manager");

        let brief = Compose::manager_brief(&self.cfg, &self.journey);
        self.call("manager", &model, &brief)?;

        let confirm = Compose::reaffirm(&self.cfg, &model);
        self.call("manager", &model, &confirm)?;
        self.check_drain()?;

        Ui::rule("create · the manager scaffolds the project");
        Ui::arrow(0, "manager creating the project skeleton");

        let prompt = Compose::manager_create(&self.cfg);
        self.deliver("manager", &model, "", 0, &prompt)?;

        Ok(())

    }

    fn mark_stopped ( &mut self ) {

        self.journey.status = Status::Stopped;
        let _ = self.journey.save(&self.cfg.paths.state);

    }

}
