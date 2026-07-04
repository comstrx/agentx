use std::path::Path as StdPath;

use crate::config::{Paths, Train};
use crate::config::base::consts::{CACHE_DIR, TOOL};
use crate::core::error::AppResult;
use crate::app::{App, Flags, Orchestrator, Project, Ui};

impl App {

    pub fn train ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Self::ensure_idle(&paths)?;

        Self::guard_signals();
        Self::init(&root, flags)?;

        let config = Project::assemble(&root)?;
        Self::ensure_agents(&config)?;

        Self::engage(&paths)?;

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

        if learned > 0 && !yes {

            let options = vec![
                format!("no  — keep the training center as it is (`{TOOL} sync` refreshes nodes and KEEPS history)"),
                format!("yes — wipe everything, including the {learned} learned report(s)"),
            ];

            let picked = Ui::choose(&format!("  reset DELETES the whole training center ~/{CACHE_DIR} — {learned} learned report(s) included, no undo. Proceed?"), &options, 0)?;

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
