use std::process::ExitCode;
use owo_colors::OwoColorize;

use super::arch::AppError;

impl AppError {

    pub fn message ( message: impl Into<String> ) -> Self {

        Self::Message(message.into())

    }

    pub fn parse ( format: impl Into<String>, message: impl Into<String> ) -> Self {

        Self::Parse { format: format.into(), message: message.into() }

    }

    pub fn encode ( format: impl Into<String>, message: impl Into<String> ) -> Self {

        Self::Encode { format: format.into(), message: message.into() }

    }

    pub fn not_found ( what: impl Into<String> ) -> Self {

        Self::NotFound(what.into())

    }

    pub fn invalid ( what: impl Into<String>, message: impl Into<String> ) -> Self {

        Self::Invalid { what: what.into(), message: message.into() }

    }

    pub fn unsupported ( what: impl Into<String> ) -> Self {

        Self::Unsupported(what.into())

    }

    pub fn timeout ( what: impl Into<String>, secs: u64 ) -> Self {

        Self::Timeout { what: what.into(), secs }

    }

    pub fn command ( name: impl Into<String>, code: i32, stderr: impl Into<String> ) -> Self {

        Self::Command { name: name.into(), code, stderr: stderr.into() }

    }

    pub fn network ( url: impl Into<String>, message: impl Into<String> ) -> Self {

        Self::Network { url: url.into(), message: message.into() }

    }

    pub fn exit_code ( &self ) -> ExitCode {

        ExitCode::from(match self {
            Self::Message(_)     => 1,
            Self::Fail { .. }    => 1,
            Self::Io(_)          => 2,
            Self::Parse { .. }   => 3,
            Self::Encode { .. }  => 3,
            Self::NotFound(_)    => 4,
            Self::Invalid { .. } => 5,
            Self::Unsupported(_) => 8,
            Self::Timeout { .. } => 6,
            Self::Command { .. } => 7,
            Self::Network { .. } => 9,
        })

    }

    pub fn print_block ( label: &str, value: &str ) {

        eprintln!("{}", format!("{label}:").bold().bright_black());

        for line in value.lines().filter(|line| !line.trim().is_empty()) {

            eprintln!("  {}", line.bright_red());

        }

    }

    pub fn report ( &self ) -> ExitCode {

        eprintln!("{}: {}", "error".bold().bright_red(), self.to_string().bold());

        if let Self::Command { stderr, .. } = self && !stderr.trim().is_empty() {

            Self::print_block("stderr", stderr);

        }

        let mut source = std::error::Error::source(self);

        while let Some(cause) = source {

            eprintln!("{} {}", "cause:".bold().bright_black(), cause.to_string().bright_black());
            source = cause.source();

        }

        self.exit_code()

    }

}
