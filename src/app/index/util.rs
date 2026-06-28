use std::path::{Path as StdPath, PathBuf};
use std::process::Command;
use nix::sys::signal::{SigSet, SigmaskHow, Signal, pthread_sigmask};
use nix::unistd::{Pid, setpgid};

use crate::config::Paths;
use crate::config::base::consts::TOOL;
use crate::core::error::{AppError, AppResult};
use crate::core::fs::{File, Path};
use crate::core::proc::Proc;
use crate::app::{App, Orchestrator, Ui};

impl App {

    pub(super) fn ensure_idle ( paths: &Paths ) -> AppResult<()> {

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is already active (pid {pid}); stop or drain it first")));

        }

        Ok(())

    }

    pub(super) fn engage ( paths: &Paths ) -> AppResult<()> {

        let _ = setpgid(Pid::from_raw(0), Pid::from_raw(0));

        Proc::write_pid(&paths.pid)

    }

    pub(super) fn disengage ( paths: &Paths ) {

        File::remove(&paths.pid);
        File::remove(&paths.active);

    }

    pub(super) fn binary () -> AppResult<PathBuf> {

        std::env::current_exe().map_err(|error| AppError::message(format!("cannot locate the {TOOL} binary: {error}")))

    }

    pub(super) fn launch ( command: Command, log: &StdPath, base: &StdPath, verb: &str, control: &str ) -> AppResult<()> {

        let pid = Proc::detach(command, log)?;

        Ui::blank();
        Ui::ok(&format!("{verb} — pid {pid}"));
        Ui::detail("logs", &Path::relative_one(log, base));
        Ui::detail("control", control);
        Ui::blank();

        Ok(())

    }

    pub(super) fn sessions_of ( path: &StdPath ) -> Vec<( String, String )> {

        let mut pairs: Vec<( String, String )> = Orchestrator::load_sessions(path).into_iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        pairs

    }

    pub(super) fn guard_signals () {

        let mut set = SigSet::empty();
        set.add(Signal::SIGINT);
        set.add(Signal::SIGTERM);

        if pthread_sigmask(SigmaskHow::SIG_BLOCK, Some(&set), None).is_err() { return; }

        std::thread::spawn(move || {

            loop {

                match set.wait() {
                    Ok(_) => Proc::request_abort(),
                    Err(_) => return,
                }

            }

        });

    }

}
