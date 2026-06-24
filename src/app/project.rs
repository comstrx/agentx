use std::path::{Path as StdPath, PathBuf};

use crate::config::{Config, Context, Paths, Spec, Train};
use crate::config::consts::{CONFIG_FILE, DEFAULT_TOML, PHASES};
use crate::core::error::AppResult;
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::text::Text;
use super::arch::Project;

impl Project {

    pub(crate) fn assemble ( root: &StdPath ) -> AppResult<Config> {

        let paths = Paths::new(root);
        let document = Spec::document(&paths.config_file)?;
        let spec = document.project;
        let context = Self::discover(&paths, &spec);

        Ok(Config { root: root.to_path_buf(), spec, gate: document.gate, agent: document.agent, paths, context, claude: document.claude, codex: document.codex })

    }

    pub(crate) fn resolve_root ( start: &StdPath ) -> PathBuf {

        let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

        for dir in start.ancestors() {

            if Self::has_config(dir) { return dir.to_path_buf(); }

        }

        start

    }

    pub(crate) fn scaffold ( paths: &Paths ) -> AppResult<()> {

        for phase in PHASES {

            Dir::ensure(&paths.reports_of(phase))?;
            Dir::ensure(&paths.rounds_of(phase))?;

        }

        for dir in [&paths.configs, &paths.manager, &paths.inbox, &paths.tasks, &paths.tests, &paths.probes, &paths.prompts] {

            Dir::ensure(dir)?;

        }

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

    pub(crate) fn clear ( paths: &Paths ) {

        Dir::clear_files(&paths.cache);

    }

    fn discover ( paths: &Paths, spec: &Spec ) -> Context {

        let mut context = Self::scan(paths, &spec.ignore, &spec.include);
        let kind = spec.inspire.as_str();

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

    fn scan ( paths: &Paths, ignore: &[String], include: &[String] ) -> Context {

        let mut context = Context::default();
        let root = paths.root.as_path();

        let mut dirs = vec![&paths.root];
        dirs.extend(&paths.docs);

        for dir in dirs {

            for entry in Dir::files(dir) {

                if !Path::has_extension(&entry, "md") { continue; }

                if Self::excluded(&entry, root, ignore, include) { continue; }

                for bucket in Context::buckets_of_stem(&Path::stem_of(&entry).to_ascii_lowercase()) {

                    context.add(bucket, entry.clone());

                }

            }

        }

        for docs in &paths.docs {

            for entry in Dir::subdirs(docs) {

                if let Some(bucket) = Context::bucket_of_dir(&Path::name_of(&entry).to_ascii_lowercase()) {

                    for md in Dir::walk(&entry) {

                        if md.is_file() && Path::has_extension(&md, "md") && !Self::excluded(&md, root, ignore, include) {

                            context.add(bucket, md);

                        }

                    }

                }

            }

        }

        Self::include_extra(&mut context, root, include);

        Self::sort(&mut context);

        context

    }

    fn excluded ( path: &StdPath, root: &StdPath, ignore: &[String], include: &[String] ) -> bool {

        if Self::path_listed(path, root, include) { return false; }

        Self::path_listed(path, root, ignore)

    }

    fn path_listed ( path: &StdPath, root: &StdPath, list: &[String] ) -> bool {

        list.iter().any(|entry| {

            let entry = entry.trim();

            !entry.is_empty() && path.starts_with(root.join(entry))

        })

    }

    fn include_extra ( context: &mut Context, root: &StdPath, include: &[String] ) {

        for entry in include {

            let entry = entry.trim();

            if entry.is_empty() { continue; }

            let target = root.join(entry);

            if target.is_file() {

                if Path::has_extension(&target, "md") { Self::add_include(context, &target); }

            }
            else if target.is_dir() {

                for md in Dir::walk(&target) {

                    if md.is_file() && Path::has_extension(&md, "md") { Self::add_include(context, &md); }

                }

            }

        }

    }

    fn add_include ( context: &mut Context, file: &StdPath ) {

        let buckets = Context::buckets_of_stem(&Path::stem_of(file).to_ascii_lowercase());

        if !buckets.is_empty() {

            for bucket in buckets { context.add(bucket, file.to_path_buf()); }

            return;

        }

        let parent = file.parent().map(|dir| Path::name_of(dir).to_ascii_lowercase()).unwrap_or_default();

        match Context::bucket_of_dir(&parent) {
            Some(bucket) => context.add(bucket, file.to_path_buf()),
            None => context.add("overview", file.to_path_buf()),
        }

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
