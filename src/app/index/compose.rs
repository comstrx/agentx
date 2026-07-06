use std::path::Path as StdPath;

use crate::config::{Paths, Train};
use crate::config::base::consts::{CONFIG_FILE, TOOL};
use crate::core::error::{AppError, AppResult};
use crate::core::fs::{Dir, File, Path};
use crate::app::{App, Compose, Journey, Project, Stage, Ui};

impl App {

    pub fn compose ( dir: &StdPath, role: Option<&str>, phase: Option<&str>, out: Option<&StdPath>, force: bool ) -> AppResult<()> {

        let root = Project::resolve_root(dir);

        if !Path::exists(&Paths::new(&root).config_file) {

            return Err(AppError::message(format!("no {CONFIG_FILE} here — run `{TOOL} init` first")));

        }

        let role = role.map(Self::resolve_role).transpose()?;
        let stage = phase.map(Self::resolve_stage).transpose()?;

        if let ( Some(role), Some(stage) ) = ( role.as_deref(), stage ) {

            if role != "manager" && stage.manager_only() {

                return Err(AppError::message(format!("--phase {phase} is the manager's turn — use --role manager, or drop --role", phase = phase.unwrap_or_default())));

            }

            if role == "manager" && stage.worker_only() {

                return Err(AppError::message(format!("the manager carries no {phase} fragment — it reviews, it does not build", phase = phase.unwrap_or_default())));

            }

        }

        Train::init()?;

        let mut cfg = Project::assemble(&root)?;
        cfg.force = force;

        let journey = Journey::load(&cfg.paths.state);
        let blocks = Compose::preview(&cfg, &journey, role.as_deref(), stage);

        if blocks.is_empty() { return Err(AppError::message("nothing matched — that role runs no such turn under the current [option] switches")); }

        let Some(target) = out else {

            let text = blocks.iter()
                .map(|( title, body )| format!("{}\n\n{body}\n", Ui::strong(&format!("# {title}"))))
                .collect::<Vec<_>>()
                .join(&format!("\n{}\n\n", Ui::dim("---")));

            print!("{text}");

            return Ok(());

        };

        let text = blocks.iter()
            .map(|( title, body )| format!("# {title}\n\n{body}\n"))
            .collect::<Vec<_>>()
            .join("\n---\n\n");

        let target = match target.is_absolute() {
            true => target.to_path_buf(),
            false => dir.join(target),
        };

        Dir::ensure_parent(&target)?;
        File::write_atomic(&target, &text)?;

        Ui::blank();
        Ui::ok(&format!("wrote {} prompt block(s) · {}", blocks.len(), Path::display(&target)));
        Ui::blank();

        Ok(())

    }

    fn resolve_role ( input: &str ) -> AppResult<String> {

        let role = match input.trim().to_ascii_lowercase().as_str() {
            "manager" | "mgr" => "manager",
            "requires" | "require" | "requirement" | "requirements" | "architect" | "architects" | "arch" => "requires",
            "tasks" | "task" | "executor" | "executors" | "executer" | "exec" => "tasks",
            "audits" | "audit" | "auditor" | "auditors" => "audits",
            "tests" | "test" | "tester" | "testers" => "tests",
            "benches" | "bench" | "bencher" | "benchers" => "benches",
            "examples" | "example" | "exampler" | "examplers" => "examples",
            "fuzzes" | "fuzz" | "fuzzer" | "fuzzers" => "fuzzes",
            _ => return Err(AppError::message(format!("unknown role '{input}' — one of: manager, architect, executor, auditor, tester, bencher, exampler, fuzzer (phase names work too)"))),
        };

        Ok(role.to_string())

    }

    fn resolve_stage ( input: &str ) -> AppResult<Stage> {

        let stage = match input.trim().to_ascii_lowercase().as_str() {
            "init" | "prime" | "priming" => Stage::Init,
            "confirm" | "reaffirm" | "recall" => Stage::Confirm,
            "work" | "turn" => Stage::Work,
            "intake" | "questions" | "checks" => Stage::Intake,
            "convert" | "backlog" => Stage::Convert,
            "discover" | "discovery" | "classify" | "inspire" | "binding" => Stage::Discover,
            "gate" => Stage::Gate,
            "review" | "reviews" | "verdict" => Stage::Review,
            "finalize" | "final" => Stage::Finalize,
            "fence" | "fences" => Stage::Fence,
            "evidence" => Stage::Evidence,
            "report" | "reports" => Stage::Report,
            _ => return Err(AppError::message(format!("unknown phase '{input}' — one of: init, confirm, work, intake, convert, discover, gate, review, finalize, fence, evidence, report"))),
        };

        Ok(stage)

    }

}
