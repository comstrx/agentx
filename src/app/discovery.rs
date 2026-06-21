use std::path::{Path as StdPath, PathBuf};

use crate::config::Context;
use crate::config::Paths;
use crate::config::names::{CONFIG_FILE, ROOT_FALLBACK_FILES, bucket_of};
use crate::core::support::fs::{Dir, Path};
use crate::core::support::text::Text;

/// Resolve the project root: `Agentx.toml` in `start` -> nearest `.git` upward
/// -> nearest `Agentx.toml` upward -> `start`.
pub fn resolve_root ( start: &StdPath ) -> PathBuf {

    let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

    if has_config(&start) {
        return start;
    }

    for dir in start.ancestors() {

        if dir.join(".git").exists() {
            return dir.to_path_buf();
        }
    }

    for dir in start.ancestors() {

        if has_config(dir) {
            return dir.to_path_buf();
        }
    }

    start

}

/// Bucket the durable docs under `agents/` into the run context.
pub fn discover ( paths: &Paths ) -> Context {

    let mut context = Context::default();

    if paths.docs.is_dir() {

        for entry in Dir::entries(&paths.docs) {

            if entry.is_file() {

                if Path::has_extension(&entry, "md")
                    && let Some(bucket) = bucket_of(&Path::stem_of(&entry).to_ascii_lowercase())
                {
                    context.add(bucket, entry);
                }

            } else if entry.is_dir()
                && let Some(bucket) = bucket_of(&Path::name_of(&entry).to_ascii_lowercase())
            {
                let mut found = Vec::new();
                collect_markdown(&entry, &mut found);

                for md in found {
                    context.add(bucket, md);
                }
            }
        }

    } else {

        for entry in Dir::entries(&paths.root) {

            let name = Path::name_of(&entry).to_ascii_lowercase();

            if entry.is_file() && ROOT_FALLBACK_FILES.contains(&name.as_str()) {
                context.add("overview", entry);
            }
        }
    }

    for bucket in [
        &mut context.overview,
        &mut context.contracts,
        &mut context.history,
        &mut context.tasks,
        &mut context.requires,
    ] {
        bucket.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));
    }

    context

}

fn has_config ( dir: &StdPath ) -> bool {

    Dir::entries(dir).iter().any(|path| path.is_file() && Path::name_of(path).eq_ignore_ascii_case(CONFIG_FILE))

}

fn collect_markdown ( dir: &StdPath, out: &mut Vec<PathBuf> ) {

    for entry in Dir::entries(dir) {

        if entry.is_dir() {
            collect_markdown(&entry, out);
        } else if entry.is_file() && Path::has_extension(&entry, "md") {
            out.push(entry);
        }
    }

}
