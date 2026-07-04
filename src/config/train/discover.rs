use crate::core::text::Text;
use super::arch::Train;

impl Train {

    pub(crate) fn catalogue () -> String {

        let names = Self::available();

        if names.is_empty() { return "  (none yet — this is the first project of its kind)".to_string(); }

        let blocks: Vec<String> = names.iter().map(|name| {

            let stack = Self::stack(&Self::project(name));
            let title = stack.name.trim();
            let description = stack.description.trim();

            let body = match ( title.is_empty(), description.is_empty() ) {
                ( false, false ) => format!("{title} — {description}"),
                ( false, true )  => title.to_string(),
                ( true, false )  => description.to_string(),
                ( true, true )   => name.clone(),
            };

            format!("### {name}\n{body}")

        }).collect();

        blocks.join("\n\n")

    }

    pub(crate) fn parse_type ( body: &str ) -> Option<( bool, String )> {

        let value = Self::parse_line(body, "type:")?;

        let ( fresh, name ) = match value.get(..4) {
            Some(head) if head.eq_ignore_ascii_case("new ") => ( true, value[4..].trim() ),
            _ => ( false, value.as_str() ),
        };

        let slug = Text::slug(name);

        if slug.is_empty() { None } else { Some(( fresh, slug )) }

    }

    pub(crate) fn parse_line ( body: &str, label: &str ) -> Option<String> {

        let value = body.lines().map(str::trim).find_map(|line| {

            let head = line.get(..label.len())?;

            head.eq_ignore_ascii_case(label).then(|| line[label.len()..].trim().to_string())

        })?;

        if value.is_empty() || value.eq_ignore_ascii_case("none") { None } else { Some(value) }

    }

}
