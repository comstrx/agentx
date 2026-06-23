use super::Str;

impl Str {

    pub fn len ( s: &str ) -> usize {

        if s.is_ascii() { return s.len(); }
        s.chars().count()

    }

    pub fn byte_pos ( s: &str, index: usize ) -> usize {

        if s.is_ascii() { return index.min(s.len()); }

        for ( count, ( pos, _ ) ) in s.char_indices().enumerate() {

            if count == index { return pos; }

        }

        s.len()

    }

    pub fn slice ( s: &str, start: usize, end: usize ) -> String {

        if start >= end { return String::new(); }

        let a = Self::byte_pos(s, start);
        let b = Self::byte_pos(s, end);

        if a >= b { return String::new(); }

        s[a..b].to_owned()

    }

    pub fn mid ( s: &str, start: usize, len: usize ) -> String {

        if len == 0 { return String::new(); }
        Self::slice(s, start, start.saturating_add(len))

    }

    pub fn take ( s: &str, n: usize ) -> String {

        if n == 0 { return String::new(); }
        s[..Self::byte_pos(s, n)].to_owned()

    }

    pub fn take_end ( s: &str, n: usize ) -> String {

        if n == 0 { return String::new(); }

        let total = Self::len(s);
        if n >= total { return s.to_owned(); }

        s[Self::byte_pos(s, total - n)..].to_owned()

    }

    pub fn skip ( s: &str, n: usize ) -> String {

        if n == 0 { return s.to_owned(); }
        s[Self::byte_pos(s, n)..].to_owned()

    }

    pub fn skip_end ( s: &str, n: usize ) -> String {

        if n == 0 { return s.to_owned(); }

        let total = Self::len(s);
        if n >= total { return String::new(); }

        s[..Self::byte_pos(s, total - n)].to_owned()

    }

    pub fn char_at ( s: &str, index: usize ) -> Option<char> {

        s.chars().nth(index)

    }

    pub fn first ( s: &str ) -> Option<char> {

        s.chars().next()

    }

    pub fn last ( s: &str ) -> Option<char> {

        s.chars().next_back()

    }

}
