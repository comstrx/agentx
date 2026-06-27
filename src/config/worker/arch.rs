use std::path::{Path as StdPath, PathBuf};

use crate::core::error::AppResult;
use crate::core::proc::Stream;

pub trait Backend {

    fn configure ( &mut self, model: &str, effort: &str );

    fn turn ( &mut self, prompt: &str, cwd: &StdPath, timeout: u64, pid_file: Option<&StdPath> ) -> AppResult<String>;

    fn set_session ( &mut self, id: &str );

    fn clear ( &mut self );

    fn session ( &self ) -> Option<&str>;

}

pub struct Worker {
    pub(crate) backend:  Box<dyn Backend>,
    pub(crate) cwd:      PathBuf,
    pub(crate) timeout:  u64,
    pub(crate) pid_file: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    Transient,
    Session,
    Exhausted,
    Fatal,
}

pub struct Claude {
    pub(crate) session: Option<String>,
    pub(crate) model:   String,
    pub(crate) effort:  String,
    pub(crate) stream:  Option<Stream>,
}

pub struct Codex {
    pub(crate) session: Option<String>,
    pub(crate) model:   String,
    pub(crate) effort:  String,
    pub(crate) stream:  Option<Stream>,
    pub(crate) seq:     i64,
}
