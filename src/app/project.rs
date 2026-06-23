use std::path::{Path as StdPath, PathBuf};

use crate::config::{Config, Context, Paths, Spec, Train};
use crate::config::consts::{CACHE_GITIGNORE, CONFIG_FILE, DEFAULT_TOML, PHASES};
use crate::core::error::AppResult;
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::text::Text;
use super::arch::Project;

impl Project {

    pub(crate) fn assemble ( root: &StdPath ) -> AppResult<Config> {

        let paths = Paths::new(root);
        let spec = Spec::load(&paths.config_file)?;
        let context = Self::discover(&paths, &spec.project_type);

        Ok(Config { root: root.to_path_buf(), spec, paths, context })

    }

    pub(crate) fn resolve_root ( start: &StdPath ) -> PathBuf {

        let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

        for dir in start.ancestors() {

            if Self::has_config(dir) { return dir.to_path_buf(); }

        }

        start

    }

    pub(crate) fn scaffold ( paths: &Paths ) -> AppResult<()> {

        for dir in [&paths.contracts, &paths.skills, &paths.requires] {

            Dir::ensure(dir)?;

        }

        if !Path::exists(&paths.overview) { File::write(&paths.overview, "")?; }

        for phase in PHASES {

            Dir::ensure(&paths.reports_of(phase))?;
            Dir::ensure(&paths.rounds_of(phase))?;

        }

        for dir in [&paths.manager, &paths.inbox, &paths.tasks, &paths.tests, &paths.probes, &paths.prompts] {

            Dir::ensure(dir)?;

        }

        if !Path::exists(&paths.gitignore) { File::write(&paths.gitignore, CACHE_GITIGNORE)?; }

        if !Path::exists(&paths.config_file) { File::write(&paths.config_file, DEFAULT_TOML)?; }

        Ok(())

    }

    pub(crate) fn reset_runtime ( paths: &Paths ) {

        for dir in [&paths.inbox, &paths.tasks, &paths.reports, &paths.rounds, &paths.prompts, &paths.probes, &paths.tests] {

            Dir::clear_files(dir);

        }

        File::remove(&paths.sessions);
        File::remove(&paths.gate_log);

    }

    pub(crate) fn clean ( paths: &Paths ) {

        Dir::clear_files(&paths.cache);

    }

    fn discover ( paths: &Paths, kind: &str ) -> Context {

        let mut context = Self::scan(paths);

        if !kind.is_empty() {

            context.overview = Self::merge(Train::overview(kind), context.overview);
            context.contracts = Self::merge(Train::contracts(kind), context.contracts);
            context.skills = Self::merge(Train::skills(kind), context.skills);
            context.history = Self::merge(Train::history(kind), context.history);

        }

        context

    }

    fn merge ( mut train: Vec<PathBuf>, project: Vec<PathBuf> ) -> Vec<PathBuf> {

        train.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));
        train.extend(project);

        train

    }

    fn scan ( paths: &Paths ) -> Context {

        let mut context = Context::default();

        for dir in [&paths.root, &paths.docs] {

            for entry in Dir::files(dir) {

                if !Path::has_extension(&entry, "md") { continue; }

                for bucket in Context::buckets_of_stem(&Path::stem_of(&entry).to_ascii_lowercase()) {

                    context.add(bucket, entry.clone());

                }

            }

        }

        for entry in Dir::subdirs(&paths.docs) {

            if let Some(bucket) = Context::bucket_of_dir(&Path::name_of(&entry).to_ascii_lowercase()) {

                for md in Dir::walk(&entry) {

                    if md.is_file() && Path::has_extension(&md, "md") { context.add(bucket, md); }

                }

            }

        }

        Self::sort(&mut context);

        context

    }

    fn sort ( context: &mut Context ) {

        for bucket in [
            &mut context.overview,
            &mut context.contracts,
            &mut context.skills,
            &mut context.history,
            &mut context.requires,
        ] {

            bucket.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        }

    }

    fn has_config ( dir: &StdPath ) -> bool {

        Dir::files(dir).iter().any(|path| Path::name_of(path).eq_ignore_ascii_case(CONFIG_FILE))

    }

}
