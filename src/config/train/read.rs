use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::core::text::Text;
use crate::config::Context;
use crate::config::base::consts::{ABOUT_FILE, CONTRACTS, DESIGNS, HISTORY, MANIFESTS_DIR, MD_EXT, OVERVIEW, REFERENCES, REPORTS_DIR, REQUIRES, SKILLS, TASKS_DIR, TRAIN_DIR};
use super::arch::Train;
use super::seed::INCLUDE;

impl Train {

    pub fn available () -> Vec<String> {

        let mut names: BTreeSet<String> = BTreeSet::new();

        for dir in Dir::subdirs(&Self::trains()) {

            names.insert(Path::name_of(&dir));

        }

        if let Some(train) = INCLUDE.get_dir(TRAIN_DIR) {

            for entry in train.dirs() {

                if let Some(name) = entry.path().file_name().and_then(|value| value.to_str()) {

                    names.insert(name.to_string());

                }

            }

        }

        names.into_iter().collect()

    }

    pub fn context ( name: &str ) -> Context {

        let mut context = Context::default();
        context.collect(&Self::trains().join(name), true);

        context

    }

    pub fn history ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::history_of(name, REPORTS_DIR))

    }

    pub fn past_requires ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::history_of(name, REQUIRES))

    }

    pub fn past_tasks ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::history_of(name, TASKS_DIR))

    }

    pub fn about ( name: &str ) -> String {

        File::read(&Self::trains().join(name).join(ABOUT_FILE))

    }

    pub(crate) fn manifests ( name: &str ) -> PathBuf {

        Self::trains().join(name).join(MANIFESTS_DIR)

    }

    pub fn title ( name: &str ) -> String {

        Text::first_line(&Self::about(name)).trim_start_matches('#').trim().to_string()

    }

    pub fn record ( name: &str, bucket: &str, stem: &str, content: &str ) -> AppResult<()> {

        let dir = Self::history_of(name, bucket);
        Dir::ensure(&dir)?;

        let target = dir.join(format!("{}-{stem}.{MD_EXT}", Dir::next_stamp(&dir)));

        File::write_atomic(&target, content)

    }

    fn history_of ( name: &str, bucket: &str ) -> PathBuf {

        Self::trains().join(name).join(HISTORY).join(bucket)

    }

    pub(crate) fn create ( name: &str ) -> AppResult<()> {

        let dir = Self::trains().join(name);

        for bucket in [OVERVIEW, CONTRACTS, SKILLS, DESIGNS, REFERENCES, MANIFESTS_DIR] {

            Dir::ensure(&dir.join(bucket))?;

        }

        for bucket in [REQUIRES, TASKS_DIR, REPORTS_DIR] {

            Dir::ensure(&Self::history_of(name, bucket))?;

        }

        let about = dir.join(ABOUT_FILE);

        if !about.exists() {

            File::write(&about, &format!("# {name}\n\nStack and description for this archetype. Describe what it is and exactly what kinds of project it fits.\n"))?;

        }

        Ok(())

    }

}
