use std::path::{Path as StdPath, PathBuf};

use super::arch::Path;

impl Path {

    /// File name as a string, or empty.
    pub fn name_of ( path: &StdPath ) -> String {

        path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string()

    }

    /// File stem (name without extension), or empty.
    pub fn stem_of ( path: &StdPath ) -> String {

        path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default().to_string()

    }

    /// Lowercased extension without the dot, or empty.
    pub fn extension ( path: &StdPath ) -> String {

        path.extension().and_then(|ext| ext.to_str()).unwrap_or_default().to_ascii_lowercase()

    }

    pub fn has_extension ( path: &StdPath, ext: &str ) -> bool {

        path.extension().and_then(|value| value.to_str()).is_some_and(|found| found.eq_ignore_ascii_case(ext))

    }

    pub fn exists ( path: &StdPath ) -> bool {

        path.exists()

    }

    pub fn is_file ( path: &StdPath ) -> bool {

        path.is_file()

    }

    pub fn is_dir ( path: &StdPath ) -> bool {

        path.is_dir()

    }

    /// Render `path` relative to `root`, falling back to the full path.
    pub fn relative_one ( path: &StdPath, root: &StdPath ) -> String {

        match path.strip_prefix(root) {
            Ok(rest) => rest.to_string_lossy().into_owned(),
            Err(_) => path.to_string_lossy().into_owned(),
        }

    }

    pub fn relative ( paths: &[PathBuf], root: &StdPath ) -> Vec<String> {

        paths.iter().map(|path| Self::relative_one(path, root)).collect()

    }

}
