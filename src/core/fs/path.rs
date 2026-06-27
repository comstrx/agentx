use std::path::{Path as StdPath, PathBuf};

use super::arch::Path;

impl Path {

    pub fn name_of ( path: &StdPath ) -> String {

        path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string()

    }

    pub fn stem_of ( path: &StdPath ) -> String {

        path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default().to_string()

    }

    pub fn extension ( path: &StdPath ) -> String {

        path.extension().and_then(|ext| ext.to_str()).unwrap_or_default().to_ascii_lowercase()

    }

    pub fn has_extension ( path: &StdPath, ext: &str ) -> bool {

        path.extension().and_then(|value| value.to_str()).is_some_and(|found| found.eq_ignore_ascii_case(ext))

    }

    pub fn parent ( path: &StdPath ) -> Option<PathBuf> {

        path.parent().map(StdPath::to_path_buf)

    }

    pub fn join ( base: &StdPath, child: impl AsRef<StdPath> ) -> PathBuf {

        base.join(child)

    }

    pub fn with_extension ( path: &StdPath, ext: &str ) -> PathBuf {

        path.with_extension(ext)

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

    pub fn relative_one ( path: &StdPath, root: &StdPath ) -> String {

        match path.strip_prefix(root) {
            Ok(rest) => rest.to_string_lossy().into_owned(),
            Err(_) => path.to_string_lossy().into_owned(),
        }

    }

    pub fn relative ( paths: &[PathBuf], root: &StdPath ) -> Vec<String> {

        paths.iter().map(|path| Self::relative_one(path, root)).collect()

    }

    pub fn relativize ( root: &StdPath, base: &StdPath, input: &StdPath ) -> String {

        let abs = if input.is_absolute() { input.to_path_buf() } else { base.join(input) };
        let abs = abs.canonicalize().unwrap_or(abs);

        let rel = abs.strip_prefix(root).unwrap_or(&abs);

        rel.to_string_lossy().replace('\\', "/")

    }

    pub fn shorten ( path: &StdPath, root: &StdPath, home: &StdPath ) -> String {

        if let Ok(rest) = path.strip_prefix(root) { return rest.to_string_lossy().into_owned(); }

        if !home.as_os_str().is_empty() && let Ok(rest) = path.strip_prefix(home) { return format!("~/{}", rest.to_string_lossy()); }

        path.to_string_lossy().into_owned()

    }

    pub fn shorten_all ( paths: &[PathBuf], root: &StdPath, home: &StdPath ) -> Vec<String> {

        paths.iter().map(|path| Self::shorten(path, root, home)).collect()

    }

    pub fn is_hidden ( path: &StdPath ) -> bool {

        Self::name_of(path).starts_with('.')

    }

    pub fn is_symlink ( path: &StdPath ) -> bool {

        path.is_symlink()

    }

    pub fn display ( path: &StdPath ) -> String {

        path.to_string_lossy().into_owned()

    }

}
