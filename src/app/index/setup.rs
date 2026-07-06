use std::path::Path as StdPath;

use crate::config::{Document, Paths, Spec, Train};
use crate::config::base::consts::{CACHE_DIR, CONFIG_FILE, DOCS_DIR, TOOL};

use crate::core::error::{AppError, AppResult};
use crate::core::fs::{Dir, File, Path};
use crate::core::text::Text;
use crate::app::{App, Flags, Project, Ui};

impl App {

    pub fn init ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        Ui::blank();
        Ui::title(&format!("{TOOL} · init"));
        Ui::blank();
        Ui::step("scaffolding the config, runtime, and docs");

        Self::prepare(dir, flags)

    }

    pub(super) fn init_stage ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        Ui::rule("init · scaffolding the config, runtime, and docs");

        Self::prepare(dir, flags)

    }

    fn prepare ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let paths = Paths::new(dir);
        Train::init()?;

        let inspire = Self::resolve_inspire(&paths, flags)?;

        let had_config = Path::exists(&paths.config_file);
        let had_cache = Path::exists(&paths.cache);
        let had_docs = Path::exists(&paths.docs);

        let copied = Self::copy_manifests(&paths, &inspire)?;
        Project::scaffold(&paths)?;

        let bound = Self::configure(&paths, flags)?;
        let bound = Self::offer_inspire(&paths, bound)?;
        let gate = Self::offer_gate(&paths)?;

        Ui::blank();
        Ui::ok(&format!("initialised -> {}", dir.display()));
        Ui::blank();

        if !bound.is_empty() { Ui::detail("inspire", &bound); }

        if !gate.is_empty() { Ui::detail("gate", &gate); }

        Ui::detail("config", &Self::state_note(CONFIG_FILE, had_config));
        Ui::detail("runtime", &Self::state_note(CACHE_DIR, had_cache));
        Ui::detail("docs", &Self::state_note(DOCS_DIR, had_docs));

        if copied > 0 { Ui::detail("manifests", &format!("{copied} file(s) copied from the project node")); }

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
        let dirty = Self::apply_flags(&mut document, flags)?;

        if dirty { document.save(&paths.config_file)?; }

        Ok(document.project.inspire)

    }

    fn offer_inspire ( paths: &Paths, bound: String ) -> AppResult<String> {

        if !bound.is_empty() || !Self::interactive() { return Ok(bound); }

        let Some(name) = Self::choose_inspire(true)? else { return Ok(bound) };

        let mut document = Spec::document(&paths.config_file)?;
        document.project.inspire = name.clone();
        document.save(&paths.config_file)?;

        Self::copy_manifests(paths, &name)?;

        Ok(name)

    }

    fn offer_gate ( paths: &Paths ) -> AppResult<String> {

        let mut document = Spec::document(&paths.config_file)?;
        let current = document.gate.command.trim().to_string();

        if !current.is_empty() || !Self::interactive() { return Ok(current); }

        Ui::blank();

        let Some(command) = Ui::ask("  set the quality gate — one read-only shell command", "(enter to skip · the manager verifies it at intake and composes one on a fix ruling)")? else { return Ok(current) };

        document.gate.command = command.clone();
        document.save(&paths.config_file)?;

        Ok(command)

    }

    pub(super) fn autofill ( paths: &Paths ) -> AppResult<()> {

        let document = Spec::document(&paths.config_file)?;

        Self::copy_manifests(paths, &document.project.inspire)?;

        Ok(())

    }

    fn copy_manifests ( paths: &Paths, inspire: &str ) -> AppResult<usize> {

        if inspire.is_empty() { return Ok(0); }

        let source = Train::manifests(inspire);

        if !source.is_dir() { return Ok(0); }

        let mut copied = 0;

        for file in Dir::walk(&source) {

            if !file.is_file() || Path::hidden_in(&file, &source) { continue; }

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

        let projects = Train::available();
        let histories = Train::history_kinds();
        let input = value.trim();

        if let Ok(number) = input.parse::<usize>() {

            if number >= 1 && number <= projects.len() { return Ok(projects[number - 1].clone()); }

            if projects.is_empty() { return Err(AppError::message(format!("--inspire {number} points at nothing - the training center has no project nodes yet; pass a name instead"))); }

            return Err(AppError::message(format!("--inspire {number} is out of range - choose 1 to {}", projects.len())));

        }

        let wanted = Text::slug(Text::unprefix(input));

        if let Some(name) = projects.iter().find(|item| Text::slug(item) == wanted) { return Ok(name.clone()); }

        if let Some(name) = histories.iter().find(|item| Text::slug(item) == wanted) { return Ok(name.clone()); }

        let mut known = String::new();

        if !projects.is_empty() {

            known.push_str("\n  project nodes:");

            for ( index, item ) in projects.iter().enumerate() { known.push_str(&format!("\n    {}) {item}", index + 1)); }

        }

        let history_only: Vec<&String> = histories.iter().filter(|item| !projects.iter().any(|node| node.eq_ignore_ascii_case(item))).collect();

        if !history_only.is_empty() {

            known.push_str("\n  history kinds:");

            for item in &history_only { known.push_str(&format!("\n    - {item}")); }

        }

        Err(AppError::message(format!("unknown inspiration '{value}' - known:{known}")))

    }

    pub(super) fn choose_inspire ( allow_auto: bool ) -> AppResult<Option<String>> {

        let types = Train::available();

        if types.is_empty() { return Ok(None); }

        let mut options = Vec::with_capacity(types.len() + 1);

        if allow_auto { options.push("auto  ·  let the manager detect it".to_string()); }

        for name in &types {

            let title = Train::title(name);
            options.push(if title.is_empty() { name.clone() } else { title });

        }

        Ui::blank();

        let keys = Ui::keys();
        let hint = if allow_auto { format!("{keys} · enter choose · q auto") } else { format!("{keys} · enter choose · required") };
        let picked = Ui::choose(&format!("  select the inspiration project node   ({hint})"), &options, 0)?;

        let base = usize::from(allow_auto);

        match picked {
            None => Ok(None),
            Some(0) if allow_auto => Ok(None),
            Some(index) => Ok(Some(types[index - base].clone())),
        }

    }

    pub fn inspire ( dir: &StdPath, name: Option<&str>, show: bool ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        if !Path::exists(&paths.config_file) {

            return Err(AppError::message(format!("no {CONFIG_FILE} here — run `{TOOL} init` first")));

        }

        Train::init()?;

        let mut document = Spec::document(&paths.config_file)?;
        let current = document.project.inspire.trim().to_string();

        if show {

            match current.is_empty() {
                true => {

                    Ui::blank();
                    Ui::point(0, &format!("no inspiration bound — the manager classifies it during intake, or bind one: `{TOOL} inspire <NAME|N>`"));
                    Ui::blank();

                }
                false => println!("{current}"),
            }

            return Ok(());

        }

        let picked = match name {
            Some(value) => Self::select_inspire(value)?,
            None if !Self::interactive() => return Err(AppError::message(format!("no interactive terminal for the menu — pass the node explicitly: `{TOOL} inspire <name|N>` (or -i)"))),
            None => match Self::choose_inspire(false)? {
                Some(node) => node,
                None => {

                    Ui::blank();
                    Ui::point(0, "cancelled — inspire unchanged");
                    Ui::blank();

                    return Ok(());

                }
            },
        };

        if picked == current {

            Ui::blank();
            Ui::point(0, &format!("inspire unchanged — already bound to {picked}"));
            Ui::blank();

            return Ok(());

        }

        document.project.inspire = picked.clone();
        document.save(&paths.config_file)?;

        Ui::blank();
        Ui::ok(&format!("inspire bound · {picked}"));

        let title = Train::title(&picked);

        if !title.is_empty() { Ui::detail("node", &title); }

        if !current.is_empty() { Ui::detail("was", &current); }

        Self::warn_training(&picked);
        Ui::blank();

        Ok(())

    }

    pub fn gate ( dir: &StdPath, command: Option<&str>, show: bool ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        if !Path::exists(&paths.config_file) {

            return Err(AppError::message(format!("no {CONFIG_FILE} here — run `{TOOL} init` first")));

        }

        let mut document = Spec::document(&paths.config_file)?;
        let current = document.gate.command.trim().to_string();

        if show {

            match current.is_empty() {
                true => {

                    Ui::blank();
                    Ui::point(0, &format!("no gate set — the manager composes one at intake on a fix ruling, or set it: `{TOOL} gate <COMMAND>`"));
                    Ui::blank();

                }
                false => println!("{current}"),
            }

            return Ok(());

        }

        let picked = match command {
            Some(value) if !value.trim().is_empty() => value.trim().to_string(),
            _ if !Self::interactive() => return Err(AppError::message(format!("no interactive terminal for the prompt — pass the command explicitly: `{TOOL} gate <COMMAND>` (or -g)"))),
            _ => {

                Ui::blank();

                match Ui::ask("  set the quality gate — one read-only shell command", "(enter to cancel)")? {
                    Some(value) => value,
                    None => {

                        Ui::blank();
                        Ui::point(0, "cancelled — gate unchanged");
                        Ui::blank();

                        return Ok(());

                    }
                }

            }
        };

        if picked == current {

            Ui::blank();
            Ui::point(0, "gate unchanged — already set to that command");
            Ui::blank();

            return Ok(());

        }

        document.gate.command = picked.clone();
        document.save(&paths.config_file)?;

        Ui::blank();
        Ui::ok(&format!("gate set · {picked}"));

        if !current.is_empty() { Ui::detail("was", &current); }

        Ui::blank();

        Ok(())

    }

}
