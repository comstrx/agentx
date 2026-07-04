use std::path::Path as StdPath;
use std::time::Instant;

use crate::config::Config;
use crate::config::base::consts::{PHASES, PROBE_TIMEOUT, TURN_TIMEOUT, TOOL};
use crate::core::error::{AppError, AppResult};
use crate::core::env::Env;
use crate::core::proc::Proc;
use crate::core::text::Text;
use crate::config::worker::{Fault, Worker};
use crate::app::{App, Mark, Project, Ui};

impl App {

    pub fn doctor ( dir: &StdPath ) -> AppResult<()> {

        let config = Project::assemble(&Project::resolve_root(dir))?;

        Ui::blank();
        Ui::title(&format!("{TOOL} · doctor"));
        Ui::blank();
        Ui::step("checking the dependencies a run needs");
        Ui::blank();

        let ok = Self::run_checks(&config);

        Ui::blank();

        if ok {

            Ui::mark(0, Mark::Cool, "all dependencies are installed and runnable — you are clear to start");
            Ui::blank();

            return Ok(());

        }

        Ui::blank();

        Err(AppError::message(format!("missing or broken dependencies — install them, then run `{TOOL} doctor` again")))

    }

    pub(super) fn ensure_agents ( config: &Config ) -> AppResult<()> {

        Ui::rule("doctor · checking the dependencies and agents this run needs");

        let ok = Self::run_checks(config);

        if ok { return Ok(()); }

        Err(AppError::message(format!("a required dependency is missing or broken — run `{TOOL} doctor` for details")))

    }

    fn run_checks ( config: &Config ) -> bool {

        let mut all_ok = true;

        let triples = Self::triples(config);

        let mut backends: Vec<String> = Vec::new();

        for ( agent, _, _ ) in &triples {

            if !backends.iter().any(|name| name == agent) { backends.push(agent.clone()); }

        }

        let mut broken: Vec<String> = Vec::new();

        for agent in &backends {

            match Worker::resolve(agent) {
                None => {

                    all_ok = false;
                    broken.push(agent.clone());
                    Ui::cross(0, &format!("{agent:<8}  unsupported worker — add it under src/config/worker/, or fix the [agent] members"));

                }
                Some(program) => {

                    let ( found, detail ) = Self::probe(program);

                    if found { Ui::tick(0, &format!("{program:<8}  {detail}")); }
                    else {

                        all_ok = false;
                        broken.push(agent.clone());
                        Ui::cross(0, &format!("{program:<8}  {detail}"));

                    }

                }
            }

        }

        if !config.gate.command.trim().is_empty() {

            let ( found, detail ) = Self::probe("sh");

            if !found { all_ok = false; Ui::cross(0, &format!("{:<8}  {detail}", "sh")); }
            else { Ui::tick(0, &format!("{:<8}  {detail}", "sh")); }

        }

        for ( agent, model, effort ) in &triples {

            if broken.iter().any(|name| name == agent) { continue; }

            let started = Instant::now();
            let ( works, note ) = Self::probe_agent(agent, model, effort);

            if !works { all_ok = false; Ui::cross(0, &format!("{agent:<8}  {note}")); }
            else { Ui::done(0, Mark::Ok, &format!("{agent:<8}  {note}"), started); }

        }

        all_ok

    }

    fn triples ( config: &Config ) -> Vec<( String, String, String )> {

        let mut members = vec![config.agent.manager.clone()];

        for phase in PHASES {

            if config.option.active(phase) { members.extend(config.agent.members(phase).iter().cloned()); }

        }

        let mut out: Vec<( String, String, String )> = Vec::new();

        for member in members {

            if member.agent.trim().is_empty() { continue; }

            let ( model, effort ) = config.resolve_member(&member);
            let triple = ( member.agent.trim().to_string(), model, effort );

            if !out.contains(&triple) { out.push(triple); }

        }

        out

    }

    fn probe_agent ( agent: &str, model: &str, effort: &str ) -> ( bool, String ) {

        let prompt = "Reply with the single word: pong — nothing else. Do not use any tool and do not read or write any file.";
        let label = Self::engine_label(model, effort);

        let mut worker = Worker::new(agent);
        worker.cwd(&Env::temp_dir()).timeout(TURN_TIMEOUT).engine(model, effort);

        match worker.turn(prompt) {
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
