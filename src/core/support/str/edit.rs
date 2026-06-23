use super::Str;

impl Str {

    pub fn replace ( s: &str, from: &str, to: &str ) -> String {

        s.replace(from, to)

    }

    pub fn replace_first ( s: &str, from: &str, to: &str ) -> String {

        s.replacen(from, to, 1)

    }

    pub fn replace_n ( s: &str, from: &str, to: &str, n: usize ) -> String {

        s.replacen(from, to, n)

    }

    pub fn remove ( s: &str, part: &str ) -> String {

        if part.is_empty() { return s.to_owned(); }
        s.replace(part, "")

    }

    pub fn repeat ( s: &str, n: usize ) -> String {

        s.repeat(n)

    }

    pub fn reverse ( s: &str ) -> String {

        s.chars().rev().collect()

    }

    pub fn insert ( s: &str, index: usize, part: &str ) -> String {

        let at = Self::byte_pos(s, index);
        let mut out = String::with_capacity(s.len() + part.len());

        out.push_str(&s[..at]);
        out.push_str(part);
        out.push_str(&s[at..]);

        out

    }

    pub fn ensure_prefix ( s: &str, prefix: &str ) -> String {

        if s.starts_with(prefix) { s.to_owned() } else { format!("{prefix}{s}") }

    }

    pub fn ensure_suffix ( s: &str, suffix: &str ) -> String {

        if s.ends_with(suffix) { s.to_owned() } else { format!("{s}{suffix}") }

    }

    pub fn unprefix ( s: &str, prefix: &str ) -> String {

        s.strip_prefix(prefix).unwrap_or(s).to_owned()

    }

    pub fn unsuffix ( s: &str, suffix: &str ) -> String {

        s.strip_suffix(suffix).unwrap_or(s).to_owned()

    }

    pub fn trim ( s: &str ) -> String {

        s.trim().to_owned()

    }

    pub fn squeeze ( s: &str ) -> String {

        let mut out = String::with_capacity(s.len());
        let mut space = false;

        for ch in s.chars() {

            if ch.is_whitespace() {

                if !space && !out.is_empty() { out.push(' '); }
                space = true;

            }
            else {

                out.push(ch);
                space = false;

            }

        }

        out.trim_end().to_owned()

    }

    pub fn truncate ( s: &str, max: usize, ellipsis: &str ) -> String {

        if Self::len(s) <= max { return s.to_owned(); }

        let keep = max.saturating_sub(Self::len(ellipsis));
        format!("{}{ellipsis}", Self::take(s, keep))

    }

    pub fn trim_start ( s: &str ) -> String {

        s.trim_start().to_owned()

    }

    pub fn trim_end ( s: &str ) -> String {

        s.trim_end().to_owned()

    }

    pub fn trim_matches ( s: &str, ch: char ) -> String {

        s.trim_matches(ch).to_owned()

    }

    pub fn template <V: AsRef<str>> ( text: &str, vars: &[( &str, V )] ) -> String {

        let mut out = text.to_string();

        for ( key, value ) in vars {

            out = out.replace(&format!("{{{key}}}"), value.as_ref());

        }

        out

    }

}
