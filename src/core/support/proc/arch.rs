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
