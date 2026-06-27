use owo_colors::Style;

use crate::app::Ui;

impl Ui {

    pub fn blank () {

        Self::line("");

    }

    pub fn rule ( label: &str ) {

        let head = format!("── {label} ");
        let fill = 68usize.saturating_sub(head.chars().count()).max(2);

        Self::line("");
        Self::line(&Self::paint(&format!("{head}{}", "─".repeat(fill)), Style::new().bright_cyan().bold()));
        Self::line("");

    }

    pub fn step ( message: &str ) {

        Self::emit(0, "▸", Style::new().bright_cyan().bold(), message);
        Self::busy(message);

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
        Self::busy(message);

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

}
