use std::collections::{HashMap, HashSet};

use crate::config::{Config, Worker};
use crate::core::error::AppError;
use crate::app::Journey;

pub struct Orchestrator {
    pub cfg: Config,
    pub journey: Journey,
    pub sessions: HashMap<String, String>,
    pub live: HashMap<String, Worker>,
    pub dropped: HashSet<String>,
}

pub enum Halt {
    Drained,
    Stopped,
    Paused,
    Failed(AppError),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Green,
    Red,
    Timeout,
    Broken,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ruling {
    Proceed,
    Fix,
    Stop,
}

pub(crate) struct Menu {
    pub paused: String,
    pub headline: String,
    pub proceed: String,
    pub fix: String,
    pub stop: String,
    pub note: String,
}

pub type Flow<T> = Result<T, Halt>;
