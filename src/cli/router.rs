use std::env;
use clap::Parser;

use crate::app::Agentx;
use crate::core::error::AppResult;
use super::arch::{Cli, Command};

/// Parse the command line and dispatch to the matching command.
pub fn run () -> AppResult<()> {

    let cli = Cli::parse();
    let cwd = env::current_dir()?;

    match cli.command {
        Command::Init => Agentx::init(&cwd),
        Command::Start => Agentx::start(&cwd),
        Command::Stop => Agentx::stop(&cwd),
        Command::Drain => Agentx::drain(&cwd),
        Command::Clean => Agentx::clean(&cwd),
    }

}
