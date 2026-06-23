use std::env;
use clap::Parser;

use crate::core::error::AppResult;
use super::arch::{App, Cli, Command};

impl Cli {

    pub fn run () -> AppResult<()> {

        let cli = Self::parse();

        let dir = match cli.dir {
            Some(path) => path,
            None => env::current_dir()?,
        };

        match cli.command {
            Command::Init    => App::init(&dir, cli.project.as_deref(), cli.gate.as_deref()),
            Command::Start   => App::start(&dir, cli.project.as_deref(), cli.gate.as_deref()),
            Command::Restart => App::restart(&dir, cli.project.as_deref(), cli.gate.as_deref()),
            Command::Stop    => App::stop(&dir),
            Command::Drain   => App::drain(&dir),
            Command::Clean   => App::clean(&dir),
            Command::Info    => App::info(&dir),
            Command::Reset   => App::reset(),
        }

    }

}
