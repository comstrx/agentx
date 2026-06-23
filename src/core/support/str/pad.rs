use super::Str;

impl Str {

    pub fn pad_left ( s: &str, width: usize, fill: char ) -> String {

        let len = Self::len(s);

        if len >= width { return s.to_owned(); }

        let mut out = String::with_capacity(width);

        for _ in 0..width - len { out.push(fill); }

        out.push_str(s);
        out

    }

    pub fn pad_right ( s: &str, width: usize, fill: char ) -> String {

        let len = Self::len(s);

        if len >= width { return s.to_owned(); }

        let mut out = String::with_capacity(width);
        out.push_str(s);

        for _ in 0..width - len { out.push(fill); }

        out

    }

    pub fn center ( s: &str, width: usize, fill: char ) -> String {

        let len = Self::len(s);

        if len >= width { return s.to_owned(); }

        let total = width - len;
        let left = total / 2;
        let right = total - left;

        let mut out = String::with_capacity(width);

        for _ in 0..left { out.push(fill); }

        out.push_str(s);

        for _ in 0..right { out.push(fill); }

        out

    }

    pub fn indent ( s: &str, prefix: &str ) -> String {

        let mut out = String::with_capacity(s.len() + prefix.len());

        for ( index, line ) in s.lines().enumerate() {

            if index > 0 { out.push('\n'); }

            if !line.is_empty() {

                out.push_str(prefix);
                out.push_str(line);

            }

        }

        out

    }

    pub fn dedent ( s: &str ) -> String {

        let trim = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        if trim == 0 { return s.to_owned(); }

        let mut out = String::with_capacity(s.len());

        for ( index, line ) in s.lines().enumerate() {

            if index > 0 { out.push('\n'); }

            out.push_str(line.get(trim..).unwrap_or(""));

        }

        out

    }

}
