/// Subprocess runner with a wall-clock timeout and process-group isolation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Proc;

/// The outcome of a finished — or timed-out — process.
#[derive(Debug, Clone)]
pub struct Output {

    pub code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}
