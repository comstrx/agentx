use std::io::IsTerminal;
use std::process::ExitCode;
use std::sync::OnceLock;
use owo_colors::{OwoColorize, Style};

use super::arch::AppError;

static TINT: OnceLock<bool> = OnceLock::new();

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

    pub fn detail ( &self ) -> String {

        let raw = match self {
            Self::Command { stderr, .. } if !stderr.trim().is_empty() => stderr.lines().find(|line| !line.trim().is_empty()).unwrap_or_default().to_string(),
            other => other.to_string(),
        };

        for key in ["\"detail\":\"", "\"message\":\"", "\"error\":\""] {

            if let Some(at) = raw.find(key) {

                let rest = &raw[at + key.len()..];
                let end = rest.find('"').unwrap_or(rest.len());

                return rest[..end].trim().to_string();

            }

        }

        raw.trim().to_string()

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

        eprintln!("{}", Self::dye(&format!("{label}:"), Style::new().bright_blue().bold()));

        for line in value.lines().filter(|line| !line.trim().is_empty()) {

            eprintln!("  {}", Self::dye(line, Style::new().bright_red()));

        }

    }

    pub fn report ( &self ) -> ExitCode {

        eprintln!("{}: {}", Self::dye("error", Style::new().bright_red().bold()), Self::dye(&self.to_string(), Style::new().bold()));

        if let Self::Command { stderr, .. } = self && !stderr.trim().is_empty() {

            Self::print_block("stderr", stderr);

        }

        let mut source = std::error::Error::source(self);

        while let Some(cause) = source {

            eprintln!("{} {}", Self::dye("cause:", Style::new().bright_blue().bold()), Self::dye(&cause.to_string(), Style::new().bright_blue()));
            source = cause.source();

        }

        self.exit_code()

    }

    fn dye ( text: &str, style: Style ) -> String {

        match *TINT.get_or_init(Self::tinted) {
            true => text.style(style).to_string(),
            false => text.to_string(),
        }

    }

    fn tinted () -> bool {

        if std::env::var_os("NO_COLOR").is_some() { return false; }

        if std::env::var("TERM").is_ok_and(|term| term.trim().eq_ignore_ascii_case("dumb")) { return false; }

        std::io::stderr().is_terminal()

    }

}
