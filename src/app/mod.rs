//! agentx itself — the orchestration logic. Builds on `core` + `config`.

mod archive;
mod base;
mod compose;
mod discovery;
mod gate;
mod lifecycle;
mod orchestrator;
mod scaffold;
mod state;
mod workers;

pub mod agentx;

pub use agentx::Agentx;
