use crate::config::{Paths, Spec};
use crate::config::base::consts::PHASES;
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::app::Project;

impl Project {

    pub(crate) fn scaffold ( paths: &Paths ) -> AppResult<()> {

        for phase in PHASES {

            Dir::ensure(&paths.reports_of(phase))?;
            Dir::ensure(&paths.rounds_of(phase))?;

        }

        for dir in [&paths.docs, &paths.configs, &paths.manager, &paths.inbox, &paths.tasks, &paths.audit, &paths.tests, &paths.probes] {

            Dir::ensure(dir)?;

        }

        if !Path::exists(&paths.config_file) { File::write(&paths.config_file, &Spec::default_toml())?; }

        Ok(())

    }

    pub(crate) fn reset_runtime ( paths: &Paths ) {

        for dir in [&paths.inbox, &paths.tasks, &paths.audit, &paths.reports, &paths.rounds, &paths.probes, &paths.tests] {

            Dir::clear_files(dir);

        }

        File::remove(&paths.sessions);
        File::remove(&paths.gate_log);

    }

    pub(crate) fn clear ( paths: &Paths ) {

        Dir::clear_files(&paths.cache);

    }

}
