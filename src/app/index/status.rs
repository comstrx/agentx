use std::io::{self, IsTerminal};
use std::path::Path as StdPath;

use crate::config::{Paths, Spec};
use crate::config::base::consts::{RUN_LOG, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, Path};
use crate::core::proc::Proc;
use crate::app::{App, Journey, Phase, Project, Status, Ui};

impl App {

    pub fn status ( dir: &StdPath, tail: bool ) -> AppResult<()> {

        if !tail || !io::stdout().is_terminal() {

            Self::status_once(dir)?;

            return Ok(());

        }

        Self::guard_signals();
        Ui::cursor(false);

        let result = Self::watch(dir);

        Ui::cursor(true);

        result

    }

    fn watch ( dir: &StdPath ) -> AppResult<()> {

        loop {

            Ui::home();

            let status = Self::status_once(dir)?;

            Ui::dot(0, "live · refreshing every second · Ctrl+C to stop");

            if Proc::aborted() || matches!(status, Status::Completed | Status::Failed) { return Ok(()); }

            for _ in 0..10 {

                if Proc::aborted() { return Ok(()); }

                std::thread::sleep(std::time::Duration::from_millis(100));

            }

        }

    }

    fn status_once ( dir: &StdPath ) -> AppResult<Status> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        let journey = Journey::load(&paths.state);
        let sessions = Self::sessions_of(&paths.sessions);

        let tool = Proc::read_pid(&paths.pid);
        let active = Proc::read_pid(&paths.active);
        let running = tool.is_some_and(Proc::is_alive);
        let worker_live = active.is_some_and(Proc::is_alive);

        Ui::blank();
        Ui::title(&format!("{TOOL} · status"));
        Ui::blank();

        let state = match ( running, tool ) {
            ( true, Some(pid) ) => format!("running   ·   pid {pid}"),
            _ => "idle".to_string(),
        };

        Ui::state("state", running, &state);

        let log = paths.configs.join(RUN_LOG);

        if running && Path::exists(&log) { Ui::field("logs", &Path::relative_one(&log, &root)); }

        let ( claude, codex ) = Spec::document(&paths.config_file)?.engines();

        Ui::blank();
        Ui::head("Engines  ·  model · effort in use");
        Ui::field("claude", &format!("model {}  ·  effort {}", claude.0, claude.1));
        Ui::field("codex", &format!("model {}  ·  effort {}", codex.0, codex.1));

        if journey.journey_id.is_empty() {

            Ui::blank();
            Ui::info(&format!("no journey yet — run `{TOOL} start`"));
            Ui::blank();

            return Ok(journey.status);

        }

        if journey.mode == "create" {

            Ui::blank();
            Ui::head(&format!("Creation  ·  {}", journey.journey_id));
            Ui::field("phase", "creation");
            Ui::field("status", &format!("{:?}", journey.status));

            if !journey.note.is_empty() {

                let label = if matches!(journey.status, Status::Failed) { "error" } else { "result" };
                Ui::field(label, &journey.note);

            }

            Ui::field("started", &journey.started_at);
            Ui::field("updated", &journey.updated_at);
            Ui::blank();

            return Ok(journey.status);

        }

        let total = Dir::markdown(&paths.tasks).len();
        let shipped = journey.task_status.values().filter(|value| value.as_str() == "shipped").count();

        Ui::blank();
        Ui::head(&format!("Journey  ·  {}", journey.journey_id));
        Ui::field("phase", &format!("{:?}", journey.phase));
        Ui::field("status", &format!("{:?}", journey.status));

        let current = match journey.current_task.is_empty() {
            true => format!("round {}", journey.current_round),
            false => format!("{} · round {} · {}", journey.current_task, journey.current_round, journey.current_agent),
        };

        Ui::field("current", &current);
        Ui::field("blocked", &if journey.blocked.is_empty() { "none".to_string() } else { journey.blocked.join(", ") });
        Ui::field("primed", &format!("{}   ·   intake {}", journey.primed, journey.intake_done));
        Ui::field("started", &journey.started_at);
        Ui::field("updated", &journey.updated_at);

        if total > 0 { Ui::field("tasks", &format!("{shipped}/{total} shipped   {}", Ui::bar(shipped, total))); }

        Ui::blank();
        Ui::head("Workers  ·  sessions");

        if sessions.is_empty() {

            Ui::info("none yet");

        }
        else {

            let active_key = Self::active_key(&journey);

            for ( key, id ) in &sessions {

                let short = id.get(..8).map(|head| format!("{head}…")).unwrap_or_else(|| id.clone());
                Ui::worker(key, &short, worker_live && key == &active_key);

            }

        }

        Ui::blank();
        Ui::head("Pids");
        Ui::field(TOOL, &Self::pid_line(tool, running));
        Ui::field("active", &Self::pid_line(active, worker_live));

        Ui::blank();
        Ui::head("Sessions");

        if sessions.is_empty() {

            Ui::info("none yet");

        }
        else {

            for ( key, id ) in &sessions {

                Ui::field(key, id);

            }

        }

        Ui::blank();

        Ok(journey.status)

    }

    fn active_key ( journey: &Journey ) -> String {

        if journey.current_agent.is_empty() { return "manager".to_string(); }

        let phase = match journey.phase {
            Phase::Requires => "requires",
            Phase::Tasks    => "tasks",
            Phase::Audit    => "audits",
            Phase::Tests    => "tests",
            Phase::Benches  => "benches",
            Phase::Examples => "examples",
            Phase::Fuzzes   => "fuzzes",
            _ => return "manager".to_string(),
        };

        format!("{phase}-{}", journey.current_agent)

    }

    fn pid_line ( pid: Option<i32>, alive: bool ) -> String {

        match pid {
            Some(value) if alive => format!("{value}   (alive)"),
            Some(value) => format!("{value}   (stale)"),
            None => "—".to_string(),
        }

    }

}
