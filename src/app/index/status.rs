use std::io::{self, IsTerminal};
use std::path::Path as StdPath;

use crate::config::{Paths, Spec};
use crate::config::base::consts::{LOG_TAIL, PHASES, POLL_MS, POLL_TICKS, RUN_LOG, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File, Path};
use crate::core::proc::Proc;
use crate::core::term::Term;
use crate::app::{App, Journey, Orchestrator, Project, Status, Ui};

impl App {

    pub fn status ( dir: &StdPath ) -> AppResult<()> {

        Self::status_once(dir)?;

        Ok(())

    }

    pub fn watch ( dir: &StdPath ) -> AppResult<()> {

        if !Term::ansi() || !io::stdout().is_terminal() {

            Self::status_once(dir)?;
            Ui::info("no TTY — printed once (watch needs an interactive terminal)");

            return Ok(());

        }

        Self::guard_signals();
        Ui::screen(true);
        Ui::cursor(false);

        let result = Self::watch_loop(dir);

        Ui::cursor(true);
        Ui::screen(false);
        Ui::blank();

        result

    }

    fn watch_loop ( dir: &StdPath ) -> AppResult<()> {

        loop {

            Ui::home();

            Self::status_once(dir)?;

            Ui::dot(0, "watching · refreshes every second · Ctrl+C to exit");

            for _ in 0..POLL_TICKS {

                if Proc::aborted() { return Ok(()); }

                std::thread::sleep(std::time::Duration::from_millis(POLL_MS));

            }

        }

    }

    fn status_once ( dir: &StdPath ) -> AppResult<Status> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        let journey = Journey::load(&paths.state);

        let tool = Proc::read_pid(&paths.pid);
        let running = tool.is_some_and(Proc::is_alive);

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
        Ui::head("Engines  ·  per seat · model · effort");

        let ( model, effort ) = document.resolve_member(&document.agent.manager);
        Ui::field("manager", &format!("model {model}  ·  effort {effort}"));

        for phase in PHASES {

            if !document.option.active(phase) { continue; }

            for ( seat, member ) in document.agent.seats(phase) {

                let ( model, effort ) = document.resolve_member(&member);
                Ui::field(&format!("{phase} {seat}"), &format!("model {model}  ·  effort {effort}"));

            }

        }

        if journey.journey_id.is_empty() {

            Ui::blank();
            Ui::info(&format!("no journey yet — run `{TOOL} start`"));
            Self::recent(&log);
            Self::stats(&paths, &document.project, &journey);
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
            Self::recent(&log);
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

        Self::recent(&log);
        Self::stats(&paths, &document.project, &journey);

        Ui::blank();
        Ui::head("Now  ·  what's happening");

        let ( who, doing, stage ) = Self::activity(&journey, running);

        Ui::state(&who, running, &doing);
        Ui::blank();
        Ui::field("phase", &stage);

        Ui::blank();

        Ok(journey.status)

    }

    fn stats ( paths: &Paths, spec: &Spec, journey: &Journey ) {

        let context = Project::discover(paths, spec);

        let knowledge = context.overview.len() + context.contracts.len() + context.skills.len()
            + context.designs.len() + context.references.len() + context.history.len();

        let sources = context.requires.len();
        let backlog = Dir::markdown(&paths.inbox).len();
        let total = Dir::markdown(&paths.tasks).len();
        let shipped = journey.task_status.values().filter(|status| status.as_str() == "shipped").count();
        let blocked = journey.task_status.values().filter(|status| status.as_str() == "blocked").count();
        let pending = total.saturating_sub(shipped + blocked);
        let recorded = Dir::markdown(&paths.manager).len();

        Ui::head("Stats");
        Ui::field("knowledge", &knowledge.to_string());
        Ui::field("sources", &sources.to_string());
        Ui::field("backlog", &backlog.to_string());
        Ui::field("tasks", &total.to_string());
        Ui::field("shipped", &Self::ratio(shipped, total));
        Ui::field("blocked", &Self::ratio(blocked, total));
        Ui::field("pending", &Self::ratio(pending, total));
        Ui::field("recorded", &Self::ratio(recorded, backlog));

    }

    fn ratio ( part: usize, whole: usize ) -> String {

        match whole {
            0 => part.to_string(),
            _ => format!("{part}  ({}%)", part * 100 / whole),
        }

    }

    fn recent ( log: &StdPath ) {

        let lines = File::tail(log, LOG_TAIL);

        if lines.is_empty() { return; }

        Ui::blank();
        Ui::head("Recent  ·  live log");

        for line in &lines { Ui::log(line); }

    }

    fn activity ( journey: &Journey, running: bool ) -> ( String, String, String ) {

        if !running {

            return ( "—".to_string(), "not running".to_string(), journey.phase.slug().to_string() );

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

                let phase = journey.phase.slug();

                if journey.manager_review == "pending" {

                    return ( "manager".to_string(), "reviewing the round — judging the reports against the real code".to_string(), format!("{phase} · round {}", journey.current_round) );

                }
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

    pub(super) fn pid_line ( pid: Option<i32>, alive: bool ) -> String {

        match pid {
            Some(value) if alive => format!("{value}   (alive)"),
            Some(value) => format!("{value}   (stale)"),
            None => "—".to_string(),
        }

    }

}
