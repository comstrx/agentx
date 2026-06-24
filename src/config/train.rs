use std::collections::BTreeSet;
use std::path::{Path as StdPath, PathBuf};
use include_dir::{Dir as Embedded, DirEntry, include_dir};

use crate::core::error::AppResult;
use crate::core::support::env::Env;
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::proc::Proc;
use crate::core::support::text::Text;
use crate::core::worker::Worker;
use super::arch::Train;
use super::consts::CONSULT_TIMEOUT;
use super::prompts as P;

static INCLUDE: Embedded<'static> = include_dir!("$CARGO_MANIFEST_DIR/seed");

impl Train {

    pub fn init () -> AppResult<()> {

        Self::extract(&INCLUDE, &Self::store())

    }

    pub fn reset () -> AppResult<()> {

        Dir::remove(&Self::store());

        Self::init()

    }

    pub fn sync () -> AppResult<()> {

        Self::resync(&INCLUDE, &Self::store())

    }

    pub fn available () -> Vec<String> {

        let mut names: BTreeSet<String> = BTreeSet::new();

        for dir in Dir::subdirs(&Self::trains()) {

            names.insert(Path::name_of(&dir));

        }

        if let Some(train) = INCLUDE.get_dir("train") {

            for entry in train.dirs() {

                if let Some(name) = entry.path().file_name().and_then(|value| value.to_str()) {

                    names.insert(name.to_string());

                }

            }

        }

        names.into_iter().collect()

    }

    pub fn discover ( root: &StdPath, manager: &str, want_inspire: bool, want_gate: bool ) -> ( Option<String>, Option<String> ) {

        if !want_inspire && !want_gate { return ( None, None ); }

        let model = if manager.trim().is_empty() { "claude" } else { manager.trim() };
        let program = if model.starts_with("codex") { "codex" } else { "claude" };

        if Env::which(program).is_none() { return ( None, None ); }

        let mut worker = Worker::new(model);
        worker.cwd(root).timeout(CONSULT_TIMEOUT);

        let kind = if want_inspire {

            let prompt = P::MANAGER_DISCOVER.replace("{types}", &Self::catalogue());
            Self::ask(&mut worker, prompt).and_then(|body| Self::parse_type(&body))

        }
        else {

            None

        };

        if let Some(slug) = &kind { let _ = Self::create(slug); }

        let gate = if want_gate {

            Self::ask(&mut worker, P::MANAGER_GATE.to_string()).and_then(|body| Self::parse_line(&body, "gate:"))

        }
        else {

            None

        };

        ( kind, gate )

    }

    pub fn overview ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::trains().join(name).join("overview"))

    }

    pub fn contracts ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::trains().join(name).join("contracts"))

    }

    pub fn skills ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::trains().join(name).join("skills"))

    }

    pub fn requires ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::trains().join(name).join("requires"))

    }

    pub fn history ( name: &str ) -> Vec<PathBuf> {

        Dir::markdown(&Self::trains().join(name).join("history"))

    }

    pub fn about ( name: &str ) -> String {

        File::read(&Self::trains().join(name).join("about.md"))

    }

    pub fn title ( name: &str ) -> String {

        Text::first_line(&Self::about(name)).trim_start_matches('#').trim().to_string()

    }

    pub fn learn ( name: &str, slug: &str, content: &str ) -> AppResult<()> {

        let dir = Self::trains().join(name).join("history");
        Dir::ensure(&dir)?;

        let target = dir.join(format!("{}-{slug}.md", Dir::next_stamp(&dir)));

        File::write_atomic(&target, content)

    }

    fn ask ( worker: &mut Worker, prompt: String ) -> Option<String> {

        let answer = Env::temp_dir().join(format!("agentx-consult-{}.md", Proc::pid()));
        File::remove(&answer);

        let prompt = prompt.replace("{answer}", &Path::display(&answer));

        if worker.start(&prompt).is_err() {

            File::remove(&answer);
            return None;

        }

        let body = File::read(&answer);
        File::remove(&answer);

        Some(body)

    }

    fn parse_type ( body: &str ) -> Option<String> {

        let value = Self::parse_line(body, "type:")?;

        let name = match value.get(..4) {
            Some(head) if head.eq_ignore_ascii_case("new ") => value[4..].trim(),
            _ => value.as_str(),
        };

        let slug = Text::slug(name);

        if slug.is_empty() { None } else { Some(slug) }

    }

    fn parse_line ( body: &str, label: &str ) -> Option<String> {

        let value = body.lines().map(str::trim).find_map(|line| {

            let head = line.get(..label.len())?;

            head.eq_ignore_ascii_case(label).then(|| line[label.len()..].trim().to_string())

        })?;

        if value.is_empty() || value.eq_ignore_ascii_case("none") { None } else { Some(value) }

    }

    fn catalogue () -> String {

        let names = Self::available();

        if names.is_empty() { return "  (none yet)".to_string(); }

        let blocks: Vec<String> = names.iter().map(|name| {

            let about = Self::about(name);
            let body = if about.trim().is_empty() { name.clone() } else { about.trim().to_string() };

            format!("### {name}\n{body}")

        }).collect();

        blocks.join("\n\n")

    }

    fn create ( name: &str ) -> AppResult<()> {

        let dir = Self::trains().join(name);

        Dir::ensure(&dir.join("overview"))?;
        Dir::ensure(&dir.join("contracts"))?;
        Dir::ensure(&dir.join("skills"))?;
        Dir::ensure(&dir.join("requires"))?;
        Dir::ensure(&dir.join("history"))?;

        let about = dir.join("about.md");

        if !about.exists() {

            File::write(&about, &format!("# {name}\n\nStack and description for this archetype. Describe what it is and exactly what kinds of project it fits.\n"))?;

        }

        Ok(())

    }

    fn extract ( dir: &Embedded, base: &StdPath ) -> AppResult<()> {

        for entry in dir.entries() {

            match entry {
                DirEntry::Dir(sub) => Self::extract(sub, base)?,
                DirEntry::File(file) => {

                    let target = base.join(file.path());

                    if !target.exists() { File::write_bytes(&target, file.contents())?; }

                }
            }

        }

        Ok(())

    }

    fn resync ( dir: &Embedded, base: &StdPath ) -> AppResult<()> {

        for entry in dir.entries() {

            match entry {
                DirEntry::Dir(sub) => Self::resync(sub, base)?,
                DirEntry::File(file) => {

                    if Self::is_history(file.path()) { continue; }

                    File::write_bytes(&base.join(file.path()), file.contents())?;

                }
            }

        }

        Ok(())

    }

    fn is_history ( path: &StdPath ) -> bool {

        path.components().any(|part| part.as_os_str() == "history")

    }

    fn store () -> PathBuf {

        Env::home().unwrap_or_else(Env::temp_dir).join(".agentx")

    }

    fn trains () -> PathBuf {

        Self::store().join("train")

    }

}
