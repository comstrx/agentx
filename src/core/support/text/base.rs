use std::cmp::Ordering;

use super::Text;

impl Text {

    /// Compare two strings the way a human reads numbered files:
    /// `2-x` before `10-x`. Case-insensitive on letters.
    #[inline]
    pub fn natural_compare ( left: &str, right: &str ) -> Ordering {

        let mut a = left.chars().peekable();
        let mut b = right.chars().peekable();

        loop {

            match ( a.peek().copied(), b.peek().copied() ) {

                ( None, None ) => return Ordering::Equal,
                ( None, Some(_) ) => return Ordering::Less,
                ( Some(_), None ) => return Ordering::Greater,

                ( Some(x), Some(y) ) if x.is_ascii_digit() && y.is_ascii_digit() => {

                    match take_number(&mut a).cmp(&take_number(&mut b)) {
                        Ordering::Equal => continue,
                        other => return other,
                    }

                }

                ( Some(x), Some(y) ) => {

                    let ( lx, ly ) = ( x.to_ascii_lowercase(), y.to_ascii_lowercase() );

                    if lx != ly {
                        return lx.cmp(&ly);
                    }

                    a.next();
                    b.next();

                }
            }
        }

    }

    /// True if the last non-empty line equals `token` (case-insensitive) — the
    /// convergence check (`ship it`).
    #[inline]
    pub fn last_line_is ( body: &str, token: &str ) -> bool {

        match body.lines().rfind(|line| !line.trim().is_empty()) {
            Some(line) => line.trim().eq_ignore_ascii_case(token),
            None => false,
        }

    }

    /// Parse a control file: the first `ACTION:` and `NOTE:` lines.
    /// `ACTION` is reduced to its alphabetic core, lowercased (`ship`/`revise`).
    pub fn parse_control ( body: &str ) -> ( String, String ) {

        let mut action = String::new();
        let mut note = String::new();

        for line in body.lines() {

            let trimmed = line.trim();

            if action.is_empty()
                && let Some(rest) = strip_prefix_ci(trimmed, "ACTION:")
            {
                action = rest.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>().to_ascii_lowercase();
            }

            if note.is_empty()
                && let Some(rest) = strip_prefix_ci(trimmed, "NOTE:")
            {
                note = rest.trim().to_string();
            }
        }

        ( action, note )

    }

    /// `Some-Value name` -> `some_value_name`.
    pub fn snake ( value: &str ) -> String {

        convert_case(value, '_')

    }

    /// `Some Value` -> `some-value`.
    pub fn kebab ( value: &str ) -> String {

        convert_case(value, '-')

    }

    /// ASCII, lowercase, non-alphanumeric collapsed to `-` (no case-splitting).
    pub fn slug ( value: &str ) -> String {

        let mut out = String::with_capacity(value.len());
        let mut pending = false;

        for ch in value.chars() {

            if ch.is_ascii_alphanumeric() {

                if pending && !out.is_empty() {
                    out.push('-');
                }

                out.push(ch.to_ascii_lowercase());
                pending = false;

            } else {
                pending = true;
            }
        }

        out

    }

}

#[inline]
fn take_number ( chars: &mut std::iter::Peekable<std::str::Chars<'_>> ) -> u64 {

    let mut value: u64 = 0;

    while let Some(c) = chars.peek().copied() {

        if !c.is_ascii_digit() {
            break;
        }

        value = value.saturating_mul(10).saturating_add((c as u8 - b'0') as u64);
        chars.next();
    }

    value

}

fn strip_prefix_ci<'a> ( text: &'a str, prefix: &str ) -> Option<&'a str> {

    if text.len() >= prefix.len() && text[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(text[prefix.len()..].trim())
    } else {
        None
    }

}

fn convert_case ( value: &str, sep: char ) -> String {

    let mut out = String::with_capacity(value.len() + 4);
    let mut pending = false;
    let mut prev_lower = false;

    for ch in value.chars() {

        if ch.is_ascii_alphanumeric() {

            let boundary = ( ch.is_ascii_uppercase() && prev_lower ) || pending;

            if boundary && !out.is_empty() {
                out.push(sep);
            }

            out.push(ch.to_ascii_lowercase());
            prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
            pending = false;

        } else {
            pending = true;
            prev_lower = false;
        }
    }

    out

}
