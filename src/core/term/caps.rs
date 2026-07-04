use std::io::{self, IsTerminal};
use std::sync::OnceLock;

use crate::core::env::Env;
use super::arch::Term;

static COLORS: OnceLock<bool> = OnceLock::new();
static ICONS: OnceLock<bool> = OnceLock::new();
static ANSI: OnceLock<bool> = OnceLock::new();

impl Term {

    pub fn colors () -> bool {

        *COLORS.get_or_init(Self::color_support)

    }

    pub fn icons () -> bool {

        *ICONS.get_or_init(Self::icon_support)

    }

    pub fn ansi () -> bool {

        *ANSI.get_or_init(|| !Self::dumb() && io::stdout().is_terminal())

    }

    fn color_support () -> bool {

        if Self::dumb() || Env::has("NO_COLOR") { return false; }

        if Self::forced("CLICOLOR_FORCE") || Self::forced("FORCE_COLOR") { return true; }

        io::stdout().is_terminal()

    }

    fn icon_support () -> bool {

        if Self::dumb() || !io::stdout().is_terminal() { return false; }

        let locale = ["LC_ALL", "LC_CTYPE", "LANG"]
            .iter()
            .find_map(|key| Env::get(key).filter(|value| !value.trim().is_empty()))
            .unwrap_or_default()
            .to_ascii_lowercase();

        locale.contains("utf-8") || locale.contains("utf8")

    }

    fn dumb () -> bool {

        Env::get("TERM").is_some_and(|term| term.trim().eq_ignore_ascii_case("dumb"))

    }

    fn forced ( key: &str ) -> bool {

        Env::get(key).is_some_and(|value| !value.trim().is_empty() && value.trim() != "0")

    }

}
