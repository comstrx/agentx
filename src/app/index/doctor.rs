use std::collections::BTreeSet;
use std::path::Path as StdPath;

use crate::config::Config;
use crate::config::base::consts::{PROBE_PROMPT, PROBE_TIMEOUT, PROBE_TURN_TIMEOUT, TOOL};
use crate::core::error::{AppError, AppResult};
use crate::core::env::Env;
use crate::core::proc::Proc;
use crate::core::text::Text;
use crate::config::worker::{Fault, Worker};
use crate::app::{App, Project, Ui};

impl App {

    pub fn doctor ( dir: &StdPath ) -> AppResult<()> {

        let config = Project::assemble(&Project::resolve_root(dir))?;

        Ui::blank();
        Ui::title(&format!("{TOOL} · doctor"));
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

        Err(AppError::message(format!("missing or broken dependencies — install them, then run `{TOOL} doctor` again")))

    }

    pub(super) fn ensure_agents ( config: &Config ) -> AppResult<()> {

        Ui::rule("doctor · checking the dependencies and agents this run needs");

        let ok = Self::run_checks(config, true);

        if ok { return Ok(()); }

        Err(AppError::message(format!("a required dependency is missing or broken — run `{TOOL} doctor` for details")))

    }

    fn run_checks ( config: &Config, verbose: bool ) -> bool {

        let mut all_ok = true;

        for model in Self::models(config) {

            if Worker::resolve(&model).is_none() {

                all_ok = false;
                Ui::cross(0, &format!("{model:<8}  unsupported worker — add it under src/config/worker/, or fix the [agent] models"));

            }

        }

        for program in Self::required_programs(config) {

            let ( found, detail ) = Self::probe(&program);

            if !found {

                all_ok = false;
                Ui::cross(0, &format!("{program:<8}  {detail}"));

                continue;

            }

            if program == "sh" {

                if verbose { Ui::tick(0, &format!("{program:<8}  {detail}")); }

                continue;

            }

            let ( works, note ) = Self::probe_agent(config, &program);

            if !works {

                all_ok = false;
                Ui::cross(0, &format!("{program:<8}  {note}"));

            }
            else if verbose {

                Ui::tick(0, &format!("{program:<8}  {note}"));

            }

        }

        all_ok

    }

    fn probe_agent ( config: &Config, backend: &str ) -> ( bool, String ) {

        let ( model, effort ) = config.engine(backend);
        let label = Self::engine_label(&model, &effort);

        let mut worker = Worker::new(backend);
        worker.cwd(&Env::temp_dir()).timeout(PROBE_TURN_TIMEOUT).engine(&model, &effort);

        match worker.turn(PROBE_PROMPT) {
            Ok(_) => ( true, format!("{label} — responding") ),
            Err(error) => match Worker::fault(&error) {
                Fault::Transient => ( true, format!("{label} — responding") ),
                _ => ( false, format!("{label} — {}", error.detail().chars().take(140).collect::<String>()) ),
            }
        }

    }

    fn engine_label ( model: &str, effort: &str ) -> String {

        let model = if model.trim().is_empty() { "default" } else { model.trim() };
        let effort = if effort.trim().is_empty() { "default" } else { effort.trim() };

        format!("model {model} · effort {effort}")

    }

    fn models ( config: &Config ) -> BTreeSet<String> {

        let agent = &config.agent;

        let mut all = vec![agent.manager.clone()];

        for roster in [&agent.requires, &agent.tasks, &agent.tests, &agent.benches, &agent.examples, &agent.fuzzes] {

            all.extend(roster.iter().cloned());

        }

        all.iter().map(|model| model.trim().to_string()).filter(|model| !model.is_empty()).collect()

    }

    fn required_programs ( config: &Config ) -> BTreeSet<String> {

        let mut programs: BTreeSet<String> = Self::models(config).iter().filter_map(|model| Worker::resolve(model)).map(str::to_string).collect();

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

}
