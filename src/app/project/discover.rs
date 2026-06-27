use std::path::{Path as StdPath, PathBuf};

use crate::config::{Context, Paths, Spec, Train};
use crate::config::base::consts::{DESIGNS, MD_EXT, OVERVIEW};
use crate::core::fs::{Dir, Path};
use crate::core::text::Text;
use crate::app::Project;

impl Project {

    pub(super) fn discover ( paths: &Paths, spec: &Spec ) -> Context {

        let mut context = Self::scan(paths, &spec.ignore, &spec.include);
        let kind = spec.inspire.as_str();

        if !kind.is_empty() {

            context.overview = Self::merge(Train::overview(kind), context.overview);
            context.contracts = Self::merge(Train::contracts(kind), context.contracts);
            context.skills = Self::merge(Train::skills(kind), context.skills);
            context.designs = Self::merge(Train::designs(kind), context.designs);
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

        for dir in [&paths.root, &paths.docs] {

            for entry in Dir::files(dir) {

                if !Path::has_extension(&entry, MD_EXT) { continue; }

                if Self::excluded(&entry, root, ignore, include) { continue; }

                for bucket in Context::buckets_of_stem(&Path::stem_of(&entry).to_ascii_lowercase()) {

                    context.add(bucket, entry.clone());

                }

            }

        }

        for entry in Dir::subdirs(&paths.docs) {

            if let Some(bucket) = Context::bucket_of_dir(&Path::name_of(&entry).to_ascii_lowercase()) {

                let any = bucket == DESIGNS;

                for md in Dir::walk(&entry) {

                    if md.is_file() && ( any || Path::has_extension(&md, MD_EXT) ) && !Self::excluded(&md, root, ignore, include) {

                        context.add(bucket, md);

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

                if Path::has_extension(&target, MD_EXT) { Self::add_include(context, &target); }

            }
            else if target.is_dir() {

                for md in Dir::walk(&target) {

                    if md.is_file() && Path::has_extension(&md, MD_EXT) { Self::add_include(context, &md); }

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
            None => context.add(OVERVIEW, file.to_path_buf()),
        }

    }

    fn sort ( context: &mut Context ) {

        for bucket in [
            &mut context.overview,
            &mut context.contracts,
            &mut context.skills,
            &mut context.designs,
            &mut context.history,
            &mut context.requires,
        ] {

            bucket.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        }

    }

}
