use std::collections::{BTreeMap, HashMap};
use std::io::{self, IsTerminal};
use std::path::Path as StdPath;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::{Pid, setpgid};

use crate::config::{Paths, Spec, Train};
use crate::config::consts::CONTEXT_BUCKETS;
use crate::core::error::{AppError, AppResult};
use crate::core::support::env::Env;
use crate::core::support::fs::{File, Path};
use crate::core::support::parse::Json;
use crate::core::support::proc::Proc;
use crate::core::support::term::Term;
use crate::core::support::text::Text;
use super::arch::{App, Journey, Orchestrator, Project, Status, Ui};

impl App {

    pub fn init ( dir: &StdPath, project: Option<&str>, gate: Option<&str> ) -> AppResult<()> {

        let paths = Paths::new(dir);
        Train::init()?;

        let bound = Self::configure(&paths, project, gate)?;

        Project::scaffold(&paths)?;

        Ui::blank();
        Ui::ok(&format!("initialised  {}", dir.display()));

        if !bound.is_empty() {

            Ui::detail("training", &format!("{bound}  ·  {}", Train::title(&bound)));

        }

        Ui::blank();

        Ok(())

    }

    pub fn start ( dir: &StdPath, project: Option<&str>, gate: Option<&str> ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);
        Train::init()?;
        Project::scaffold(&paths)?;

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is already active (pid {pid}); stop or drain it first")));

        }

        Self::prepare(&paths, project, gate)?;

        let config = Project::assemble(&root)?;

        let _ = setpgid(Pid::from_raw(0), Pid::from_raw(0));
        Proc::write_pid(&paths.pid)?;

        let mut orchestrator = Orchestrator::new(&config);
        let result = orchestrator.run();

        File::remove(&paths.pid);
        File::remove(&paths.active);
        File::remove(&paths.drain);

        if result.is_ok() && orchestrator.journey.status == Status::Completed {

            Project::clean(&paths);
            Ui::ok("runtime cleaned — .agentx reset to a clean slate (layout kept)");
            Ui::blank();

        }

        result

    }

    pub fn stop ( dir: &StdPath ) -> AppResult<()> {

        let paths = Paths::new(&Project::resolve_root(dir));

        Ui::blank();

        if !paths.cache.exists() {

            Ui::info("no run to stop");
            Ui::blank();
            return Ok(());

        }

        let mut hit = false;

        for pid_file in [&paths.active, &paths.pid] {

            if let Some(pid) = Proc::read_pid(pid_file) && Proc::is_alive(pid) {

                let _ = killpg(Pid::from_raw(pid), Signal::SIGTERM);
                hit = true;

            }

        }

        File::remove(&paths.active);
        File::remove(&paths.pid);
        Self::mark(&paths, Status::Stopped);

        if hit {

            Ui::ok(&format!("stopped the running cycle{} — `start` resumes from the saved cursor", Self::position(&paths)));

        }
        else {

            Ui::info("no running cycle found");

        }

        Ui::blank();

        Ok(())

    }

    pub fn drain ( dir: &StdPath ) -> AppResult<()> {

        let paths = Paths::new(&Project::resolve_root(dir));

        Ui::blank();

        if !paths.cache.exists() {

            Ui::info("no run to drain");
            Ui::blank();
            return Ok(());

        }

        File::write(&paths.drain, "true\n")?;
        Self::mark(&paths, Status::Draining);
        Ui::ok(&format!("drain requested — the run stops cleanly after the current turn{}", Self::position(&paths)));
        Ui::blank();

        Ok(())

    }

    pub fn clean ( dir: &StdPath ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Ui::blank();

        if !paths.cache.exists() {

            Ui::info("nothing to clean");
            Ui::blank();
            return Ok(());

        }

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is active (pid {pid}); stop or drain it before cleaning")));

        }

        Project::clean(&paths);
        Ui::ok(&format!("cleared {} — kept the directory layout", Path::relative_one(&paths.cache, &root)));
        Ui::blank();

        Ok(())

    }

    pub fn reset () -> AppResult<()> {

        Train::reset()?;

        Ui::blank();
        Ui::ok("training center re-seeded from the binary at ~/.agentx");
        Ui::blank();

        Ok(())

    }

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
            false => "Agentx.toml  (absent — run `agentx init` here)".to_string(),
        };

        let home = Env::home().unwrap_or_default();

        Ui::blank();
        Ui::title("agentx · project snapshot");

        Ui::blank();
        Ui::head("Project");
        Ui::field("root", &Path::display(&root));
        Ui::field("config", &config_note);
        Ui::field("cache", &Path::relative_one(&paths.cache, &root));

        let kind = match spec.project_type.is_empty() {
            true => "(unbound — run `agentx init` to classify)".to_string(),
            false => format!("{}  ·  {}", spec.project_type, Train::title(&spec.project_type)),
        };

        Ui::field("training", &kind);

        let live = match Proc::read_pid(&paths.pid) {
            Some(pid) if Proc::is_alive(pid) => format!("running (pid {pid})"),
            _ => "idle".to_string(),
        };

        Ui::field("run state", &live);

        Ui::blank();
        Ui::head("Config  ·  [project]  (Agentx.toml)");
        Ui::pair("project_type", &format!("{:?}", spec.project_type));
        Ui::pair("max_rounds", &spec.max_rounds.to_string());
        Ui::pair("max_fixes", &spec.max_fixes.to_string());
        Ui::pair("gate_cmd", &format!("{:?}", spec.gate_cmd));
        Ui::pair("gate_timeout", &spec.gate_timeout.to_string());
        Ui::pair("manager_model", &format!("{:?}", spec.manager_model));
        Ui::pair("architect_models", &format!("{:?}", spec.architect_models));
        Ui::pair("executor_models", &format!("{:?}", spec.executor_models));
        Ui::pair("tester_models", &format!("{:?}", spec.tester_models));

        Ui::blank();
        Ui::head("Rosters (expanded)");
        Ui::field("requires", &spec.roster("requires").join(" "));
        Ui::field("tasks", &spec.roster("tasks").join(" "));
        Ui::field("tests", &spec.roster("tests").join(" "));
        Ui::field("manager", config.manager());

        Ui::blank();
        Ui::head("Paths (.agentx runtime)");
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

        for name in CONTEXT_BUCKETS {

            Self::files(name, &Path::shorten_all(config.context.bucket(name), &root, &home));

        }

        Self::files("requires", &Path::shorten_all(&config.context.requires, &root, &home));

        Ui::blank();
        Ui::head("Journey (state.json)");

        if journey.journey_id.is_empty() {

            Ui::info("no journey yet — run `agentx start`");

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
            Ui::field("task_status", &Self::map_str(&journey.task_status));
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

    fn files ( label: &str, files: &[String] ) {

        Ui::field(label, &format!("{} file(s)", files.len()));

        for file in files {

            Ui::item(file);

        }

    }

    fn map_str ( map: &BTreeMap<String, String> ) -> String {

        if map.is_empty() { return "(none)".to_string(); }

        map.iter().map(|( key, value )| format!("{key}={value}")).collect::<Vec<_>>().join(", ")

    }

    fn sessions_of ( path: &StdPath ) -> Vec<( String, String )> {

        let body = File::read(path);

        if body.trim().is_empty() { return Vec::new(); }

        let map: HashMap<String, String> = Json::parse(&body).unwrap_or_default();
        let mut pairs: Vec<( String, String )> = map.into_iter().filter(|( _, id )| !id.is_empty()).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        pairs

    }

    fn configure ( paths: &Paths, project: Option<&str>, gate: Option<&str> ) -> AppResult<String> {

        let mut spec = Spec::load(&paths.config_file)?;

        if Self::apply_flags(&mut spec, project, gate)? { spec.save(&paths.config_file)?; }

        Ok(spec.project_type)

    }

    fn prepare ( paths: &Paths, project: Option<&str>, gate: Option<&str> ) -> AppResult<String> {

        let mut spec = Spec::load(&paths.config_file)?;
        let mut dirty = Self::apply_flags(&mut spec, project, gate)?;

        let mut want_type = spec.project_type.is_empty();
        let want_gate = spec.gate_cmd.trim().is_empty();

        if want_type && io::stdin().is_terminal() && let Some(name) = Self::choose_type()? {

            spec.project_type = name;
            dirty = true;
            want_type = false;

        }

        if want_type || want_gate {

            let what = match ( want_type, want_gate ) {
                ( true, true ) => "project type and gate command",
                ( true, false ) => "project type",
                _ => "gate command",
            };

            Ui::blank();
            Ui::step(&format!("consulting {} to detect the {what}", spec.manager_model));
            Ui::info("running the agent CLI — this can take a moment (Ctrl-C to skip)");

            let ( detected, command ) = Train::discover(&paths.root, &spec.manager_model, want_type, want_gate);

            if want_type {

                match &detected {
                    Some(name) => Ui::ok(&format!("project type    {name}")),
                    None => Ui::warn("could not detect a project type — left empty (set project_type or pass --project)"),
                }

            }

            if let Some(name) = detected { spec.project_type = name; dirty = true; }

            if want_gate {

                match &command {
                    Some(value) => Ui::ok(&format!("gate command    {value}")),
                    None => Ui::warn("no gate command set — the gate is skipped until you set gate_cmd"),
                }

            }

            if let Some(value) = command { spec.gate_cmd = value; dirty = true; }

            Ui::blank();

        }

        if dirty { spec.save(&paths.config_file)?; }

        Ok(spec.project_type)

    }

    fn apply_flags ( spec: &mut Spec, project: Option<&str>, gate: Option<&str> ) -> AppResult<bool> {

        let mut dirty = false;

        if let Some(value) = project {

            let name = Self::select_type(value)?;

            if name != spec.project_type { spec.project_type = name; dirty = true; }

        }

        if let Some(value) = gate {

            let command = value.trim();

            if !command.is_empty() && command != spec.gate_cmd {

                spec.gate_cmd = command.to_string();
                dirty = true;

            }

        }

        Ok(dirty)

    }

    fn select_type ( value: &str ) -> AppResult<String> {

        let types = Train::available();
        let input = value.trim();

        if let Ok(number) = input.parse::<usize>() {

            if number >= 1 && number <= types.len() { return Ok(types[number - 1].clone()); }

            return Err(AppError::message(format!("--project {number} is out of range - choose 1 to {}", types.len())));

        }

        let slug = Text::slug(input);

        if types.iter().any(|item| item == &slug) { return Ok(slug); }

        let mut known = String::new();

        for ( index, item ) in types.iter().enumerate() {

            known.push_str(&format!("\n  {}) {item}", index + 1));

        }

        Err(AppError::message(format!("unknown project '{value}' - known training-center types:{known}")))

    }

    fn choose_type () -> AppResult<Option<String>> {

        let types = Train::available();

        if types.is_empty() { return Ok(None); }

        let mut options = Vec::with_capacity(types.len() + 1);
        options.push("auto  ·  let the manager detect it".to_string());

        for name in &types {

            let title = Train::title(name);
            options.push(if title.is_empty() { name.clone() } else { format!("{name}  ·  {title}") });

        }

        Ui::blank();

        let picked = Term::select("  select the project archetype   (↑/↓ move · enter choose · q auto)", &options, 0)?;

        match picked {
            Some(0) | None => Ok(None),
            Some(index) => Ok(Some(types[index - 1].clone())),
        }

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

}
