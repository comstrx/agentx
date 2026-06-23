use super::arch::{Backend, Claude, Codex};

impl Backend {

    pub(crate) fn select ( model: &str ) -> Self {

        if model.starts_with("codex") { Self::Codex(Codex::new()) } else { Self::Claude(Claude::new()) }

    }

}
