use super::Str;

impl Str {

    pub fn ends_with_vowel_before_y ( lower: &str ) -> bool {

        let bytes = lower.as_bytes();
        if bytes.len() < 2 { return false; }

        matches!(bytes[bytes.len() - 2], b'a' | b'e' | b'i' | b'o' | b'u')

    }

    pub fn plural ( word: &str ) -> String {

        if word.is_empty() { return String::new(); }

        let lower = word.to_ascii_lowercase();

        if lower.ends_with('y') && !Self::ends_with_vowel_before_y(&lower) {

            return format!("{}ies", &word[..word.len() - 1]);

        }

        if lower.ends_with('s') || lower.ends_with("sh") || lower.ends_with("ch") || lower.ends_with('x') || lower.ends_with('z') {

            return format!("{word}es");

        }

        format!("{word}s")

    }

    pub fn singular ( word: &str ) -> String {

        let lower = word.to_ascii_lowercase();

        if lower.ends_with("ies") && word.len() > 3 {

            return format!("{}y", &word[..word.len() - 3]);

        }

        if lower.ends_with("shes") || lower.ends_with("ches") || lower.ends_with("xes") || lower.ends_with("zes") || lower.ends_with("ses") {

            return word[..word.len() - 2].to_owned();

        }

        if lower.ends_with('s') && !lower.ends_with("ss") {

            return word[..word.len() - 1].to_owned();

        }

        word.to_owned()

    }

    pub fn ordinal ( n: i64 ) -> String {

        let abs = n.unsigned_abs() % 100;

        let suffix = if ( 11..=13 ).contains(&abs) {

            "th"

        }
        else {

            match abs % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }

        };

        format!("{n}{suffix}")

    }

    pub fn article ( word: &str ) -> &'static str {

        match word.chars().next().map(|c| c.to_ascii_lowercase()) {
            Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
            _ => "a",
        }

    }

}
