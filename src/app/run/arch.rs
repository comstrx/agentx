use std::collections::HashMap;

use crate::config::{Config, Worker};
use crate::core::error::AppError;
use crate::app::Journey;

pub struct Orchestrator {
    pub cfg: Config,
    pub journey: Journey,
    pub sessions: HashMap<String, String>,
    pub live: HashMap<String, Worker>,
}

pub enum Halt {
    Drained,
    Stopped,
    Failed(AppError),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Green,
    Red,
    Timeout,
}

pub type Flow<T> = Result<T, Halt>;
