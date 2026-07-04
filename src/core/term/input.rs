use std::io::{self, BufRead, Write};
use owo_colors::{OwoColorize, Style};

use crate::core::error::AppResult;
use super::arch::Term;

impl Term {

    pub fn input ( prompt: &str, hint: &str ) -> AppResult<Option<String>> {

        if !Self::is_tty() || !Self::ansi() { return Ok(None); }

        let cursor = if Self::icons() { "❯" } else { ">" };

        println!("{}   {}", Self::tint(prompt, Style::new().bright_blue()), Self::tint(hint, Style::new().bright_blue()));
        print!("  {} ", Self::tint(cursor, Style::new().bright_cyan().bold()));

        let _ = io::stdout().flush();

        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;

        let value = line.trim();

        match value.is_empty() {
            true => Ok(None),
            false => Ok(Some(value.to_string())),
        }

    }

    pub(super) fn tint ( text: &str, style: Style ) -> String {

        if Self::colors() { text.style(style).to_string() } else { text.to_string() }

    }

}
