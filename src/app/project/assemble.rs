use std::path::{Path as StdPath, PathBuf};

use crate::config::{Config, Paths, Spec};
use crate::config::base::consts::CONFIG_FILE;
use crate::core::error::AppResult;
use crate::core::fs::{Dir, Path};
use crate::app::Project;

impl Project {

    pub(crate) fn assemble ( root: &StdPath ) -> AppResult<Config> {

        let paths = Paths::new(root);
        let document = Spec::document(&paths.config_file)?;
        let spec = document.project;
        let context = Self::discover(&paths, &spec);

        Ok(Config { root: root.to_path_buf(), spec, option: document.option, gate: document.gate, agent: document.agent, paths, context, claude: document.claude, codex: document.codex })

    }

    pub(crate) fn resolve_root ( start: &StdPath ) -> PathBuf {

        let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

        for dir in start.ancestors() {

            if Self::has_config(dir) { return dir.to_path_buf(); }

        }

        start

    }

    fn has_config ( dir: &StdPath ) -> bool {

        Dir::files(dir).iter().any(|path| Path::name_of(path).eq_ignore_ascii_case(CONFIG_FILE))

    }

}
