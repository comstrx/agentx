use crate::config::{Paths, Spec};
use crate::config::base::consts::PHASES;
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::app::Project;

impl Project {

    pub(crate) fn scaffold ( paths: &Paths ) -> AppResult<()> {

        Self::scaffold_cache(paths)?;

        Dir::ensure(&paths.docs)?;

        if !Path::exists(&paths.config_file) { File::write(&paths.config_file, &Spec::default_toml())?; }

        Ok(())

    }

    fn scaffold_cache ( paths: &Paths ) -> AppResult<()> {

        for phase in PHASES {

            Dir::ensure(&paths.reports_of(phase))?;
            Dir::ensure(&paths.rounds_of(phase))?;

        }

        for dir in [&paths.configs, &paths.manager, &paths.inbox, &paths.tasks, &paths.audit] {

            Dir::ensure(dir)?;

        }

        Ok(())

    }

    pub(crate) fn reset_runtime ( paths: &Paths ) {

        for dir in [&paths.inbox, &paths.tasks, &paths.audit, &paths.reports, &paths.rounds] {

            Dir::clear_files(dir);

        }

        File::remove(&paths.sessions);
        File::remove(&paths.gate_log);

    }

    pub(crate) fn clear ( paths: &Paths ) {

        Dir::remove(&paths.cache);
        let _ = Self::scaffold_cache(paths);

    }

}
