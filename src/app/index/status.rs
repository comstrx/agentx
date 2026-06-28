use std::io::{self, IsTerminal};
use std::path::Path as StdPath;

use crate::config::{Paths, Spec};
use crate::config::base::consts::{RUN_LOG, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, Path};
use crate::core::proc::Proc;
use crate::app::{App, Journey, Orchestrator, Phase, Project, Status, Ui};

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

        let document = Spec::document(&paths.config_file)?;

        Ui::blank();
        Ui::head("Engines  ·  model · effort in use");

        for name in document.agent.backends() {

            let ( model, effort ) = document.engine_of(name);
            Ui::field(name, &format!("model {model}  ·  effort {effort}"));

        }

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
        Ui::head("Workers");

        if sessions.is_empty() {

            Ui::info("none yet");

        }
        else {

            for ( key, id ) in &sessions {

                let short = id.get(..8).map(|head| format!("{head}…")).unwrap_or_else(|| id.clone());
                Ui::field(key, &short);

            }

        }

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
        Ui::head("Pids");
        Ui::field(TOOL, &Self::pid_line(tool, running));
        Ui::field("active", &Self::pid_line(active, worker_live));

        Ui::blank();
        Ui::head("Now  ·  what's happening");

        let ( who, doing, stage ) = Self::activity(&journey, running);

        Ui::state(&who, running, &doing);
        Ui::field("phase", &stage);

        Ui::blank();

        Ok(journey.status)

    }

    fn phase_slug ( phase: Phase ) -> &'static str {

        match phase {
            Phase::Requires => "requires",
            Phase::Tasks    => "tasks",
            Phase::Audit    => "audits",
            Phase::Tests    => "tests",
            Phase::Benches  => "benches",
            Phase::Examples => "examples",
            Phase::Fuzzes   => "fuzzes",
            _               => "idle",
        }

    }

    fn activity ( journey: &Journey, running: bool ) -> ( String, String, String ) {

        if !running {

            return ( "—".to_string(), "not running".to_string(), Self::phase_slug(journey.phase).to_string() );

        }

        match journey.status {
            Status::Completed => ( "—".to_string(), "journey complete".to_string(), "completed".to_string() ),
            Status::Failed    => ( "—".to_string(), if journey.note.is_empty() { "journey failed".to_string() } else { journey.note.clone() }, "failed".to_string() ),
            Status::Stopped   => ( "—".to_string(), "stopped".to_string(), "stopped".to_string() ),
            Status::Drained   => ( "—".to_string(), "drained".to_string(), "drained".to_string() ),
            _ => {

                if !journey.primed {

                    let who = if journey.current_agent.is_empty() { "the team".to_string() } else { journey.current_agent.clone() };

                    return ( who, "being primed — training on the project and contracts".to_string(), "priming".to_string() );

                }

                if !journey.intake_done {

                    return ( "manager".to_string(), "ordering the discovered requirements into a backlog".to_string(), "intake".to_string() );

                }

                let phase = Self::phase_slug(journey.phase);
                let who = if journey.current_agent.is_empty() { "manager".to_string() } else { journey.current_agent.clone() };
                let verb = Orchestrator::verb_of(phase);

                let doing = match journey.current_task.is_empty() {
                    true  => verb.to_string(),
                    false => format!("{verb} · {}", journey.current_task),
                };

                let stage = format!("{phase} · round {}", journey.current_round);

                ( who, doing, stage )

            }
        }

    }

    fn pid_line ( pid: Option<i32>, alive: bool ) -> String {

        match pid {
            Some(value) if alive => format!("{value}   (alive)"),
            Some(value) => format!("{value}   (stale)"),
            None => "—".to_string(),
        }

    }

}
