use super::Str;

impl Str {

    pub fn contains ( s: &str, needle: &str ) -> bool {

        s.contains(needle)

    }

    pub fn starts_with ( s: &str, prefix: &str ) -> bool {

        s.starts_with(prefix)

    }

    pub fn ends_with ( s: &str, suffix: &str ) -> bool {

        s.ends_with(suffix)

    }

    pub fn find ( s: &str, needle: &str ) -> Option<usize> {

        s.find(needle).map(|byte| Self::char_index(s, byte))

    }

    pub fn rfind ( s: &str, needle: &str ) -> Option<usize> {

        s.rfind(needle).map(|byte| Self::char_index(s, byte))

    }

    pub fn count ( s: &str, needle: &str ) -> usize {

        if needle.is_empty() { return 0; }
        s.matches(needle).count()

    }

    pub fn count_char ( s: &str, c: char ) -> usize {

        if c.is_ascii() && s.is_ascii() {

            return memchr::memchr_iter(c as u8, s.as_bytes()).count();

        }

        s.chars().filter(|&x| x == c).count()

    }

    pub fn count_lines ( s: &str ) -> usize {

        s.lines().count()

    }

    pub fn count_words ( s: &str ) -> usize {

        s.split_whitespace().count()

    }

    pub fn before ( s: &str, sep: &str ) -> Option<String> {

        s.split_once(sep).map(|( head, _ )| head.to_owned())

    }

    pub fn after ( s: &str, sep: &str ) -> Option<String> {

        s.split_once(sep).map(|( _, tail )| tail.to_owned())

    }

    pub fn between ( s: &str, open: &str, close: &str ) -> Option<String> {

        let start = s.find(open)? + open.len();
        let rest = &s[start..];
        let end = rest.find(close)?;

        Some(rest[..end].to_owned())

    }

    pub fn includes_any ( s: &str, needles: &[&str] ) -> bool {

        needles.iter().any(|needle| s.contains(needle))

    }

    pub fn includes_all ( s: &str, needles: &[&str] ) -> bool {

        needles.iter().all(|needle| s.contains(needle))

    }

    pub fn char_index ( s: &str, byte: usize ) -> usize {

        if s.is_ascii() { byte } else { s[..byte].chars().count() }

    }

    pub fn common_prefix ( a: &str, b: &str ) -> String {

        a.chars().zip(b.chars()).take_while(|( x, y )| x == y).map(|( x, _ )| x).collect()

    }

    pub fn common_suffix ( a: &str, b: &str ) -> String {

        let mut out: Vec<char> = a.chars().rev().zip(b.chars().rev()).take_while(|( x, y )| x == y).map(|( x, _ )| x).collect();
        out.reverse();

        out.into_iter().collect()

    }

}
