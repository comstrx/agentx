use std::io::IsTerminal;
use std::sync::OnceLock;
use owo_colors::{OwoColorize, Style};

use super::arch::Ui;

static COLOR: OnceLock<bool> = OnceLock::new();

impl Ui {

    pub fn blank () {

        println!();

    }

    pub fn rule ( label: &str ) {

        let head = format!("── {label} ");
        let fill = 66usize.saturating_sub(head.chars().count());

        println!();
        println!("{}", Self::paint(&format!("{head}{}", "─".repeat(fill)), Style::new().bright_cyan().bold()));

    }

    pub fn step ( message: &str ) {

        Self::emit(0, "▸", Style::new().bright_cyan().bold(), message);

    }

    pub fn ok ( message: &str ) {

        Self::emit(0, "✓", Style::new().bright_green().bold(), message);

    }

    pub fn warn ( message: &str ) {

        Self::emit(0, "▲", Style::new().yellow().bold(), message);

    }

    pub fn info ( message: &str ) {

        Self::emit(0, "·", Style::new().bright_black().bold(), message);

    }

    pub fn arrow ( depth: usize, message: &str ) {

        Self::emit(depth, "▸", Style::new().bright_cyan().bold(), message);

    }

    pub fn tick ( depth: usize, message: &str ) {

        Self::emit(depth, "✓", Style::new().bright_green().bold(), message);

    }

    pub fn cross ( depth: usize, message: &str ) {

        Self::emit(depth, "✗", Style::new().bright_red().bold(), message);

    }

    pub fn bang ( depth: usize, message: &str ) {

        Self::emit(depth, "▲", Style::new().yellow().bold(), message);

    }

    pub fn dot ( depth: usize, message: &str ) {

        Self::emit(depth, "·", Style::new().bright_black().bold(), &Self::paint(message, Style::new().bright_black()));

    }

    pub fn beat ( depth: usize, message: &str ) {

        Self::emit(depth, "↻", Style::new().bright_magenta().bold(), message);

    }

    pub fn detail ( label: &str, value: &str ) {

        println!("      {}  {value}", Self::paint(&format!("{label:<13}"), Style::new().bright_black()));

    }

    pub fn title ( text: &str ) {

        println!("{}", Self::paint(text, Style::new().bright_magenta().bold()));

    }

    pub fn head ( text: &str ) {

        println!("{}", Self::paint(text, Style::new().bright_cyan().bold()));

    }

    pub fn field ( label: &str, value: &str ) {

        println!("  {}  {value}", Self::paint(&format!("{label:<15}"), Style::new().bright_black()));

    }

    pub fn pair ( key: &str, value: &str ) {

        println!("  {} {value}", Self::paint(&format!("{key:<18}="), Style::new().bright_black()));

    }

    pub fn item ( value: &str ) {

        println!("      {}", Self::paint(value, Style::new().bright_black()));

    }

    fn emit ( depth: usize, glyph: &str, style: Style, message: &str ) {

        println!("{}{}  {message}", "  ".repeat(depth + 1), Self::paint(glyph, style));

    }

    fn paint ( text: &str, style: Style ) -> String {

        if Self::tinted() { text.style(style).to_string() } else { text.to_string() }

    }

    fn tinted () -> bool {

        *COLOR.get_or_init(|| std::io::stdout().is_terminal())

    }

}
