use std::path::PathBuf;

pub struct Worker {
    pub(crate) backend:  Backend,
    pub(crate) cwd:      PathBuf,
    pub(crate) timeout:  u64,
    pub(crate) pid_file: Option<PathBuf>,
}

pub enum Backend {
    Claude(Claude),
    Codex(Codex),
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
}

pub struct Codex {
    pub(crate) session: Option<String>,
    pub(crate) model:   String,
    pub(crate) effort:  String,
}
