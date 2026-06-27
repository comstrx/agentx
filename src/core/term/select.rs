use std::io::{self, Write};
use owo_colors::{OwoColorize, Style};

use crate::core::error::AppResult;
use super::arch::{Key, Term};

impl Term {

    pub fn select ( prompt: &str, options: &[String], default: usize ) -> AppResult<Option<usize>> {

        if options.is_empty() { return Ok(None); }

        let last = options.len() - 1;
        let mut index = default.min(last);

        if !Self::is_tty() { return Ok(Some(index)); }

        let _guard = Self::enter_raw()?;
        let mut input = io::stdin().lock();

        print!("\x1b[?25l");
        Self::render(prompt, options, index);

        loop {

            match Self::read_key(&mut input) {
                Key::Up     => index = if index == 0 { last } else { index - 1 },
                Key::Down   => index = if index == last { 0 } else { index + 1 },
                Key::Enter  => { Self::erase(options.len() + 1); return Ok(Some(index)); }
                Key::Cancel => { Self::erase(options.len() + 1); return Ok(None); }
                Key::Other  => continue,
            }

            Self::erase(options.len() + 1);
            Self::render(prompt, options, index);

        }

    }

    fn render ( prompt: &str, options: &[String], index: usize ) {

        println!("{}", prompt.style(Style::new().bright_black()));

        for ( position, option ) in options.iter().enumerate() {

            if position == index {

                println!("  {} {}", "❯".style(Style::new().bright_cyan().bold()), option.style(Style::new().bright_cyan().bold()));

            }
            else {

                println!("    {}", option.style(Style::new().bright_black()));

            }

        }

        let _ = io::stdout().flush();

    }

    fn erase ( rows: usize ) {

        print!("\x1b[{rows}A\x1b[0J");
        let _ = io::stdout().flush();

    }

}
