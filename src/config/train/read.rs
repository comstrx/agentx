use std::path::PathBuf;

use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::core::text::Text;
use crate::config::base::consts::{MANIFESTS_DIR, MD_EXT, PROJECT_DIR};
use super::arch::Train;
use super::seed::INCLUDE;

impl Train {

    pub fn available () -> Vec<String> {

        let mut out: Vec<String> = Vec::new();

        for dir in Dir::subdirs(&Self::projects()) { Self::enlist(&mut out, &Path::name_of(&dir)); }

        if let Some(projects) = INCLUDE.get_dir(PROJECT_DIR) {

            for entry in projects.dirs() {

                if let Some(name) = entry.path().file_name().and_then(|value| value.to_str()) { Self::enlist(&mut out, name); }

            }

        }

        out

    }

    pub fn history_kinds () -> Vec<String> {

        let mut out: Vec<String> = Vec::new();

        for dir in Dir::subdirs(&Self::histories()) { Self::enlist(&mut out, &Path::name_of(&dir)); }

        out

    }

    fn enlist ( out: &mut Vec<String>, raw: &str ) {

        let clean = Text::unprefix(raw);

        if !clean.is_empty() && !out.iter().any(|existing| existing.eq_ignore_ascii_case(clean)) { out.push(clean.to_string()); }

    }

    pub fn title ( name: &str ) -> String {

        Self::stack(&Self::project(name)).name.trim().to_string()

    }

    pub(crate) fn manifests ( name: &str ) -> PathBuf {

        Self::project(name).join(MANIFESTS_DIR)

    }

    fn history_key ( name: &str ) -> String {

        let bound = Self::stack(&Self::project(name)).history.trim().to_string();

        if bound.is_empty() { name.to_string() } else { bound }

    }

    pub(crate) fn history ( name: &str ) -> AppResult<PathBuf> {

        let key = Self::history_key(name);
        let clean = Text::unprefix(&key);
        let base = Self::histories();

        if let Some(dir) = Dir::subdirs(&base).into_iter().rfind(|entry| Text::unprefix(&Path::name_of(entry)).eq_ignore_ascii_case(clean)) {

            return Ok(dir);

        }

        let mut highest = 0u32;

        for entry in Dir::subdirs(&base) {

            if let Some(number) = Dir::leading_number(&Path::name_of(&entry)) { highest = highest.max(number); }

        }

        let dir = base.join(format!("{:02}_{clean}", highest + 1));
        Dir::ensure(&dir)?;

        Ok(dir)

    }

    pub fn learned () -> usize {

        Dir::walk(&Self::histories())
            .into_iter()
            .filter(|path| path.is_file() && Path::has_extension(path, MD_EXT))
            .count()

    }

    pub(crate) fn history_reports ( name: &str ) -> Vec<PathBuf> {

        let mut reports: Vec<PathBuf> = Dir::walk(&Dir::locate(&Self::histories(), &Self::history_key(name)))
            .into_iter()
            .filter(|path| path.is_file() && Path::has_extension(path, MD_EXT))
            .collect();

        reports.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        reports

    }

    pub fn record ( name: &str, stem: &str, content: &str ) -> AppResult<()> {

        let dir = Self::history(name)?;
        let target = dir.join(format!("{}-{stem}.{MD_EXT}", Dir::next_stamp(&dir)));

        File::write_atomic(&target, content)

    }

    pub(super) fn project ( name: &str ) -> PathBuf {

        Dir::locate(&Self::projects(), name)

    }

}
