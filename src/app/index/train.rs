use std::path::Path as StdPath;

use crate::config::{Paths, Train};
use crate::config::base::consts::CACHE_DIR;
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

    pub fn reset () -> AppResult<()> {

        Train::reset()?;

        Ui::blank();
        Ui::ok(&format!("training center re-seeded from the binary at ~/{CACHE_DIR}"));
        Ui::blank();

        Ok(())

    }

}
