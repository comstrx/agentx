use std::path::{Path as StdPath, PathBuf};

use crate::core::error::AppResult;
use crate::core::{date::Date, text::Text};
use super::arch::{Dir, Path};

impl Dir {

    pub fn ensure ( dir: &StdPath ) -> AppResult<()> {

        std::fs::create_dir_all(dir)?;

        Ok(())

    }

    pub fn ensure_parent ( path: &StdPath ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        Ok(())

    }

    pub fn entries ( dir: &StdPath ) -> Vec<PathBuf> {

        let mut out: Vec<PathBuf> = match std::fs::read_dir(dir) {
            Ok(reader) => reader.flatten().map(|entry| entry.path()).collect(),
            Err(_) => Vec::new(),
        };

        out.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        out

    }

    pub fn markdown ( dir: &StdPath ) -> Vec<PathBuf> {

        Self::entries(dir).into_iter().filter(|path| path.is_file() && Path::has_extension(path, "md")).collect()

    }

    pub fn names ( dir: &StdPath ) -> Vec<String> {

        Self::entries(dir).iter().map(|path| Path::name_of(path)).collect()

    }

    pub fn is_empty ( dir: &StdPath ) -> bool {

        match std::fs::read_dir(dir) {
            Ok(mut reader) => reader.next().is_none(),
            Err(_) => true,
        }

    }

    pub fn clear ( dir: &StdPath ) {

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                let path = entry.path();

                if path.is_dir() { let _ = std::fs::remove_dir_all(&path); }
                else { let _ = std::fs::remove_file(&path); }

            }

        }

    }

    pub fn remove ( dir: &StdPath ) {

        let _ = std::fs::remove_dir_all(dir);

    }

    pub fn clear_files ( dir: &StdPath ) {

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                let path = entry.path();

                if path.is_symlink() { let _ = std::fs::remove_file(&path); }
                else if path.is_dir() { Self::clear_files(&path); }
                else { let _ = std::fs::remove_file(&path); }

            }

        }

    }

    pub fn copy_tree ( src: &StdPath, dst: &StdPath ) -> AppResult<()> {

        let entries: Vec<_> = match std::fs::read_dir(src) {
            Ok(reader) => reader.flatten().collect(),
            Err(_) => return Ok(()),
        };

        if entries.is_empty() { return Ok(()); }

        std::fs::create_dir_all(dst)?;

        for entry in entries {

            let from = entry.path();
            let into = dst.join(Path::name_of(&from));

            if from.is_dir() { Self::copy_tree(&from, &into)?; }
            else { std::fs::copy(&from, &into)?; }

        }

        Ok(())

    }

    pub fn leading_number ( name: &str ) -> Option<u32> {

        let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();

        digits.parse().ok()

    }

    pub fn next_sequence ( dir: &StdPath ) -> String {

        let _ = std::fs::create_dir_all(dir);
        let mut highest = 0u32;

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                if let Some(number) = Self::leading_number(&Path::name_of(&entry.path())) {

                    highest = highest.max(number);

                }

            }

        }

        format!("{:03}", highest + 1)

    }

    pub fn next_stamp ( dir: &StdPath ) -> String {

        let _ = std::fs::create_dir_all(dir);

        let day = Date::stamp();
        let prefix = format!("{day}-");

        let mut highest = 0u32;

        if let Ok(reader) = std::fs::read_dir(dir) {

            for entry in reader.flatten() {

                let name = Path::name_of(&entry.path());

                if let Some(rest) = name.strip_prefix(&prefix) && let Some(number) = Self::leading_number(rest) {

                    highest = highest.max(number);

                }

            }

        }

        format!("{day}-{:04}", highest + 1)

    }

    pub fn files ( dir: &StdPath ) -> Vec<PathBuf> {

        Self::entries(dir).into_iter().filter(|path| path.is_file()).collect()

    }

    pub fn subdirs ( dir: &StdPath ) -> Vec<PathBuf> {

        Self::entries(dir).into_iter().filter(|path| path.is_dir()).collect()

    }

    pub fn walk ( dir: &StdPath ) -> Vec<PathBuf> {

        let mut out = Vec::new();

        for path in Self::entries(dir) {

            if path.is_dir() {

                out.push(path.clone());
                out.extend(Self::walk(&path));

            }
            else {

                out.push(path);

            }

        }

        out

    }

}
