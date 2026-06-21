use std::path::{Path as StdPath, PathBuf};

use crate::core::error::AppResult;
use crate::core::support::text::Text;
use crate::core::support::time::Time;
use super::arch::{Dir, Path};

impl Dir {

    /// Create a directory and all parents.
    pub fn ensure ( dir: &StdPath ) -> AppResult<()> {

        std::fs::create_dir_all(dir)?;

        Ok(())

    }

    /// Direct children, naturally sorted by name. Empty on any error.
    pub fn entries ( dir: &StdPath ) -> Vec<PathBuf> {

        let mut out: Vec<PathBuf> = match std::fs::read_dir(dir) {
            Ok(reader) => reader.flatten().map(|entry| entry.path()).collect(),
            Err(_) => Vec::new(),
        };

        out.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        out

    }

    /// Markdown files directly under `dir`, naturally sorted.
    pub fn markdown ( dir: &StdPath ) -> Vec<PathBuf> {

        Self::entries(dir).into_iter().filter(|path| path.is_file() && Path::has_extension(path, "md")).collect()

    }

    /// Child names, naturally sorted.
    pub fn names ( dir: &StdPath ) -> Vec<String> {

        Self::entries(dir).iter().map(|path| Path::name_of(path)).collect()

    }

    pub fn is_empty ( dir: &StdPath ) -> bool {

        match std::fs::read_dir(dir) {
            Ok(mut reader) => reader.next().is_none(),
            Err(_) => true,
        }

    }

    /// Delete every child of `dir`, leaving `dir` itself.
    pub fn clear ( dir: &StdPath ) {

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                let path = entry.path();

                if path.is_dir() {
                    let _ = std::fs::remove_dir_all(&path);
                } else {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }

    }

    /// Remove the directory and everything under it, ignoring a missing target.
    pub fn remove ( dir: &StdPath ) {

        let _ = std::fs::remove_dir_all(dir);

    }

    /// Recursively copy `src` into `dst`. No-op when `src` is missing or empty.
    pub fn copy_tree ( src: &StdPath, dst: &StdPath ) -> AppResult<()> {

        let entries: Vec<_> = match std::fs::read_dir(src) {
            Ok(reader) => reader.flatten().collect(),
            Err(_) => return Ok(()),
        };

        if entries.is_empty() {
            return Ok(());
        }

        std::fs::create_dir_all(dst)?;

        for entry in entries {

            let from = entry.path();
            let into = dst.join(Path::name_of(&from));

            if from.is_dir() {
                Self::copy_tree(&from, &into)?;
            } else {
                std::fs::copy(&from, &into)?;
            }
        }

        Ok(())

    }

    /// Next zero-padded counter (`001`, `002`, ...) for files in `dir`.
    pub fn next_sequence ( dir: &StdPath ) -> String {

        let _ = std::fs::create_dir_all(dir);
        let mut highest = 0u32;

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                if let Some(number) = leading_number(&Path::name_of(&entry.path())) {
                    highest = highest.max(number);
                }
            }
        }

        format!("{:03}", highest + 1)

    }

    /// Next dated stamp (`YYYY-MM-DD-0001`) for entries in `dir`.
    pub fn next_stamp ( dir: &StdPath ) -> String {

        let _ = std::fs::create_dir_all(dir);
        let day = Time::stamp();
        let prefix = format!("{day}-");
        let mut highest = 0u32;

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                let name = Path::name_of(&entry.path());

                if let Some(rest) = name.strip_prefix(&prefix)
                    && let Some(number) = leading_number(rest)
                {
                    highest = highest.max(number);
                }
            }
        }

        format!("{day}-{:04}", highest + 1)

    }

}

fn leading_number ( name: &str ) -> Option<u32> {

    let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();

    digits.parse().ok()

}
