use std::env;
use std::io;
use std::process::Command as Process;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use clap_mangen::Man;

use crate::core::error::AppResult;
use crate::app::App;
use super::arch::{Cli, Command, Flags};

impl Flags<'_> {

    pub(crate) fn forward ( &self, command: &mut Process ) {

        let options = [
            ( "--description", self.description ),
            ( "--lint", self.lint ),
            ( "--format", self.format ),
            ( "--audits", self.audits ),
            ( "--tests", self.tests ),
            ( "--fuzzes", self.fuzzes ),
            ( "--benches", self.benches ),
            ( "--examples", self.examples ),
            ( "--comments", self.comments ),
            ( "--doc-blocks", self.doc_blocks ),
            ( "--doc-contracts", self.doc_contracts ),
        ];

        for ( flag, value ) in options {

            if let Some(value) = value { command.arg(flag).arg(value); }

        }

        if self.no_train { command.arg("--no-train"); }

        if self.no_clear { command.arg("--no-clear"); }

    }

}

impl Cli {

    pub fn run () -> AppResult<()> {

        let cli = Self::parse();

        let dir = match cli.dir {
            Some(path) => path,
            None => env::current_dir()?,
        };

        let base = Flags {
            inspire: cli.inspire.as_deref(),
            description: cli.description.as_deref(),
            gate: cli.gate.as_deref(),
            lint: cli.lint.as_deref(),
            format: cli.format.as_deref(),
            audits: cli.audits.as_deref(),
            tests: cli.tests.as_deref(),
            fuzzes: cli.fuzzes.as_deref(),
            benches: cli.benches.as_deref(),
            examples: cli.examples.as_deref(),
            comments: cli.comments.as_deref(),
            doc_blocks: cli.doc_blocks.as_deref(),
            doc_contracts: cli.doc_contracts.as_deref(),
            background: cli.background,
            no_train: cli.no_train,
            no_clear: cli.no_clear,
            ..Flags::default()
        };

        match cli.command {
            Command::Init                        => App::init(&dir, &base),
            Command::New { path }                => App::create(&dir, &path, &base),
            Command::Start { ignore, include }   => App::start(&dir, &Flags { ignore: &ignore, include: &include, ..base }),
            Command::Restart { ignore, include } => App::restart(&dir, &Flags { ignore: &ignore, include: &include, ..base }),
            Command::Stop                        => App::stop(&dir),
            Command::Drain                       => App::drain(&dir),
            Command::Train                       => App::train(&dir, &base),
            Command::Clear                       => App::clear(&dir),
            Command::Ignore { paths }            => App::ignore(&dir, &paths),
            Command::Include { paths }           => App::include(&dir, &paths),
            Command::Refresh { ignore, include } => App::refresh(&dir, &ignore, &include),
            Command::Info                        => App::info(&dir),
            Command::Status { tail }             => App::status(&dir, tail),
            Command::Doctor                      => App::doctor(&dir),
            Command::Sync                        => App::sync(),
            Command::Reset                       => App::reset(),
            Command::Completions { shell }       => Self::completions(shell),
            Command::Man                         => Self::man(),
            Command::Help { command }            => Self::help(command.as_deref()),
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
