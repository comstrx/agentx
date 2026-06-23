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

pub struct Claude {
    pub(crate) session: Option<String>,
}

pub struct Codex {
    pub(crate) session: Option<String>,
}
