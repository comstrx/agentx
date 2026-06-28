use std::path::PathBuf;

use crate::config::{Agent, Engine, Gate, Options, Paths, Spec};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub overview: Vec<PathBuf>,
    pub contracts: Vec<PathBuf>,
    pub skills: Vec<PathBuf>,
    pub designs: Vec<PathBuf>,
    pub references: Vec<PathBuf>,
    pub history: Vec<PathBuf>,
    pub requires: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub root: PathBuf,
    pub spec: Spec,
    pub option: Options,
    pub gate: Gate,
    pub agent: Agent,
    pub paths: Paths,
    pub context: Context,
    pub claude: Engine,
    pub codex: Engine,
}
