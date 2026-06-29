use std::io::Write;
use std::path::Path as StdPath;
use std::time::SystemTime;

use crate::core::error::AppResult;
use super::arch::File;

impl File {

    pub fn read ( path: &StdPath ) -> String {

        std::fs::read_to_string(path).unwrap_or_default()

    }

    pub fn read_opt ( path: &StdPath ) -> Option<String> {

        std::fs::read_to_string(path).ok()

    }

    pub fn read_bytes ( path: &StdPath ) -> Vec<u8> {

        std::fs::read(path).unwrap_or_default()

    }

    pub fn lines ( path: &StdPath ) -> Vec<String> {

        Self::read(path).lines().map(str::to_owned).collect()

    }

    pub fn tail ( path: &StdPath, count: usize ) -> Vec<String> {

        let mut lines = Self::lines(path);

        while lines.last().is_some_and(|line| line.trim().is_empty()) { lines.pop(); }

        let start = lines.len().saturating_sub(count);
        let mut out = lines.split_off(start);

        while out.first().is_some_and(|line| line.trim().is_empty()) { out.remove(0); }

        out

    }

    pub fn write ( path: &StdPath, body: &str ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        std::fs::write(path, body)?;

        Ok(())

    }

    pub fn write_atomic ( path: &StdPath, body: &str ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("tmp");
        let tmp = path.with_file_name(format!(".{name}.{}.tmp", std::process::id()));

        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(body.as_bytes())?;
        file.sync_all()?;
        drop(file);

        std::fs::rename(&tmp, path)?;

        Ok(())

    }

    pub fn write_bytes ( path: &StdPath, body: &[u8] ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        std::fs::write(path, body)?;

        Ok(())

    }

    pub fn append ( path: &StdPath, body: &str ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(body.as_bytes())?;

        Ok(())

    }

    pub fn touch ( path: &StdPath ) -> AppResult<()> {

        if let Some(parent) = path.parent() {

            std::fs::create_dir_all(parent)?;

        }

        std::fs::OpenOptions::new().create(true).append(true).open(path)?;

        Ok(())

    }

    pub fn copy ( src: &StdPath, dst: &StdPath ) -> AppResult<()> {

        if let Some(parent) = dst.parent() {

            std::fs::create_dir_all(parent)?;

        }

        std::fs::copy(src, dst)?;

        Ok(())

    }

    pub fn remove ( path: &StdPath ) {

        let _ = std::fs::remove_file(path);

    }

    pub fn exists ( path: &StdPath ) -> bool {

        path.is_file()

    }

    pub fn size ( path: &StdPath ) -> u64 {

        std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)

    }

    pub fn rename ( src: &StdPath, dst: &StdPath ) -> AppResult<()> {

        if let Some(parent) = dst.parent() {

            std::fs::create_dir_all(parent)?;

        }

        std::fs::rename(src, dst)?;

        Ok(())

    }

    pub fn read_or ( path: &StdPath, default: impl Into<String> ) -> String {

        std::fs::read_to_string(path).unwrap_or_else(|_| default.into())

    }

    pub fn modified ( path: &StdPath ) -> Option<SystemTime> {

        std::fs::metadata(path).and_then(|meta| meta.modified()).ok()

    }

}
