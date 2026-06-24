use std::env;
use std::io;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use clap_mangen::Man;

use crate::core::error::AppResult;
use super::arch::{App, Cli, Command, Flags};

impl Cli {

    pub fn run () -> AppResult<()> {

        let cli = Self::parse();

        let dir = match cli.dir {
            Some(path) => path,
            None => env::current_dir()?,
        };

        let base = Flags {
            inspire: cli.inspire.as_deref(),
            gate: cli.gate.as_deref(),
            tests: cli.tests.as_deref(),
            background: cli.background,
            ..Flags::default()
        };

        match cli.command {
            Command::Init                       => App::init(&dir, &base),
            Command::Start { ignore, include }  => App::start(&dir, &Flags { ignore: &ignore, include: &include, ..base }),
            Command::Restart { ignore, include }=> App::restart(&dir, &Flags { ignore: &ignore, include: &include, ..base }),
            Command::Stop                       => App::stop(&dir),
            Command::Drain                      => App::drain(&dir),
            Command::Clear                      => App::clear(&dir),
            Command::Ignore { paths }           => App::ignore(&dir, &paths),
            Command::Include { paths }          => App::include(&dir, &paths),
            Command::Refresh { ignore, include }=> App::refresh(&dir, &ignore, &include),
            Command::Info                       => App::info(&dir),
            Command::Status { tail }            => App::status(&dir, tail),
            Command::Doctor                     => App::doctor(&dir),
            Command::Sync                       => App::sync(),
            Command::Reset                      => App::reset(),
            Command::Completions { shell }      => Self::completions(shell),
            Command::Man                        => Self::man(),
            Command::Help { command }           => Self::help(command.as_deref()),
        }

    }

    fn completions ( shell: Shell ) -> AppResult<()> {

        let mut command = Self::command();
        let name = command.get_name().to_string();

        generate(shell, &mut command, name, &mut io::stdout());

        Ok(())

    }

    fn man () -> AppResult<()> {

        Man::new(Self::command()).render(&mut io::stdout())?;

        Ok(())

    }

    fn help ( command: Option<&str> ) -> AppResult<()> {

        let mut root = Self::command();

        if let Some(name) = command && let Some(sub) = root.find_subcommand_mut(name) {

            sub.print_help()?;

            return Ok(());

        }

        root.print_help()?;

        Ok(())

    }

}
