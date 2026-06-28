use std::path::Path as StdPath;

use crate::config::{Paths, Train};
use crate::config::base::consts::{CACHE_DIR, CONFIG_FILE, TOOL};
use crate::core::error::AppResult;
use crate::core::fs::Path;
use crate::core::proc::Proc;
use crate::app::{App, Journey, Project, Ui};

impl App {

    pub fn info ( dir: &StdPath ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Train::init()?;

        let config = Project::assemble(&root)?;
        let spec = &config.spec;
        let journey = Journey::load(&paths.state);
        let sessions = Self::sessions_of(&paths.sessions);

        let config_note = match paths.config_file.exists() {
            true => Path::relative_one(&paths.config_file, &root),
            false => format!("{CONFIG_FILE}  (absent — run `{TOOL} init` here)"),
        };

        Ui::blank();
        Ui::title(&format!("{TOOL} · project snapshot"));

        Ui::blank();
        Ui::head("Project");
        Ui::field("root", &Path::display(&root));
        Ui::field("config", &config_note);
        Ui::field("cache", &Path::relative_one(&paths.cache, &root));

        let kind = match spec.inspire.is_empty() {
            true => format!("(unbound — run `{TOOL} init` to classify)"),
            false => spec.inspire.clone(),
        };

        Ui::field("inspire", &kind);

        let live = match Proc::read_pid(&paths.pid) {
            Some(pid) if Proc::is_alive(pid) => format!("running (pid {pid})"),
            _ => "idle".to_string(),
        };

        Ui::field("run state", &live);

        Ui::blank();
        Ui::head(&format!("Config  ·  [project]  ({CONFIG_FILE})"));
        Ui::pair("inspire", &format!("{:?}", spec.inspire));
        Ui::pair("description", &format!("{:?}", spec.description));

        if !spec.ignore.is_empty() { Ui::pair("ignore", &format!("{:?}", spec.ignore)); }

        if !spec.include.is_empty() { Ui::pair("include", &format!("{:?}", spec.include)); }

        let opt = &config.option;

        Ui::blank();
        Ui::head("[option]");
        Ui::pair("lint", &opt.lint.to_string());
        Ui::pair("format", &opt.format.to_string());
        Ui::pair("audits", &opt.audits.to_string());
        Ui::pair("tests", &opt.tests.to_string());
        Ui::pair("fuzzes", &opt.fuzzes.to_string());
        Ui::pair("benches", &opt.benches.to_string());
        Ui::pair("examples", &opt.examples.to_string());
        Ui::pair("comments", &opt.comments.to_string());
        Ui::pair("doc_blocks", &opt.doc_blocks.to_string());
        Ui::pair("doc_contracts", &opt.doc_contracts.to_string());
        Ui::pair("train", &opt.train.to_string());
        Ui::pair("clear", &opt.clear.to_string());

        Ui::blank();
        Ui::head("[gate]");
        Ui::pair("timeout", &config.gate.timeout.to_string());
        Ui::pair("command", &format!("{:?}", config.gate.command));

        Ui::blank();
        Ui::head("[agent]");
        Ui::pair("max_audits", &config.agent.max_audits.to_string());
        Ui::pair("max_rounds", &config.agent.max_rounds.to_string());
        Ui::pair("max_fixes", &config.agent.max_fixes.to_string());
        Ui::pair("timeout", &config.agent.timeout.to_string());
        Ui::pair("manager", &format!("{:?}", config.agent.manager));
        Ui::pair("requires", &format!("{:?}", config.agent.requires));
        Ui::pair("tasks", &format!("{:?}", config.agent.tasks));
        Ui::pair("audits", &format!("{:?}", config.agent.audits));
        Ui::pair("tests", &format!("{:?}", config.agent.tests));
        Ui::pair("benches", &format!("{:?}", config.agent.benches));
        Ui::pair("examples", &format!("{:?}", config.agent.examples));
        Ui::pair("fuzzes", &format!("{:?}", config.agent.fuzzes));

        Ui::blank();
        Ui::head("Rosters (expanded)");
        Ui::field("manager", config.manager());
        Ui::field("requires", &config.roster("requires").join(" "));
        Ui::field("tasks", &config.roster("tasks").join(" "));
        Ui::field("audits", &config.roster("audits").join(" "));
        Ui::field("tests", &config.roster("tests").join(" "));
        Ui::field("benches", &config.roster("benches").join(" "));
        Ui::field("examples", &config.roster("examples").join(" "));
        Ui::field("fuzzes", &config.roster("fuzzes").join(" "));

        Ui::blank();
        Ui::head("Engines (model · effort · empty field → strong default)");
        Ui::field("claude", &Self::engine_line(config.engine("claude")));
        Ui::field("codex", &Self::engine_line(config.engine("codex")));

        Ui::blank();
        Ui::head(&format!("Paths ({CACHE_DIR} runtime)"));
        Ui::field("state", &Path::relative_one(&paths.state, &root));
        Ui::field("sessions", &Path::relative_one(&paths.sessions, &root));
        Ui::field("pid", &Path::relative_one(&paths.pid, &root));
        Ui::field("active", &Path::relative_one(&paths.active, &root));
        Ui::field("inbox", &Path::relative_one(&paths.inbox, &root));
        Ui::field("tasks", &Path::relative_one(&paths.tasks, &root));
        Ui::field("reports", &Path::relative_one(&paths.reports, &root));
        Ui::field("rounds", &Path::relative_one(&paths.rounds, &root));
        Ui::field("gate_log", &Path::relative_one(&paths.gate_log, &root));

        Ui::blank();
        Ui::head("Classification (briefing files injected per bucket)");

        Self::classification(&config, &root);

        Ui::blank();
        Ui::head("Journey (state.json)");

        if journey.journey_id.is_empty() {

            Ui::info(&format!("no journey yet — run `{TOOL} start`"));

        }
        else {

            Ui::field("journey_id", &journey.journey_id);
            Ui::field("primed", &journey.primed.to_string());
            Ui::field("intake_done", &journey.intake_done.to_string());
            Ui::field("phase", &format!("{:?}", journey.phase));
            Ui::field("status", &format!("{:?}", journey.status));
            Ui::field("current_task", &journey.current_task);
            Ui::field("current_agent", &journey.current_agent);
            Ui::field("current_round", &journey.current_round.to_string());
            Ui::field("manager_review", &journey.manager_review);
            if journey.task_status.is_empty() {

                Ui::field("task_status", "(none)");

            }
            else {

                Ui::field("task_status", &format!("{} task(s)", journey.task_status.len()));

                for ( name, status ) in &journey.task_status {

                    Ui::task(name, status);

                }

            }
            Ui::field("agents_done", &journey.agents_done.join(", "));
            Ui::field("agents_pending", &journey.agents_pending.join(", "));
            Ui::field("blocked", &journey.blocked.join(", "));
            Ui::field("last_action", &journey.last_action);
            Ui::field("started_at", &journey.started_at);
            Ui::field("updated_at", &journey.updated_at);

        }

        Ui::blank();
        Ui::head("Sessions (sessions.json)");

        if sessions.is_empty() {

            Ui::info("none");

        }
        else {

            for ( key, id ) in &sessions {

                Ui::field(key, id);

            }

        }

        Ui::blank();

        Ok(())

    }

    fn engine_line ( engine: ( String, String ) ) -> String {

        format!("model {}  ·  effort {}", engine.0, engine.1)

    }

}
