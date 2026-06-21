use std::io::Write;
use std::path::Path as StdPath;

use crate::core::error::AppResult;
use super::arch::File;

impl File {

    /// Read a file to a string, or empty when missing/unreadable.
    pub fn read ( path: &StdPath ) -> String {

        std::fs::read_to_string(path).unwrap_or_default()

    }

    /// Read a file, or `None` when missing/unreadable.
    pub fn read_opt ( path: &StdPath ) -> Option<String> {

        std::fs::read_to_string(path).ok()

    }

    /// Write a file, creating parent directories as needed.
    pub fn write ( path: &StdPath, body: &str ) -> AppResult<()> {

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, body)?;

        Ok(())

    }

    /// Append to a file, creating it (and parents) if absent.
    pub fn append ( path: &StdPath, body: &str ) -> AppResult<()> {

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(body.as_bytes())?;

        Ok(())

    }

    /// Copy a file, creating the destination's parents.
    pub fn copy ( src: &StdPath, dst: &StdPath ) -> AppResult<()> {

        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::copy(src, dst)?;

        Ok(())

    }

    /// Remove a file, ignoring a missing target.
    pub fn remove ( path: &StdPath ) {

        let _ = std::fs::remove_file(path);

    }

}
