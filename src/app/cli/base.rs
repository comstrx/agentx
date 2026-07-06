use std::env;
use std::io;
use std::process::Command as Process;
use clap::builder::StyledStr;
use clap::error::ErrorKind;
use clap::{CommandFactory, FromArgMatches};
use clap_complete::{Shell, generate};
use clap_mangen::Man;

use crate::config::base::consts::{PAD_BOTTOM, PAD_TOP};
use crate::core::error::AppResult;
use crate::core::term::Term;
use crate::core::text::Text;
use crate::app::{App, Ui};
use super::arch::{Cli, Command, Flags, HELP_TEMPLATE};

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

        if self.yes { command.arg("--yes"); }

        if self.force { command.arg("--force"); }

        if self.no_train { command.arg("--no-train"); }

        if self.no_clear { command.arg("--no-clear"); }

    }

}

impl Cli {

    pub fn run () -> AppResult<()> {

        let cli = match Self::try_cli() {
            Ok(cli) => cli,
            Err(error) => return Self::render_clap(error),
        };

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
            yes: cli.yes,
            force: cli.force,
            background: cli.background,
            no_train: cli.no_train,
            no_clear: cli.no_clear,
            ..Flags::default()
        };

        if Term::ansi() { Ui::pad(PAD_TOP); }

        let result = match cli.command {
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
            Command::Inspire { name, show }      => App::inspire(&dir, name.as_deref(), show),
            Command::Gate { command, show }      => App::gate(&dir, command.as_deref(), show),
            Command::Info                        => App::info(&dir),
            Command::Status                      => App::status(&dir),
            Command::Watch                       => App::watch(&dir),
            Command::Doctor                      => App::doctor(&dir),
            Command::Compose { role, phase, out } => App::compose(&dir, role.as_deref(), phase.as_deref(), out.as_deref(), cli.force),
            Command::Sync                        => App::sync(),
            Command::Reset                       => App::reset(cli.yes),
            Command::Completions { shell }       => Self::completions(shell),
            Command::Man                         => Self::man(),
            Command::Help { command }            => Self::help(command.as_deref()),
        };

        if Term::ansi() { Ui::pad(PAD_BOTTOM); }

        result

    }

    fn try_cli () -> Result<Cli, clap::Error> {

        let mut command = Self::tree();
        let matches = command.try_get_matches_from_mut(env::args_os())?;

        Self::from_arg_matches(&matches).map_err(|error| error.format(&mut command))

    }

    fn tree () -> clap::Command {

        Self::command().mut_subcommands(|sub| sub.help_template(HELP_TEMPLATE))

    }

    fn render_clap ( error: clap::Error ) -> AppResult<()> {

        match error.kind() {
            ErrorKind::DisplayHelp => {

                print!("{}", Self::breathe(error.render()));

                Ok(())

            }
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {

                eprint!("{}", Self::breathe(error.render()));

                std::process::exit(2);

            }
            _ => error.exit(),
        }

    }

    fn breathe ( rendered: StyledStr ) -> String {

        let help = if Term::colors() { rendered.ansi().to_string() } else { rendered.to_string() };

        Self::spaced(&help)

    }

    fn spaced ( help: &str ) -> String {

        let lines: Vec<&str> = help.lines().collect();
        let mut out = Vec::with_capacity(lines.len() + 8);

        for ( index, line ) in lines.iter().enumerate() {

            out.push((*line).to_string());

            let plain = Text::plain(line);
            let heading = !plain.starts_with(' ') && !plain.trim().is_empty() && plain.trim_end().ends_with(':');
            let packed = lines.get(index + 1).is_some_and(|next| !next.trim().is_empty());

            if heading && packed { out.push(String::new()); }

        }

        out.join("\n") + "\n"

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

        let mut root = Self::tree();

        if let Some(name) = command && let Some(sub) = root.find_subcommand_mut(name) {

            print!("{}", Self::breathe(sub.render_help()));

            return Ok(());

        }

        print!("{}", Self::breathe(root.render_help()));

        Ok(())

    }

}
