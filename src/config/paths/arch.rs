use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Paths {
    pub root: PathBuf,
    pub docs: PathBuf,
    pub cache: PathBuf,
    pub configs: PathBuf,

    pub config_file: PathBuf,

    pub state: PathBuf,
    pub pid: PathBuf,
    pub active: PathBuf,
    pub sessions: PathBuf,
    pub drain: PathBuf,
    pub gate_log: PathBuf,

    pub inbox: PathBuf,
    pub tasks: PathBuf,
    pub audit: PathBuf,
    pub reports: PathBuf,
    pub manager: PathBuf,
    pub rounds: PathBuf,
    pub tests: PathBuf,
    pub probes: PathBuf,
}
