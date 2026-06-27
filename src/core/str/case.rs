use super::Str;

impl Str {

    pub fn lower ( s: &str ) -> String {

        s.to_lowercase()

    }

    pub fn upper ( s: &str ) -> String {

        s.to_uppercase()

    }

    pub fn capitalize ( s: &str ) -> String {

        let mut chars = s.chars();

        match chars.next() {
            Some(first) => first.to_uppercase().chain(chars.flat_map(char::to_lowercase)).collect(),
            None => String::new(),
        }

    }

    pub fn title ( s: &str ) -> String {

        let mut out = String::with_capacity(s.len());
        let mut fresh = true;

        for ch in s.chars() {

            if ch.is_whitespace() {

                fresh = true;
                out.push(ch);

            }
            else if fresh {

                out.extend(ch.to_uppercase());
                fresh = false;

            }
            else {

                out.extend(ch.to_lowercase());

            }

        }

        out

    }

    pub fn swap_case ( s: &str ) -> String {

        let mut out = String::with_capacity(s.len());

        for ch in s.chars() {

            if ch.is_uppercase() { out.extend(ch.to_lowercase()); }
            else if ch.is_lowercase() { out.extend(ch.to_uppercase()); }
            else { out.push(ch); }

        }

        out

    }

    pub fn snake ( s: &str ) -> String {

        Self::delimit(s, '_')

    }

    pub fn kebab ( s: &str ) -> String {

        Self::delimit(s, '-')

    }

    pub fn pascal ( s: &str ) -> String {

        let mut out = String::with_capacity(s.len());

        for word in Self::word_parts(s) {

            let mut chars = word.chars();

            if let Some(first) = chars.next() {

                out.extend(first.to_uppercase());
                out.extend(chars.flat_map(char::to_lowercase));

            }

        }

        out

    }

    pub fn camel ( s: &str ) -> String {

        let pascal = Self::pascal(s);
        let mut chars = pascal.chars();

        match chars.next() {
            Some(first) => first.to_lowercase().chain(chars).collect(),
            None => String::new(),
        }

    }

    fn delimit ( s: &str, sep: char ) -> String {

        let mut out = String::with_capacity(s.len() + 4);

        for ( index, word ) in Self::word_parts(s).iter().enumerate() {

            if index > 0 { out.push(sep); }
            out.extend(word.chars().flat_map(char::to_lowercase));

        }

        out

    }

    fn word_parts ( s: &str ) -> Vec<String> {

        let mut parts = Vec::new();
        let mut current = String::new();
        let mut prev_lower = false;

        for ch in s.chars() {

            if ch.is_alphanumeric() {

                if ch.is_uppercase() && prev_lower && !current.is_empty() {

                    parts.push(std::mem::take(&mut current));

                }

                current.push(ch);
                prev_lower = ch.is_lowercase() || ch.is_numeric();

            }
            else if !current.is_empty() {

                parts.push(std::mem::take(&mut current));
                prev_lower = false;

            }
            else {

                prev_lower = false;

            }

        }

        if !current.is_empty() { parts.push(current); }

        parts

    }

}
