use super::arch::{Backend, Claude, Codex, Worker};

const KNOWN: &[&str] = &["claude", "codex"];

impl Worker {

    pub fn resolve ( model: &str ) -> Option<&'static str> {

        let name = model.trim().to_ascii_lowercase();

        KNOWN.iter().copied().find(|key| name.starts_with(key))
            .or_else(|| KNOWN.iter().copied().find(|key| name.contains(key)))

    }

    pub(crate) fn make ( model: &str ) -> Box<dyn Backend> {

        match Self::resolve(model) {
            Some("codex") => Box::new(Codex::new()),
            _ => Box::new(Claude::new()),
        }

    }

}
