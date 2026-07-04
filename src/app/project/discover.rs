use std::path::{Path as StdPath, PathBuf};

use crate::config::{Context, Paths, Spec, Train};
use crate::config::base::consts::{HISTORY, MD_EXT, OVERVIEW};
use crate::core::fs::{Dir, Path};
use crate::core::text::Text;
use crate::app::Project;

impl Project {

    pub(crate) fn discover ( paths: &Paths, spec: &Spec ) -> Context {

        let project = Self::scan(paths, &spec.ignore, &spec.include);
        let kind = spec.inspire.trim();

        if kind.is_empty() { return project; }

        let mut context = Context::default();

        for node in Train::resolve(kind) {

            let mut node_context = Context::default();
            node_context.collect(&node, true);
            node_context.sort();

            context.extend(&node_context);

        }

        for report in Train::history_reports(kind) { context.add(HISTORY, report); }

        context.extend(&project);
        context.requires = project.requires;

        context

    }

    pub(crate) fn unmatched ( docs: &StdPath ) -> Vec<PathBuf> {

        let mut out = Vec::new();

        for entry in Dir::entries(docs) {

            let name = Path::name_of(&entry);

            if name.starts_with('.') { continue; }

            let matched = if entry.is_dir() {

                Context::bucket_of_dir(&name.to_ascii_lowercase()).is_some()

            }
            else if Path::has_extension(&entry, MD_EXT) {

                !Context::buckets_of_stem(&Path::stem_of(&entry).to_ascii_lowercase()).is_empty()

            }
            else {

                true

            };

            if !matched { out.push(entry); }

        }

        out.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        out

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
