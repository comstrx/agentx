use std::path::{Path as StdPath, PathBuf};
use include_dir::{Dir as Embedded, DirEntry, include_dir};

use crate::core::error::AppResult;
use crate::core::env::Env;
use crate::core::fs::{Dir, File};
use crate::config::base::consts::{BASE_DIR, CACHE_DIR, HISTORY_DIR, PROJECT_DIR};
use super::arch::Train;

pub(super) static INCLUDE: Embedded<'static> = include_dir!("$CARGO_MANIFEST_DIR/seed");

impl Train {

    pub fn init () -> AppResult<()> {

        Self::extract(&INCLUDE, &Self::store())

    }

    pub fn reset () -> AppResult<()> {

        Dir::remove(&Self::store());

        Self::init()

    }

    pub fn sync () -> AppResult<()> {

        Self::resync(&INCLUDE, &Self::store())

    }

    fn extract ( dir: &Embedded, base: &StdPath ) -> AppResult<()> {

        for entry in dir.entries() {

            match entry {
                DirEntry::Dir(sub) => Self::extract(sub, base)?,
                DirEntry::File(file) => {

                    let target = base.join(file.path());

                    if !target.exists() { File::write_bytes(&target, file.contents())?; }

                }
            }

        }

        Ok(())

    }

    fn resync ( dir: &Embedded, base: &StdPath ) -> AppResult<()> {

        for entry in dir.entries() {

            match entry {
                DirEntry::Dir(sub) => Self::resync(sub, base)?,
                DirEntry::File(file) => {

                    if Self::is_history(file.path()) { continue; }

                    File::write_bytes(&base.join(file.path()), file.contents())?;

                }
            }

        }

        Ok(())

    }

    fn is_history ( path: &StdPath ) -> bool {

        path.components().next().is_some_and(|part| part.as_os_str() == HISTORY_DIR)

    }

    fn store () -> PathBuf {

        Env::home().unwrap_or_else(Env::temp_dir).join(CACHE_DIR)

    }

    pub(crate) fn base () -> PathBuf {

        Self::store().join(BASE_DIR)

    }

    pub(crate) fn projects () -> PathBuf {

        Self::store().join(PROJECT_DIR)

    }

    pub(crate) fn histories () -> PathBuf {

        Self::store().join(HISTORY_DIR)

    }

}
