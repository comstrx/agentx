use crate::config::base::consts::{BAR_WIDTH, GLYPH_FULL, GLYPH_INFO, GLYPH_OFF, GLYPH_ON, GLYPH_REST, GLYPH_TITLE};
use crate::core::error::AppResult;
use crate::core::term::Term;
use crate::app::{Mark, Ui};

impl Ui {

    pub fn ask ( prompt: &str, hint: &str ) -> AppResult<Option<String>> {

        Self::settle();

        let answer = Term::input(prompt, hint);

        Self::printed();

        answer

    }

    pub fn choose ( prompt: &str, options: &[String], default: usize ) -> AppResult<Option<usize>> {

        Self::settle();

        let picked = Term::select(prompt, options, default);

        Self::printed();

        picked

    }

    pub fn detail ( label: &str, value: &str ) {

        Self::line(&format!("      {}  {value}", Self::paint(&format!("{label:<13}"), Self::muted())));

    }

    pub fn title ( text: &str ) {

        Self::blank();
        Self::line(&format!("{} {}", Self::paint(Self::pick(GLYPH_TITLE), Self::brand()), Self::paint(text, Self::brand())));
        Self::blank();

    }

    pub fn head ( text: &str ) {

        Self::blank();
        Self::line(&Self::paint(text, Self::accent()));
        Self::blank();

    }

    pub fn field ( label: &str, value: &str ) {

        Self::line(&format!("  {}  {value}", Self::paint(&format!("{label:<18}"), Self::muted())));

    }

    pub fn pair ( key: &str, value: &str ) {

        Self::line(&format!("  {} {value}", Self::paint(&format!("{key:<18}="), Self::muted())));

    }

    pub fn item ( value: &str ) {

        Self::line(&format!("      {value}"));

    }

    pub fn log ( line: &str ) {

        Self::line(line);

    }

    pub(crate) fn strong ( text: &str ) -> String {

        Self::paint(text, Self::accent())

    }

    pub(crate) fn keys () -> &'static str {

        if Term::icons() { "↑/↓ move" } else { "j/k move" }

    }

    pub(crate) fn dim ( text: &str ) -> String {

        Self::paint(text, Self::muted())

    }

    pub fn role ( label: &str, members: &str ) {

        Self::line(&format!("      {}  {}{}", Self::paint(Self::pick(GLYPH_INFO), Self::accent()), Self::paint(&format!("{label:<18}"), Self::muted()), Self::paint(members, Self::good())));

    }

    pub fn task ( name: &str, status: &str ) {

        let mark = match status {
            "shipped"   => Mark::Ok,
            "executing" => Mark::Beat,
            "blocked"   => Mark::Fail,
            _           => Mark::Info,
        };

        let ( glyph, style ) = Self::glyph(mark);

        Self::line(&format!("      {}  {name:<34}{}", Self::paint(glyph, style), Self::paint(status, style)));

    }

    pub fn state ( label: &str, on: bool, value: &str ) {

        let glyph = match on {
            true => Self::paint(Self::pick(GLYPH_ON), Self::good().bold()),
            false => Self::pick(GLYPH_OFF).to_string(),
        };

        Self::line(&format!("  {glyph}  {}  {value}", Self::paint(&format!("{label:<14}"), Self::muted())));

    }

    pub fn bar ( done: usize, total: usize ) -> String {

        let ( filled, percent ) = match total {
            0 => ( 0, 0 ),
            _ => ( done * BAR_WIDTH / total, done * 100 / total ),
        };

        let full = Self::paint(&Self::pick(GLYPH_FULL).repeat(filled), Self::good());
        let rest = Self::pick(GLYPH_REST).repeat(BAR_WIDTH.saturating_sub(filled));

        format!("{full}{rest}  {percent}%")

    }

}
