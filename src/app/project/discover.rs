use std::path::{Path as StdPath, PathBuf};

use crate::config::{Context, Paths, Spec, Train};
use crate::config::base::consts::{MD_EXT, OVERVIEW};
use crate::core::fs::{Dir, Path};
use crate::core::text::Text;
use crate::app::Project;

impl Project {

    pub(super) fn discover ( paths: &Paths, spec: &Spec ) -> Context {

        let mut context = Self::scan(paths, &spec.ignore, &spec.include);
        let kind = spec.inspire.as_str();

        if !kind.is_empty() {

            let train = Train::context(kind);

            context.overview = Self::merge(train.overview, context.overview);
            context.contracts = Self::merge(train.contracts, context.contracts);
            context.skills = Self::merge(train.skills, context.skills);
            context.designs = Self::merge(train.designs, context.designs);
            context.references = Self::merge(train.references, context.references);
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

        let root = paths.root.as_path();
        let mut context = Context::default();

        context.collect(&paths.root, false);
        context.collect(&paths.docs, true);

        context.retain(|path| !Self::excluded(path, root, ignore, include));

        Self::include_extra(&mut context, root, include);

        context.sort();

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

}
