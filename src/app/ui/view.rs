use owo_colors::Style;

use crate::app::Ui;

impl Ui {

    pub fn detail ( label: &str, value: &str ) {

        Self::line(&format!("      {}  {value}", Self::paint(&format!("{label:<13}"), Style::new().bright_black())));

    }

    pub fn title ( text: &str ) {

        Self::line(&Self::paint(text, Style::new().bright_magenta().bold()));

    }

    pub fn head ( text: &str ) {

        Self::line(&Self::paint(text, Style::new().bright_cyan().bold()));

    }

    pub fn field ( label: &str, value: &str ) {

        Self::line(&format!("  {}  {value}", Self::paint(&format!("{label:<15}"), Style::new().bright_black())));

    }

    pub fn pair ( key: &str, value: &str ) {

        Self::line(&format!("  {} {value}", Self::paint(&format!("{key:<18}="), Style::new().bright_black())));

    }

    pub fn item ( value: &str ) {

        Self::line(&format!("      {}", Self::paint(value, Style::new().bright_black())));

    }

    pub fn log ( line: &str ) {

        Self::line(&Self::paint(line, Style::new().bright_black()));

    }

    pub fn role ( label: &str, members: &str ) {

        Self::line(&format!("      {}  {}{}", Self::paint("▸", Style::new().bright_cyan().bold()), Self::paint(&format!("{label:<12}"), Style::new().bright_black()), Self::paint(members, Style::new().bright_green())));

    }

    pub fn task ( name: &str, status: &str ) {

        let ( glyph, style ) = match status {
            "shipped"   => ( "✓", Style::new().bright_green().bold() ),
            "executing" => ( "↻", Style::new().bright_magenta().bold() ),
            "blocked"   => ( "✗", Style::new().bright_red().bold() ),
            _           => ( "·", Style::new().bright_black().bold() ),
        };

        Self::line(&format!("      {}  {}{}", Self::paint(glyph, style), Self::paint(&format!("{name:<34}"), Style::new().bright_black()), Self::paint(status, style)));

    }

    pub fn state ( label: &str, on: bool, value: &str ) {

        let glyph = match on {
            true => Self::paint("●", Style::new().bright_green().bold()),
            false => Self::paint("○", Style::new().bright_black()),
        };

        Self::line(&format!("  {glyph}  {}  {value}", Self::paint(&format!("{label:<11}"), Style::new().bright_black())));

    }

    pub fn bar ( done: usize, total: usize ) -> String {

        let width = 18;

        let ( filled, percent ) = match total {
            0 => ( 0, 0 ),
            _ => ( done * width / total, done * 100 / total ),
        };

        let full = Self::paint(&"█".repeat(filled), Style::new().bright_green());
        let rest = Self::paint(&"░".repeat(width.saturating_sub(filled)), Style::new().bright_black());

        format!("{full}{rest}  {percent}%")

    }

}
