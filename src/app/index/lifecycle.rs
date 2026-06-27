use std::path::{Path as StdPath, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;

use crate::config::{Paths, Spec, Train};
use crate::config::base::consts::{CACHE_DIR, RUN_LOG, TOOL};
use crate::core::error::{AppError, AppResult};
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

        Self::init(&root, flags)?;
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

        let config = Project::assemble(&root)?;

        Self::ensure_agents(&config)?;

        Self::engage(&paths)?;

        let mut orchestrator = Orchestrator::new(config);

        Ui::loading("orchestrating");

        let result = orchestrator.run();

        let completed = result.is_ok() && orchestrator.journey.status == Status::Completed;
        let clean = completed && orchestrator.journey.blocked.is_empty();
        let blocked = orchestrator.journey.blocked.join(", ");

        let do_train = clean && orchestrator.cfg.option.train;
        let do_clear = clean && orchestrator.cfg.option.clear;

        let trained = if do_train { orchestrator.run_train(true) } else { Ok(()) };

        Ui::loaded();

        Self::disengage(&paths);
        File::remove(&paths.drain);

        if clean {

            trained?;

            if do_clear { Project::clear(&paths); }

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

        let exe = std::env::current_exe().map_err(|error| AppError::message(format!("cannot locate the {TOOL} binary: {error}")))?;
        let log = paths.configs.join(RUN_LOG);

        let mut command = Command::new(exe);
        command.arg("start").arg("--dir").arg(&root);

        if let Some(name) = flags.inspire { command.arg("--inspire").arg(name); }

        if let Some(value) = flags.gate { command.arg("--gate").arg(value); }

        flags.forward(&mut command);

        Self::forward_paths(&mut command, dir, "--ignore", flags.ignore);
        Self::forward_paths(&mut command, dir, "--include", flags.include);

        command.current_dir(&root);

        let pid = Proc::detach(command, &log)?;

        Ui::blank();
        Ui::ok(&format!("started in the background — pid {pid}"));
        Ui::detail("logs", &Path::relative_one(&log, &root));
        Ui::detail("control", &format!("{TOOL} status · {TOOL} drain · {TOOL} stop"));
        Ui::blank();

        Ok(())

    }

    pub fn stop ( dir: &StdPath ) -> AppResult<()> {

        let paths = Paths::new(&Project::resolve_root(dir));

        Ui::blank();

        if !Self::is_running(&paths) {

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

        File::remove(&paths.active);
        File::remove(&paths.pid);
        Self::mark(paths, Status::Stopped);

    }

    fn signal ( paths: &Paths, sig: Signal ) {

        for pid_file in [&paths.active, &paths.pid] {

            if let Some(pid) = Proc::read_pid(pid_file) && Proc::is_alive(pid) {

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
