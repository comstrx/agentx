use std::process::{Child, ChildStdin};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub const POLL: Duration = Duration::from_millis(50);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Proc;

#[derive(Debug, Clone)]
pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

pub struct Stream {
    pub(crate) child:  Child,
    pub(crate) stdin:  ChildStdin,
    pub(crate) lines:  Receiver<String>,
    pub(crate) errors: Arc<Mutex<String>>,
    pub(crate) pid:    i32,
}

pub enum Recv {
    Line(String),
    Idle,
    Closed,
}
