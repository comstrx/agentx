use clap::{Parser, Subcommand};

/// Hierarchical multi-agent orchestrator.
#[derive(Parser)]
#[command(name = "agentx", version, about)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {

    /// Scaffold Agentx.toml, agents/, and .agentx/ in the current directory.
    Init,

    /// Resolve the project root and run a full orchestration cycle.
    Start,

    /// Kill the running cycle and its agents immediately.
    Stop,

    /// Stop the running cycle cleanly after the current turn.
    Drain,

    /// Delete the .agentx cache entirely.
    Clean,
}
