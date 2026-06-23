use super::Str;

impl Str {

    pub fn is_empty ( s: &str ) -> bool {

        s.is_empty()

    }

    pub fn is_blank ( s: &str ) -> bool {

        s.trim().is_empty()

    }

    pub fn is_ascii ( s: &str ) -> bool {

        s.is_ascii()

    }

    pub fn is_numeric ( s: &str ) -> bool {

        !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())

    }

    pub fn is_alpha ( s: &str ) -> bool {

        !s.is_empty() && s.chars().all(char::is_alphabetic)

    }

    pub fn is_alnum ( s: &str ) -> bool {

        !s.is_empty() && s.chars().all(char::is_alphanumeric)

    }

    pub fn is_identifier ( s: &str ) -> bool {

        let mut chars = s.chars();

        match chars.next() {
            Some(first) if first.is_alphabetic() || first == '_' => chars.all(|c| c.is_alphanumeric() || c == '_'),
            _ => false,
        }

    }

    pub fn eq_ignore_case ( a: &str, b: &str ) -> bool {

        a.eq_ignore_ascii_case(b)

    }

}
