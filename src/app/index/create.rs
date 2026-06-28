use std::io::{self, IsTerminal};
use std::path::Path as StdPath;
use std::process::Command;

use crate::config::{Paths, Train};
use crate::config::base::consts::{CACHE_DIR, CONFIG_FILE, DOCS_DIR, RUN_LOG, TOOL};
use crate::core::error::{AppError, AppResult};
use crate::core::fs::{Dir, Path};
use crate::core::proc::Proc;
use crate::app::{App, Flags, Orchestrator, Project, Ui};

impl App {

    pub fn create ( base: &StdPath, path: &StdPath, flags: &Flags ) -> AppResult<()> {

        let target = if path.is_absolute() { path.to_path_buf() } else { base.join(path) };

        if Self::occupied(&target) {

            return Err(AppError::message(format!("{} exists and is not empty — `new` needs a fresh directory", target.display())));

        }

        let inspire = Self::require_inspire(flags)?;

        Dir::ensure(&target)?;

        if flags.background { return Self::spawn_create(&target, &inspire, flags); }

        let paths = Paths::new(&target);

        Self::guard_signals();

        Self::init(&target, &Flags { inspire: Some(inspire.as_str()), ..*flags })?;

        if Proc::aborted() {

            Ui::blank();
            Ui::warn("interrupted before creation started");
            Ui::blank();

            return Ok(());

        }

        let config = Project::assemble(&target)?;

        Self::ensure_agents(&config)?;

        Self::engage(&paths)?;

        let mut orchestrator = Orchestrator::new(config);

        Ui::loading("creating");

        let result = orchestrator.create();

        Ui::loaded();

        Self::disengage(&paths);

        result

    }

    fn occupied ( dir: &StdPath ) -> bool {

        if !dir.exists() { return false; }

        let allowed = [CACHE_DIR, CONFIG_FILE, DOCS_DIR];

        let mut entries = Dir::files(dir);
        entries.extend(Dir::subdirs(dir));

        entries.iter().any(|entry| !allowed.contains(&Path::name_of(entry).as_str()))

    }

    fn require_inspire ( flags: &Flags ) -> AppResult<String> {

        if let Some(value) = flags.inspire { return Self::select_inspire(value); }

        if io::stdin().is_terminal() && let Some(name) = Self::choose_inspire(false)? { return Ok(name); }

        Err(AppError::message("`new` needs an inspiration archetype — pass --inspire <name|N>, or pick one from the menu"))

    }

    fn spawn_create ( target: &StdPath, inspire: &str, flags: &Flags ) -> AppResult<()> {

        let paths = Paths::new(target);
        Train::init()?;
        Project::scaffold(&paths)?;

        let exe = std::env::current_exe().map_err(|error| AppError::message(format!("cannot locate the {TOOL} binary: {error}")))?;
        let log = paths.configs.join(RUN_LOG);

        let mut command = Command::new(exe);
        command.arg("new").arg(target).arg("--inspire").arg(inspire);

        if let Some(value) = flags.gate { command.arg("--gate").arg(value); }

        flags.forward(&mut command);

        command.current_dir(target);

        let pid = Proc::detach(command, &log)?;

        Ui::blank();
        Ui::ok(&format!("creating in the background — pid {pid}"));
        Ui::detail("logs", &Path::relative_one(&log, target));
        Ui::detail("control", &format!("{TOOL} status · {TOOL} stop"));
        Ui::blank();

        Ok(())

    }

}
