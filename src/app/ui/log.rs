use crate::config::base::consts::{GLYPH_RULE, RULE_WIDTH};
use crate::app::{Mark, Ui};

impl Ui {

    pub fn blank () {

        Self::line("");

    }

    pub fn point ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Step, message);

    }

    pub fn rule ( label: &str ) {

        let dash = Self::pick(GLYPH_RULE);
        let head = format!("{} {label} ", dash.repeat(2));
        let fill = RULE_WIDTH.saturating_sub(head.chars().count()).max(2);

        Self::blank();
        Self::line(&Self::paint(&format!("{head}{}", dash.repeat(fill)), Self::accent()));
        Self::blank();

    }

    pub fn step ( message: &str ) {

        Self::working(0, Mark::Step, message);

    }

    pub fn ok ( message: &str ) {

        Self::mark(0, Mark::Ok, message);

    }

    pub fn warn ( message: &str ) {

        Self::mark(0, Mark::Warn, message);

    }

    pub fn info ( message: &str ) {

        Self::mark(0, Mark::Info, message);

    }

    pub fn arrow ( depth: usize, message: &str ) {

        Self::working(depth, Mark::Step, message);

    }

    pub fn tick ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Ok, message);

    }

    pub fn cross ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Fail, message);

    }

    pub fn bang ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Warn, message);

    }

    pub fn dot ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Info, message);

    }

    pub fn beat ( depth: usize, message: &str ) {

        Self::mark(depth, Mark::Beat, message);

    }

}
