use std::path::{Path as StdPath, PathBuf};

use crate::config::{Config, Paths, Spec, Train};
use crate::config::base::consts::{CONTEXT_BUCKETS, REQUIRES};
use crate::core::error::AppResult;
use crate::core::env::Env;
use crate::core::fs::Path;
use crate::app::{App, Project, Ui};

impl App {

    pub fn ignore ( dir: &StdPath, paths: &[PathBuf] ) -> AppResult<()> {

        Self::classify_paths(dir, paths, &[])

    }

    pub fn include ( dir: &StdPath, paths: &[PathBuf] ) -> AppResult<()> {

        Self::classify_paths(dir, &[], paths)

    }

    fn classify_paths ( dir: &StdPath, ignore: &[PathBuf], include: &[PathBuf] ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        let mut spec = Spec::load(&paths.config_file)?;
        let mut dirty = Self::merge_into(&mut spec, &root, dir, ignore, false);
        dirty |= Self::merge_into(&mut spec, &root, dir, include, true);

        Ui::blank();

        if !dirty {

            Ui::info("nothing changed — those paths are already classified that way");
            Ui::blank();

            return Ok(());

        }

        spec.save(&paths.config_file)?;

        if !ignore.is_empty() {

            Ui::ok(&format!("ignore — {} path(s) skipped during classification", spec.ignore.len()));

            for entry in &spec.ignore { Ui::item(entry); }

        }

        if !include.is_empty() {

            Ui::ok(&format!("include — {} path(s) forced in (overrides ignore)", spec.include.len()));

            for entry in &spec.include { Ui::item(entry); }

        }

        Ui::blank();

        Ok(())

    }

    pub fn refresh ( dir: &StdPath, ignore: &[PathBuf], include: &[PathBuf] ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);
        Train::init()?;

        let mut spec = Spec::load(&paths.config_file)?;
        spec.ignore.clear();
        spec.include.clear();

        Self::merge_into(&mut spec, &root, dir, ignore, false);
        Self::merge_into(&mut spec, &root, dir, include, true);

        spec.save(&paths.config_file)?;

        let config = Project::assemble(&root)?;

        Ui::blank();
        Ui::ok("classification refreshed — ignore/include lists reset");

        if !config.spec.ignore.is_empty() { Ui::detail("ignore", &format!("{:?}", config.spec.ignore)); }

        if !config.spec.include.is_empty() { Ui::detail("include", &format!("{:?}", config.spec.include)); }

        Ui::blank();
        Ui::head("Classification (briefing files injected per bucket)");

        Self::classification(&config, &root);

        Ui::blank();

        Ok(())

    }

    pub(super) fn merge_into ( spec: &mut Spec, root: &StdPath, base: &StdPath, paths: &[PathBuf], into_include: bool ) -> bool {

        let mut dirty = false;

        for path in paths {

            let entry = Path::relativize(root, base, path);

            if entry.is_empty() { continue; }

            let ( target, other ) = match into_include {
                true => ( &mut spec.include, &mut spec.ignore ),
                false => ( &mut spec.ignore, &mut spec.include ),
            };

            let before = other.len();
            other.retain(|existing| existing != &entry);

            if other.len() != before { dirty = true; }

            if !target.contains(&entry) {

                target.push(entry);
                dirty = true;

            }

        }

        dirty

    }

    pub(super) fn classification ( config: &Config, root: &StdPath ) {

        let home = Env::home().unwrap_or_default();

        for name in CONTEXT_BUCKETS {

            Self::files(name, &Path::shorten_all(config.context.bucket(name), root, &home));

        }

        Self::files(REQUIRES, &Path::shorten_all(&config.context.requires, root, &home));

    }

    fn files ( label: &str, files: &[String] ) {

        Ui::field(label, &format!("{} file(s)", files.len()));

        for file in files {

            Ui::item(file);

        }

    }

}
