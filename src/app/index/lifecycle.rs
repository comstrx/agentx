use std::path::{Path as StdPath, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;

use crate::config::{Paths, Spec, Train};
use crate::config::base::consts::{BUSY_LABEL, CACHE_DIR, DOCS_DIR, RUN_LOG, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{File, Path};
use crate::core::proc::Proc;
use crate::app::{App, Flags, Journey, Orchestrator, Project, Status, Ui};

impl App {

    pub fn start ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        if flags.background { return Self::spawn_background(dir, flags); }

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Self::ensure_idle(&paths)?;

        File::remove(&paths.drain);

        Self::guard_signals();

        Self::init_stage(&root, flags)?;
        Self::autofill(&paths)?;

        if !flags.ignore.is_empty() || !flags.include.is_empty() {

            let mut spec = Spec::load(&paths.config_file)?;
            let mut dirty = Self::merge_into(&mut spec, &root, dir, flags.ignore, false);
            dirty |= Self::merge_into(&mut spec, &root, dir, flags.include, true);

            if dirty { spec.save(&paths.config_file)?; }

        }

        if Proc::aborted() {

            Ui::blank();
            Ui::warn("interrupted before the run started");
            Ui::blank();

            return Ok(());

        }

        let mut config = Project::assemble(&root)?;
        config.force = flags.force || !Self::interactive();

        Self::warn_unmatched(&config.paths.docs, &root);
        Self::warn_training(&config.spec.inspire);

        Self::ensure_agents(&config)?;

        Self::claim(&paths)?;

        let mut orchestrator = Orchestrator::new(config);

        Ui::loading(BUSY_LABEL);

        let result = orchestrator.run();

        let completed = result.is_ok() && orchestrator.journey.status == Status::Completed;
        let clean = completed && orchestrator.journey.blocked.is_empty();
        let blocked = orchestrator.journey.blocked.join(", ");

        let unbound = orchestrator.cfg.spec.inspire.trim().is_empty();
        let do_train = clean && orchestrator.cfg.option.train;
        let do_clear = clean && orchestrator.cfg.option.clear && !( do_train && unbound );

        let trained = if do_train { orchestrator.run_train(true) } else { Ok(()) };

        Ui::loaded();

        Self::disengage(&paths);
        File::remove(&paths.drain);

        if clean {

            trained?;

            if do_clear { Project::clear(&paths); }

            Ui::blank();

            if do_train && unbound {

                Ui::warn(&format!("run NOT recorded — no inspiration is bound; {CACHE_DIR} kept so the reports survive"));
                Ui::detail("record", &format!("`{TOOL} inspire <name|N>` to bind, then `{TOOL} train` · `{TOOL} clear` when done"));
                Ui::blank();

                return result;

            }

            match ( do_train, do_clear ) {
                ( true, true )  => Ui::ok(&format!("trained & cleared — recorded to the training center, {CACHE_DIR} reset to a clean slate (layout kept)")),
                ( true, false ) => Ui::ok(&format!("trained — recorded to the training center; {CACHE_DIR} kept (auto-clear is off — run `{TOOL} clear` when ready)")),
                ( false, true ) => Ui::ok(&format!("cleared — {CACHE_DIR} reset; the run was NOT recorded (auto-train is off)")),
                ( false, false ) => {

                    Ui::ok(&format!("journey complete — {CACHE_DIR} kept and NOT recorded (auto-train and auto-clear are both off)"));
                    Ui::detail("manual", &format!("`{TOOL} train` to record · `{TOOL} clear` to reset"));

                }
            }

            Ui::blank();

        }
        else if completed {

            Ui::warn(&format!("runtime kept for inspection — unresolved: {blocked}"));
            Ui::detail("review", &format!("{} · reports/ · rounds/ — run `{TOOL} train` then `{TOOL} clear` when ready", Path::relative_one(&paths.state, &root)));
            Ui::blank();

        }

        result

    }

    pub fn restart ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        Self::clear(dir)?;

        Self::start(dir, flags)

    }

    fn spawn_background ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);
        Train::init()?;
        Project::scaffold(&paths)?;

        Self::ensure_idle(&paths)?;

        let log = paths.configs.join(RUN_LOG);

        let mut command = Command::new(Self::binary()?);
        command.arg("start").arg("--dir").arg(&root);

        if let Some(name) = flags.inspire { command.arg("--inspire").arg(name); }

        if let Some(value) = flags.gate { command.arg("--gate").arg(value); }

        flags.forward(&mut command);

        Self::forward_paths(&mut command, dir, "--ignore", flags.ignore);
        Self::forward_paths(&mut command, dir, "--include", flags.include);

        command.current_dir(&root);

        Self::launch(command, &log, &root, "started in the background", &format!("{TOOL} status · {TOOL} drain · {TOOL} stop"))

    }

    pub fn stop ( dir: &StdPath ) -> AppResult<()> {

        let paths = Paths::new(&Project::resolve_root(dir));

        Ui::blank();

        if !Self::is_running(&paths) {

            Self::sweep_workers(&paths);
            File::remove(&paths.active);
            File::remove(&paths.pid);
            Ui::info("nothing is running — no cycle to stop");
            Ui::blank();

            return Ok(());

        }

        let position = Self::position(&paths);

        Self::terminate(&paths);

        Ui::ok(&format!("stopped the running cycle{position} — every agent killed; `start` resumes from the saved cursor"));
        Ui::blank();

        Ok(())

    }

    fn terminate ( paths: &Paths ) {

        Self::signal(paths, Signal::SIGTERM);

        if !Self::await_exit(&paths.pid, Duration::from_secs(8)) {

            Self::signal(paths, Signal::SIGKILL);
            let _ = Self::await_exit(&paths.pid, Duration::from_secs(2));

        }

        Self::sweep_workers(paths);
        File::remove(&paths.active);
        File::remove(&paths.pid);
        Self::mark(paths, Status::Stopped);

    }

    fn signal ( paths: &Paths, sig: Signal ) {

        for pid_file in [&paths.active, &paths.pid] {

            if let Some(pid) = Proc::read_pid(pid_file) && Proc::is_alive(pid) && Proc::leads(pid) {

                let _ = killpg(Pid::from_raw(pid), sig);

            }

        }

    }

    fn await_exit ( pid_file: &StdPath, grace: Duration ) -> bool {

        let deadline = Instant::now() + grace;

        while Instant::now() < deadline {

            match Proc::read_pid(pid_file) {
                Some(pid) if Proc::is_alive(pid) => std::thread::sleep(Duration::from_millis(100)),
                _ => return true,
            }

        }

        Proc::read_pid(pid_file).is_none_or(|pid| !Proc::is_alive(pid))

    }

    pub fn drain ( dir: &StdPath ) -> AppResult<()> {

        let paths = Paths::new(&Project::resolve_root(dir));

        Ui::blank();

        if !Self::is_running(&paths) {

            Ui::info("nothing is running — no cycle to drain");
            Ui::blank();

            return Ok(());

        }

        File::write(&paths.drain, "true\n")?;
        Self::mark(&paths, Status::Draining);
        Ui::ok(&format!("drain requested — the run stops cleanly after the current turn{}", Self::position(&paths)));
        Ui::blank();

        Ok(())

    }

    pub fn clear ( dir: &StdPath ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Ui::blank();

        if !paths.cache.exists() {

            Ui::info("nothing to clear");
            Ui::blank();
            return Ok(());

        }

        if Self::is_running(&paths) {

            Ui::step("a run is active — stopping it first");
            Self::terminate(&paths);

        }
        else {

            Self::sweep_workers(&paths);

        }

        Project::clear(&paths);
        Ui::ok(&format!("cleared {} — kept the directory layout", Path::relative_one(&paths.cache, &root)));
        Ui::blank();

        Ok(())

    }

    fn is_running ( paths: &Paths ) -> bool {

        Proc::read_pid(&paths.pid).is_some_and(Proc::is_alive)

    }

    fn mark ( paths: &Paths, status: Status ) {

        if !paths.state.exists() { return; }

        let mut journey = Journey::load(&paths.state);

        if journey.journey_id.is_empty() { return; }

        journey.status = status;
        let _ = journey.save(&paths.state);

    }

    fn position ( paths: &Paths ) -> String {

        let journey = Journey::load(&paths.state);

        if journey.journey_id.is_empty() { return String::new(); }

        format!(" (phase {:?}, round {})", journey.phase, journey.current_round)

    }

    pub(super) fn warn_unmatched ( docs: &StdPath, root: &StdPath ) {

        let stray = Project::unmatched(docs);

        if stray.is_empty() { return; }

        Ui::blank();
        Ui::warn(&format!("{} item(s) in {DOCS_DIR}/ are NOT recognized and will be IGNORED — fix the name:", stray.len()));

        for path in &stray { Ui::dot(1, &Path::relative_one(path, root)); }

    }

    pub(super) fn warn_training ( inspire: &str ) {

        let kind = inspire.trim();

        if kind.is_empty() { return; }

        let ( chain, missing ) = Train::trace(kind);

        if chain.is_empty() && Train::history_reports(kind).is_empty() {

            Ui::blank();
            Ui::warn(&format!("inspire '{kind}' resolves to no project node and no history — running with the project's own files only"));

        }

        if !missing.is_empty() {

            Ui::blank();
            Ui::warn(&format!("{} dependency node(s) declared but missing — that knowledge will NOT be injected:", missing.len()));

            for node in &missing { Ui::dot(1, node); }

        }

    }

    fn forward_paths ( command: &mut Command, base: &StdPath, flag: &str, paths: &[PathBuf] ) {

        if paths.is_empty() { return; }

        command.arg(flag);

        for path in paths {

            let abs = match path.is_absolute() {
                true => path.clone(),
                false => base.join(path),
            };

            command.arg(abs);

        }

    }

}
