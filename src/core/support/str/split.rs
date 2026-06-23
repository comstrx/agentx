use super::Str;

impl Str {

    pub fn split ( s: &str, sep: &str ) -> Vec<String> {

        if sep.is_empty() { return s.chars().map(String::from).collect(); }
        s.split(sep).map(str::to_owned).collect()

    }

    pub fn split_n ( s: &str, sep: &str, n: usize ) -> Vec<String> {

        s.splitn(n, sep).map(str::to_owned).collect()

    }

    pub fn lines ( s: &str ) -> Vec<String> {

        s.lines().map(str::to_owned).collect()

    }

    pub fn nonempty_lines ( s: &str ) -> Vec<String> {

        s.lines().filter(|line| !line.trim().is_empty()).map(str::to_owned).collect()

    }

    pub fn words ( s: &str ) -> Vec<String> {

        s.split_whitespace().map(str::to_owned).collect()

    }

    pub fn split_once ( s: &str, sep: &str ) -> Option<( String, String )> {

        s.split_once(sep).map(|( a, b )| ( a.to_owned(), b.to_owned() ))

    }

    pub fn rsplit_once ( s: &str, sep: &str ) -> Option<( String, String )> {

        s.rsplit_once(sep).map(|( a, b )| ( a.to_owned(), b.to_owned() ))

    }

    pub fn chunks ( s: &str, size: usize ) -> Vec<String> {

        if size == 0 { return Vec::new(); }

        let chars: Vec<char> = s.chars().collect();
        chars.chunks(size).map(|chunk| chunk.iter().collect()).collect()

    }

}
