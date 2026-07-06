use std::path::Path as StdPath;

use crate::config::{Paths, Train};
use crate::config::base::consts::{CACHE_DIR, TOOL};
use crate::core::error::{AppError, AppResult};
use crate::app::{App, Flags, Orchestrator, Project, Ui};

impl App {

    pub fn train ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Self::ensure_idle(&paths)?;

        Self::guard_signals();
        Self::init_stage(&root, flags)?;

        let config = Project::assemble(&root)?;
        Self::ensure_agents(&config)?;

        Self::claim(&paths)?;

        let mut orchestrator = Orchestrator::new(config);

        Ui::loading("training");

        let result = orchestrator.run_train(false);

        Ui::loaded();

        Self::disengage(&paths);

        result

    }

    pub fn sync () -> AppResult<()> {

        Train::sync()?;

        Ui::blank();
        Ui::ok("training center synced from the binary — learned history kept");
        Ui::blank();

        Ok(())

    }

    pub fn reset ( yes: bool ) -> AppResult<()> {

        let learned = Train::learned();

        let stake = match learned {
            0 => "every hand-added node and bucket".to_string(),
            n => format!("every hand-added node and bucket AND the {n} learned report(s)"),
        };

        if !yes {

            if !Self::interactive() {

                return Err(AppError::message(format!("reset wipes the whole training center ~/{CACHE_DIR} — {stake} — a headless reset needs an explicit `-y/--yes` (or use `{TOOL} sync` to refresh nodes and KEEP everything learned)")));

            }

            let options = vec![
                format!("no  — keep the training center as it is (`{TOOL} sync` refreshes nodes and KEEPS history)"),
                format!("yes — wipe everything: {stake}"),
            ];

            let picked = Ui::choose(&format!("  reset DELETES the whole training center ~/{CACHE_DIR} — {stake}, no undo. Proceed?   ({} · enter = no)", Ui::keys()), &options, 0)?;

            if picked != Some(1) {

                Ui::blank();
                Ui::point(0, "reset cancelled — nothing touched");
                Ui::blank();

                return Ok(());

            }

        }

        Train::reset()?;

        Ui::blank();
        Ui::ok(&format!("training center re-seeded from the binary at ~/{CACHE_DIR}"));
        Ui::blank();

        Ok(())

    }

}
