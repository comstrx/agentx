use std::io::{self, IsTerminal};
use std::path::Path as StdPath;

use crate::config::{Document, Paths, Spec, Train};
use crate::config::base::consts::{CACHE_DIR, CONFIG_FILE, DOCS_DIR};

use crate::core::error::{AppError, AppResult};
use crate::core::fs::{Dir, File, Path};
use crate::core::term::Term;
use crate::core::text::Text;
use crate::app::{App, Flags, Project, Ui};

impl App {

    pub fn init ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let paths = Paths::new(dir);
        Train::init()?;

        let inspire = Self::resolve_inspire(&paths, flags)?;

        let had_config = Path::exists(&paths.config_file);
        let had_cache = Path::exists(&paths.cache);
        let had_docs = Path::exists(&paths.docs);

        let copied = Self::copy_manifests(&paths, &inspire)?;
        Project::scaffold(&paths)?;

        let bound = Self::configure(&paths, flags)?;

        Ui::blank();
        Ui::ok(&format!("initialised  {}", dir.display()));

        if !bound.is_empty() { Ui::detail("inspire", &bound); }

        Ui::detail("config", &Self::state_note(CONFIG_FILE, had_config));
        Ui::detail("runtime", &Self::state_note(CACHE_DIR, had_cache));
        Ui::detail("docs", &Self::state_note(DOCS_DIR, had_docs));

        if copied > 0 { Ui::detail("manifests", &format!("{copied} file(s) copied from the archetype")); }

        Ui::blank();

        Ok(())

    }

    fn state_note ( name: &str, existed: bool ) -> String {

        match existed {
            true => format!("{name}  (already present)"),
            false => format!("{name}  (created)"),
        }

    }

    fn resolve_inspire ( paths: &Paths, flags: &Flags ) -> AppResult<String> {

        if let Some(value) = flags.inspire { return Self::select_inspire(value); }

        Ok(Spec::load(&paths.config_file).map(|spec| spec.inspire).unwrap_or_default())

    }

    fn configure ( paths: &Paths, flags: &Flags ) -> AppResult<String> {

        let mut document = Spec::document(&paths.config_file)?;
        let mut dirty = Self::apply_flags(&mut document, flags)?;

        dirty |= document.fill_defaults();

        if dirty { document.save(&paths.config_file)?; }

        Ok(document.project.inspire)

    }

    pub(super) fn autofill ( paths: &Paths ) -> AppResult<()> {

        let mut document = Spec::document(&paths.config_file)?;

        if document.project.inspire.is_empty() && io::stdin().is_terminal() && let Some(name) = Self::choose_inspire(true)? {

            document.project.inspire = name;
            document.save(&paths.config_file)?;

        }

        Self::copy_manifests(paths, &document.project.inspire)?;

        Ok(())

    }

    fn copy_manifests ( paths: &Paths, inspire: &str ) -> AppResult<usize> {

        if inspire.is_empty() { return Ok(0); }

        let source = Train::manifests(inspire);

        if !source.is_dir() { return Ok(0); }

        let mut copied = 0;

        for file in Dir::walk(&source) {

            if !file.is_file() { continue; }

            let Ok(rel) = file.strip_prefix(&source) else { continue; };

            let dest = paths.root.join(rel);

            if dest.exists() { continue; }

            if let Some(parent) = dest.parent() { Dir::ensure(parent)?; }

            File::copy(&file, &dest)?;

            copied += 1;

        }

        Ok(copied)

    }

    pub(super) fn apply_flags ( document: &mut Document, flags: &Flags ) -> AppResult<bool> {

        let mut dirty = false;

        if let Some(value) = flags.inspire {

            let name = Self::select_inspire(value)?;

            if name != document.project.inspire { document.project.inspire = name; dirty = true; }

        }

        if let Some(value) = flags.description {

            let text = value.trim();

            if text != document.project.description { document.project.description = text.to_string(); dirty = true; }

        }

        if let Some(value) = flags.gate {

            let command = value.trim();

            if !command.is_empty() && command != document.gate.command {

                document.gate.command = command.to_string();
                dirty = true;

            }

        }

        dirty |= Self::apply_bool(flags.lint, "--lint", &mut document.option.lint)?;
        dirty |= Self::apply_bool(flags.format, "--format", &mut document.option.format)?;
        dirty |= Self::apply_bool(flags.audits, "--audits", &mut document.option.audits)?;
        dirty |= Self::apply_bool(flags.tests, "--tests", &mut document.option.tests)?;
        dirty |= Self::apply_bool(flags.fuzzes, "--fuzzes", &mut document.option.fuzzes)?;
        dirty |= Self::apply_bool(flags.benches, "--benches", &mut document.option.benches)?;
        dirty |= Self::apply_bool(flags.examples, "--examples", &mut document.option.examples)?;
        dirty |= Self::apply_bool(flags.comments, "--comments", &mut document.option.comments)?;
        dirty |= Self::apply_bool(flags.doc_blocks, "--doc-blocks", &mut document.option.doc_blocks)?;
        dirty |= Self::apply_bool(flags.doc_contracts, "--doc-contracts", &mut document.option.doc_contracts)?;

        if flags.no_train && document.option.train { document.option.train = false; dirty = true; }

        if flags.no_clear && document.option.clear { document.option.clear = false; dirty = true; }

        Ok(dirty)

    }

    fn apply_bool ( value: Option<&str>, flag: &str, field: &mut bool ) -> AppResult<bool> {

        let Some(value) = value else { return Ok(false) };

        let on = Spec::parse_bool(value)
            .ok_or_else(|| AppError::message(format!("invalid {flag} value {value:?} (use true/false, 1/0, yes/no)")))?;

        if on == *field { return Ok(false); }

        *field = on;

        Ok(true)

    }

    pub(super) fn select_inspire ( value: &str ) -> AppResult<String> {

        let types = Train::available();
        let input = value.trim();

        if let Ok(number) = input.parse::<usize>() {

            if number >= 1 && number <= types.len() { return Ok(types[number - 1].clone()); }

            return Err(AppError::message(format!("--inspire {number} is out of range - choose 1 to {}", types.len())));

        }

        let slug = Text::slug(input);

        if types.iter().any(|item| item == &slug) { return Ok(slug); }

        let mut known = String::new();

        for ( index, item ) in types.iter().enumerate() {

            known.push_str(&format!("\n  {}) {item}", index + 1));

        }

        Err(AppError::message(format!("unknown inspiration '{value}' - known training-center archetypes:{known}")))

    }

    pub(super) fn choose_inspire ( allow_auto: bool ) -> AppResult<Option<String>> {

        let types = Train::available();

        if types.is_empty() { return Ok(None); }

        let mut options = Vec::with_capacity(types.len() + 1);

        if allow_auto { options.push("auto  ·  let the manager detect it".to_string()); }

        for name in &types {

            let title = Train::title(name);
            options.push(if title.is_empty() { name.clone() } else { format!("{name}  ·  {title}") });

        }

        Ui::blank();

        let hint = if allow_auto { "↑/↓ move · enter choose · q auto" } else { "↑/↓ move · enter choose · required" };
        let picked = Term::select(&format!("  select the inspiration archetype   ({hint})"), &options, 0)?;

        let base = usize::from(allow_auto);

        match picked {
            None => Ok(None),
            Some(0) if allow_auto => Ok(None),
            Some(index) => Ok(Some(types[index - base].clone())),
        }

    }

}
