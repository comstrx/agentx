use std::path::PathBuf;

use super::arch::Env;

impl Env {

    pub fn get ( key: impl AsRef<str> ) -> Option<String> {

        std::env::var(key.as_ref()).ok()

    }

    pub fn get_or ( key: impl AsRef<str>, default: impl Into<String> ) -> String {

        std::env::var(key.as_ref()).unwrap_or_else(|_| default.into())

    }

    pub fn has ( key: impl AsRef<str> ) -> bool {

        std::env::var_os(key.as_ref()).is_some()

    }

    pub fn vars () -> Vec<( String, String )> {

        std::env::vars().collect()

    }

    pub fn args () -> Vec<String> {

        std::env::args().collect()

    }

    pub fn cwd () -> PathBuf {

        std::env::current_dir().unwrap_or_default()

    }

    pub fn home () -> Option<PathBuf> {

        std::env::var_os("HOME").map(PathBuf::from).filter(|path| !path.as_os_str().is_empty())

    }

    pub fn temp_dir () -> PathBuf {

        std::env::temp_dir()

    }

    pub fn which ( program: impl AsRef<str> ) -> Option<PathBuf> {

        let program = program.as_ref();
        let path = std::env::var_os("PATH")?;

        for dir in std::env::split_paths(&path) {

            let candidate = dir.join(program);

            if candidate.is_file() { return Some(candidate); }

        }

        None

    }

    pub fn os () -> &'static str {

        std::env::consts::OS

    }

    pub fn arch () -> &'static str {

        std::env::consts::ARCH

    }

    pub fn path_dirs () -> Vec<PathBuf> {

        match std::env::var_os("PATH") {
            Some(path) => std::env::split_paths(&path).collect(),
            None => Vec::new(),
        }

    }

    pub fn user () -> Option<String> {

        Self::get("USER").or_else(|| Self::get("USERNAME"))

    }

}
