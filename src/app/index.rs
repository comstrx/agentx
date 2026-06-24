use std::collections::{BTreeSet, HashMap};
use std::io::{self, IsTerminal};
use std::path::{Path as StdPath, PathBuf};
use std::process::Command;
use nix::sys::signal::{SigSet, SigmaskHow, Signal, killpg, pthread_sigmask};
use nix::unistd::{Pid, setpgid};

use crate::config::{Config, Document, Paths, Spec, Train};
use crate::config::consts::{CONTEXT_BUCKETS, PROBE_TIMEOUT};
use crate::core::error::{AppError, AppResult};
use crate::core::support::env::Env;
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::parse::Json;
use crate::core::support::proc::Proc;
use crate::core::support::term::Term;
use crate::core::support::text::Text;
use super::arch::{App, Flags, Journey, Orchestrator, Phase, Project, Status, Ui};

impl App {

    pub fn init ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        let paths = Paths::new(dir);
        Train::init()?;
        Project::scaffold(&paths)?;

        Self::guard_signals();

        let bound = Self::prepare(&paths, flags)?;

        Ui::blank();
        Ui::ok(&format!("initialised  {}", dir.display()));

        if !bound.is_empty() {

            Ui::detail("inspiration", &bound);

        }

        Ui::blank();

        Ok(())

    }

    pub fn start ( dir: &StdPath, flags: &Flags ) -> AppResult<()> {

        if flags.background { return Self::spawn_background(dir, flags); }

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is already active (pid {pid}); stop or drain it first")));

        }

        File::remove(&paths.drain);

        Self::init(&root, flags)?;

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

        let _ = setpgid(Pid::from_raw(0), Pid::from_raw(0));
        Proc::write_pid(&paths.pid)?;

        let mut orchestrator = Orchestrator::new(&config);

        Ui::loading("orchestrating");

        let result = orchestrator.run();

        Ui::loaded();

        File::remove(&paths.pid);
        File::remove(&paths.active);
        File::remove(&paths.drain);

        if result.is_ok() && orchestrator.journey.status == Status::Completed {

            Project::clear(&paths);
            Ui::ok("runtime cleared — .agentx reset to a clean slate (layout kept)");
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

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is already active (pid {pid}); stop or drain it first")));

        }

        let exe = std::env::current_exe().map_err(|error| AppError::message(format!("cannot locate the agentx binary: {error}")))?;
        let log = paths.configs.join("run.log");

        let mut command = Command::new(exe);
        command.arg("start").arg("--dir").arg(&root);

        if let Some(name) = flags.inspire { command.arg("--inspire").arg(name); }

        if let Some(value) = flags.gate { command.arg("--gate").arg(value); }

        if let Some(value) = flags.tests { command.arg("--tests").arg(value); }

        Self::forward_paths(&mut command, dir, "--ignore", flags.ignore);
        Self::forward_paths(&mut command, dir, "--include", flags.include);

        command.current_dir(&root);

        let pid = Proc::detach(command, &log)?;

        Ui::blank();
        Ui::ok(&format!("started in the background — pid {pid}"));
        Ui::detail("logs", &Path::relative_one(&log, &root));
        Ui::detail("control", "agentx status · agentx drain · agentx stop");
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

        for pid_file in [&paths.active, &paths.pid] {

            if let Some(pid) = Proc::read_pid(pid_file) && Proc::is_alive(pid) {

                let _ = killpg(Pid::from_raw(pid), Signal::SIGTERM);

            }

        }

        File::remove(&paths.active);
        File::remove(&paths.pid);
        Self::mark(&paths, Status::Stopped);

        Ui::ok(&format!("stopped the running cycle{position} — `start` resumes from the saved cursor"));
        Ui::blank();

        Ok(())

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

    fn is_running ( paths: &Paths ) -> bool {

        Proc::read_pid(&paths.pid).is_some_and(Proc::is_alive)

    }

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

        let agentx = Proc::read_pid(&paths.pid);
        let active = Proc::read_pid(&paths.active);
        let running = agentx.is_some_and(Proc::is_alive);
        let worker_live = active.is_some_and(Proc::is_alive);

        Ui::blank();
        Ui::title("agentx · status");
        Ui::blank();

        let state = match ( running, agentx ) {
            ( true, Some(pid) ) => format!("running   ·   pid {pid}"),
            _ => "idle".to_string(),
        };

        Ui::state("state", running, &state);

        let log = paths.configs.join("run.log");

        if running && Path::exists(&log) { Ui::field("logs", &Path::relative_one(&log, &root)); }

        let ( claude, codex ) = Spec::document(&paths.config_file)?.engines();

        Ui::blank();
        Ui::head("Engines  ·  model · effort in use");
        Ui::field("claude", &format!("model {}  ·  effort {}", claude.0, claude.1));
        Ui::field("codex", &format!("model {}  ·  effort {}", codex.0, codex.1));

        if journey.journey_id.is_empty() {

            Ui::blank();
            Ui::info("no journey yet — run `agentx start`");
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
        Ui::field("agentx", &Self::pid_line(agentx, running));
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
            Phase::Tasks => "tasks",
            Phase::Tests => "tests",
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

    pub fn clear ( dir: &StdPath ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        Ui::blank();

        if !paths.cache.exists() {

            Ui::info("nothing to clear");
            Ui::blank();
            return Ok(());

        }

        if let Some(pid) = Proc::read_pid(&paths.pid) && Proc::is_alive(pid) {

            return Err(AppError::message(format!("a run is active (pid {pid}); stop or drain it before clearing")));

        }

        Project::clear(&paths);
        Ui::ok(&format!("cleared {} — kept the directory layout", Path::relative_one(&paths.cache, &root)));
        Ui::blank();

        Ok(())

    }

    pub fn ignore ( dir: &StdPath, paths: &[PathBuf] ) -> AppResult<()> {

        Self::classify_paths(dir, paths, &[])

    }

    pub fn include ( dir: &StdPath, paths: &[PathBuf] ) -> AppResult<()> {

        Self::classify_paths(dir, &[], paths)

    }

    fn classify_paths ( dir: &StdPath, ignore: &[PathBuf], include: &[PathBuf] ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);

        let mut spec = Spec::load(&paths.config_file)?;
        let mut dirty = Self::merge_into(&mut spec, &root, dir, ignore, false);
        dirty |= Self::merge_into(&mut spec, &root, dir, include, true);

        Ui::blank();

        if !dirty {

            Ui::info("nothing changed — those paths are already classified that way");
            Ui::blank();

            return Ok(());

        }

        spec.save(&paths.config_file)?;

        if !ignore.is_empty() {

            Ui::ok(&format!("ignore — {} path(s) skipped during classification", spec.ignore.len()));

            for entry in &spec.ignore { Ui::item(entry); }

        }

        if !include.is_empty() {

            Ui::ok(&format!("include — {} path(s) forced in (overrides ignore)", spec.include.len()));

            for entry in &spec.include { Ui::item(entry); }

        }

        Ui::blank();

        Ok(())

    }

    pub fn refresh ( dir: &StdPath, ignore: &[PathBuf], include: &[PathBuf] ) -> AppResult<()> {

        let root = Project::resolve_root(dir);
        let paths = Paths::new(&root);
        Train::init()?;

        let mut spec = Spec::load(&paths.config_file)?;
        spec.ignore.clear();
        spec.include.clear();

        Self::merge_into(&mut spec, &root, dir, ignore, false);
        Self::merge_into(&mut spec, &root, dir, include, true);

        spec.save(&paths.config_file)?;

        let config = Project::assemble(&root)?;

        Ui::blank();
        Ui::ok("classification refreshed — ignore/include lists reset");

        if !config.spec.ignore.is_empty() { Ui::detail("ignore", &format!("{:?}", config.spec.ignore)); }

        if !config.spec.include.is_empty() { Ui::detail("include", &format!("{:?}", config.spec.include)); }

        Ui::blank();
        Ui::head("Classification (briefing files injected per bucket)");

        Self::classification(&config, &root);

        Ui::blank();

        Ok(())

    }

    pub fn sync () -> AppResult<()> {

        Train::sync()?;

        Ui::blank();
        Ui::ok("training center synced from the binary — learned history kept");
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

        Ui::blank();
        Ui::title("agentx · project snapshot");

        Ui::blank();
        Ui::head("Project");
        Ui::field("root", &Path::display(&root));
        Ui::field("config", &config_note);
        Ui::field("cache", &Path::relative_one(&paths.cache, &root));

        let kind = match spec.inspire.is_empty() {
            true => "(unbound — run `agentx init` to classify)".to_string(),
            false => spec.inspire.clone(),
        };

        Ui::field("inspiration", &kind);

        let live = match Proc::read_pid(&paths.pid) {
            Some(pid) if Proc::is_alive(pid) => format!("running (pid {pid})"),
            _ => "idle".to_string(),
        };

        Ui::field("run state", &live);

        Ui::blank();
        Ui::head("Config  ·  [project]  (Agentx.toml)");
        Ui::pair("inspire", &format!("{:?}", spec.inspire));
        Ui::pair("tests", &spec.tests.to_string());
        Ui::pair("max_rounds", &spec.max_rounds.to_string());
        Ui::pair("max_fixes", &spec.max_fixes.to_string());

        if !spec.ignore.is_empty() { Ui::pair("ignore", &format!("{:?}", spec.ignore)); }

        if !spec.include.is_empty() { Ui::pair("include", &format!("{:?}", spec.include)); }

        Ui::blank();
        Ui::head("[gate]");
        Ui::pair("timeout", &config.gate.timeout.to_string());
        Ui::pair("command", &format!("{:?}", config.gate.command));

        Ui::blank();
        Ui::head("[agent]");
        Ui::pair("timeout", &config.agent.timeout.to_string());
        Ui::pair("manager", &format!("{:?}", config.agent.manager));
        Ui::pair("architects", &format!("{:?}", config.agent.architects));
        Ui::pair("executors", &format!("{:?}", config.agent.executors));
        Ui::pair("testers", &format!("{:?}", config.agent.testers));

        Ui::blank();
        Ui::head("Rosters (expanded)");
        Ui::field("requires", &config.roster("requires").join(" "));
        Ui::field("tasks", &config.roster("tasks").join(" "));
        Ui::field("tests", &config.roster("tests").join(" "));
        Ui::field("manager", config.manager());

        Ui::blank();
        Ui::head("Engines (model · effort · empty field → strong default)");
        Ui::field("claude", &Self::engine_line(config.engine("claude")));
        Ui::field("codex", &Self::engine_line(config.engine("codex")));

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

        Self::classification(&config, &root);

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

    fn files ( label: &str, files: &[String] ) {

        Ui::field(label, &format!("{} file(s)", files.len()));

        for file in files {

            Ui::item(file);

        }

    }

    fn engine_line ( engine: ( String, String ) ) -> String {

        format!("model {}  ·  effort {}", engine.0, engine.1)

    }

    fn classification ( config: &Config, root: &StdPath ) {

        let home = Env::home().unwrap_or_default();

        for name in CONTEXT_BUCKETS {

            Self::files(name, &Path::shorten_all(config.context.bucket(name), root, &home));

        }

        Self::files("requires", &Path::shorten_all(&config.context.requires, root, &home));

    }

    fn sessions_of ( path: &StdPath ) -> Vec<( String, String )> {

        let body = File::read(path);

        if body.trim().is_empty() { return Vec::new(); }

        let map: HashMap<String, String> = Json::parse(&body).unwrap_or_default();
        let mut pairs: Vec<( String, String )> = map.into_iter().filter(|( _, id )| !id.is_empty()).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        pairs

    }

    pub fn doctor ( dir: &StdPath ) -> AppResult<()> {

        let config = Project::assemble(&Project::resolve_root(dir))?;

        Ui::blank();
        Ui::title("agentx · doctor");
        Ui::blank();
        Ui::step("checking the dependencies a run needs");
        Ui::blank();

        let ok = Self::run_checks(&config, true);

        Ui::blank();

        if ok {

            Ui::ok("all dependencies are installed and runnable — you are clear to start");
            Ui::blank();

            return Ok(());

        }

        Ui::blank();

        Err(AppError::message("missing or broken dependencies — install them, then run `agentx doctor` again"))

    }

    fn ensure_agents ( config: &Config ) -> AppResult<()> {

        if Self::run_checks(config, false) { return Ok(()); }

        Err(AppError::message("a required dependency is missing or broken — run `agentx doctor` for details"))

    }

    fn run_checks ( config: &Config, verbose: bool ) -> bool {

        let mut all_ok = true;

        for program in Self::required_programs(config) {

            let ( ok, detail ) = Self::probe(&program);

            if !ok { all_ok = false; }

            if ok && verbose {

                Ui::tick(0, &format!("{program:<8}  {detail}"));

            }
            else if !ok {

                Ui::cross(0, &format!("{program:<8}  {detail}"));

            }

        }

        all_ok

    }

    fn required_programs ( config: &Config ) -> BTreeSet<String> {

        let agent = &config.agent;
        let mut programs: BTreeSet<String> = BTreeSet::new();

        for model in std::iter::once(&agent.manager).chain(&agent.architects).chain(&agent.executors).chain(&agent.testers) {

            let backend = if model.trim().starts_with("codex") { "codex" } else { "claude" };
            programs.insert(backend.to_string());

        }

        if !config.gate.command.trim().is_empty() { programs.insert("sh".to_string()); }

        programs

    }

    fn probe ( program: &str ) -> ( bool, String ) {

        if Env::which(program).is_none() {

            return ( false, "not found on PATH — install it (or change the [agent] models)".to_string() );

        }

        match Proc::command(program, &["--version"], PROBE_TIMEOUT) {
            Ok(output) if output.code == 0 => {

                let line = Text::first_line(&output.stdout);
                let line = if line.trim().is_empty() { Text::first_line(&output.stderr) } else { line };

                match line.trim().is_empty() {
                    true => ( true, "installed".to_string() ),
                    false => ( true, line.trim().to_string() ),
                }

            }
            Ok(_) => ( true, "installed".to_string() ),
            Err(_) => ( false, "found on PATH but failed to run".to_string() ),
        }

    }

    fn prepare ( paths: &Paths, flags: &Flags ) -> AppResult<String> {

        let mut document = Spec::document(&paths.config_file)?;
        let mut dirty = Self::apply_flags(&mut document, flags.inspire, flags.gate, flags.tests)?;

        let mut want_inspire = document.project.inspire.is_empty();
        let want_gate = document.gate.command.is_empty();

        if want_inspire && io::stdin().is_terminal() && let Some(name) = Self::choose_inspire()? {

            document.project.inspire = name;
            dirty = true;
            want_inspire = false;

        }

        if want_inspire || want_gate {

            let what = match ( want_inspire, want_gate ) {
                ( true, true ) => "inspiration and gate command",
                ( true, false ) => "inspiration",
                _ => "gate command",
            };

            Ui::blank();
            Ui::step(&format!("consulting {} to detect the {what}", document.agent.manager));
            Ui::info("running the agent CLI — this can take a moment (Ctrl-C to skip)");

            let ( detected, command ) = Train::discover(&paths.root, &document.agent.manager, want_inspire, want_gate);

            if want_inspire {

                match &detected {
                    Some(name) => Ui::ok(&format!("inspiration    {name}")),
                    None => Ui::warn("could not detect an inspiration — left empty (set inspire or pass --inspire)"),
                }

            }

            if let Some(name) = detected { document.project.inspire = name; dirty = true; }

            if want_gate {

                match &command {
                    Some(value) => Ui::ok(&format!("gate command    {value}")),
                    None => Ui::warn("no gate command set — the gate is skipped until you set [gate].command"),
                }

            }

            if let Some(value) = command { document.gate.command = value; dirty = true; }

            Ui::blank();

        }

        dirty |= document.fill_defaults();

        if dirty { document.save(&paths.config_file)?; }

        Ok(document.project.inspire)

    }

    fn apply_flags ( document: &mut Document, inspire: Option<&str>, gate: Option<&str>, tests: Option<&str> ) -> AppResult<bool> {

        let mut dirty = false;

        if let Some(value) = inspire {

            let name = Self::select_inspire(value)?;

            if name != document.project.inspire { document.project.inspire = name; dirty = true; }

        }

        if let Some(value) = gate {

            let command = value.trim();

            if !command.is_empty() && command != document.gate.command {

                document.gate.command = command.to_string();
                dirty = true;

            }

        }

        if let Some(value) = tests {

            let on = Spec::parse_tests(value)
                .ok_or_else(|| AppError::message(format!("invalid --tests value {value:?} (use true/false, 1/0, yes/no)")))?;

            if on != document.project.tests { document.project.tests = on; dirty = true; }

        }

        Ok(dirty)

    }

    fn merge_into ( spec: &mut Spec, root: &StdPath, base: &StdPath, paths: &[PathBuf], into_include: bool ) -> bool {

        let mut dirty = false;

        for path in paths {

            let entry = Path::relativize(root, base, path);

            if entry.is_empty() { continue; }

            let ( target, other ) = match into_include {
                true => ( &mut spec.include, &mut spec.ignore ),
                false => ( &mut spec.ignore, &mut spec.include ),
            };

            let before = other.len();
            other.retain(|existing| existing != &entry);

            if other.len() != before { dirty = true; }

            if !target.contains(&entry) {

                target.push(entry);
                dirty = true;

            }

        }

        dirty

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

    fn select_inspire ( value: &str ) -> AppResult<String> {

        let types = Train::available();
        let input = value.trim();

        if let Ok(number) = input.parse::<usize>() {

            if number >= 1 && number <= types.len() { return Ok(types[number - 1].clone()); }

            return Err(AppError::message(format!("--inspire {number} is out of range - choose 1 to {}", types.len())));

        }

        let slug = Text::slug(input);

        if types.iter().any(|item| item == &slug) { return Ok(slug); }

        let mut known = String::new();

        for ( index, item ) in types.iter().enumerate() {

            known.push_str(&format!("\n  {}) {item}", index + 1));

        }

        Err(AppError::message(format!("unknown inspiration '{value}' - known training-center archetypes:{known}")))

    }

    fn choose_inspire () -> AppResult<Option<String>> {

        let types = Train::available();

        if types.is_empty() { return Ok(None); }

        let mut options = Vec::with_capacity(types.len() + 1);
        options.push("auto  ·  let the manager detect it".to_string());

        for name in &types {

            let title = Train::title(name);
            options.push(if title.is_empty() { name.clone() } else { format!("{name}  ·  {title}") });

        }

        Ui::blank();

        let picked = Term::select("  select the inspiration archetype   (↑/↓ move · enter choose · q auto)", &options, 0)?;

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

    fn guard_signals () {

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
