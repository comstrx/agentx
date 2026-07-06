use std::path::{Path as StdPath, PathBuf};

use crate::config::{Config, base::prompts as P};
use crate::config::base::consts::{CONSULT_FILE, MD_EXT, PHASES};
use crate::core::fs::{Dir, Path};
use crate::app::Journey;
use super::arch::{Compose, Stage};

impl Compose {

    pub(crate) fn preview ( cfg: &Config, journey: &Journey, role: Option<&str>, stage: Option<Stage> ) -> Vec<( String, String )> {

        let mut blocks: Vec<( String, String )> = Vec::new();

        let phases: Vec<&str> = match role {
            Some(name) if name != "manager" => PHASES.into_iter().filter(|phase| *phase == name).collect(),
            _ => PHASES.into_iter().filter(|phase| cfg.option.active(phase)).collect(),
        };

        let manager = role.is_none_or(|name| name == "manager");
        let workers = role.is_none_or(|name| name != "manager");
        let want = |this: Stage| stage.is_none_or(|wanted| wanted == this);
        let manager_work = stage == Some(Stage::Work);
        let answer = Path::display(&cfg.paths.configs.join(format!("{CONSULT_FILE}.{MD_EXT}")));
        let task = Self::sample_task(cfg, journey);

        if let Some(fragment) = stage.filter(|s| matches!(s, Stage::Fence | Stage::Evidence | Stage::Report)) {

            Self::fragments(&mut blocks, cfg, &phases, manager, workers, fragment, task.as_deref());

            return blocks;

        }

        if manager && want(Stage::Init) { blocks.push(( "manager · init".to_string(), Self::manager_brief(cfg, journey) )); }

        if manager && want(Stage::Confirm) { blocks.push(( "manager · confirm".to_string(), Self::reaffirm(cfg, cfg.manager()) )); }

        if !cfg.force {

            if manager && want(Stage::Intake) { blocks.push(( "manager · intake · project".to_string(), Self::manager_project_check(cfg) )); }

            if manager && want(Stage::Discover) { blocks.push(( "manager · discover".to_string(), Self::manager_discover(cfg, &answer) )); }

            if manager && ( want(Stage::Intake) || want(Stage::Gate) ) { blocks.push(( "manager · intake · gate".to_string(), Self::manager_gate_check(cfg) )); }

            if manager && want(Stage::Intake) {

                blocks.push(( "manager · intake · requirements".to_string(), Self::manager_requires_check(cfg) ));

                if stage == Some(Stage::Intake) {

                    blocks.push(( "manager · intake · fix · project".to_string(), Self::manager_project_fix(cfg) ));
                    blocks.push(( "manager · intake · fix · requirements".to_string(), Self::manager_requires_fix(cfg) ));

                }

            }

            if manager && matches!(stage, Some(Stage::Gate) | Some(Stage::Intake)) { blocks.push(( "manager · intake · fix · gate".to_string(), Self::manager_gate_fix(cfg, &answer) )); }

        }

        if workers && want(Stage::Init) {

            for ( phase, seat ) in Self::exemplars(cfg, &phases) {

                blocks.push(( format!("{} · init", Self::role_label(&phase)), Self::prime(cfg, journey, &phase, &seat) ));

            }

        }

        if workers && want(Stage::Confirm) {

            for ( phase, seat ) in Self::exemplars(cfg, &phases) {

                blocks.push(( format!("{} · confirm", Self::role_label(&phase)), Self::reaffirm(cfg, &seat) ));

            }

        }

        if manager && ( want(Stage::Convert) || manager_work ) { blocks.push(( "manager · convert".to_string(), Self::manager_convert(cfg, journey, None) )); }

        if workers && want(Stage::Work) {

            for ( phase, seat ) in Self::exemplars(cfg, &phases) {

                if phase == "tasks" && task.is_none() {

                    blocks.push(( "executor · work".to_string(), "(skipped — the executor turn wraps a real task file, and the backlog holds none yet)".to_string() ));
                    continue;

                }

                let bound = if phase == "tasks" { task.as_deref() } else { None };

                blocks.push(( format!("{} · work", Self::role_label(&phase)), Self::work(cfg, journey, &phase, &seat, bound, false, false) ));

            }

        }

        if manager && ( want(Stage::Review) || manager_work ) {

            let reviewed: Vec<&str> = match role {
                Some(name) if name != "manager" => Vec::new(),
                _ => PHASES.into_iter().filter(|phase| cfg.option.active(phase)).collect(),
            };

            for phase in reviewed {

                if phase == "tasks" && task.is_none() {

                    blocks.push(( "manager · review · tasks".to_string(), "(skipped — the tasks review wraps a real task file, and the backlog holds none yet)".to_string() ));
                    continue;

                }

                let bound = if phase == "tasks" { task.as_deref() } else { None };

                blocks.push(( format!("manager · review · {phase}"), Self::manager_review(cfg, phase, bound, 1, true) ));

            }

        }

        if manager && want(Stage::Finalize) { blocks.push(( "manager · finalize".to_string(), Self::manager_finalize(cfg) )); }

        blocks

    }

    fn fragments ( blocks: &mut Vec<( String, String )>, cfg: &Config, phases: &[&str], manager: bool, workers: bool, fragment: Stage, task: Option<&StdPath> ) {

        if fragment == Stage::Evidence {

            blocks.push(( "evidence".to_string(), P::EVIDENCE.to_string() ));

            return;

        }

        if !workers {

            if manager && fragment == Stage::Report {

                blocks.push(( "manager · report".to_string(), Self::render(&[P::MANAGER_VERDICT.to_string()], &Self::values(cfg, "requires", "manager", None)) ));

            }

            return;

        }

        for ( phase, seat ) in Self::exemplars(cfg, phases) {

            let bound = if phase == "tasks" { task } else { None };

            let ( title, body ) = match fragment {
                Stage::Fence => (
                    format!("{} · fence", Self::role_label(&phase)),
                    Self::render(&[P::WRITE_FENCE.to_string()], &[( "fence", Self::fence(cfg, &phase, &seat) )]),
                ),
                _ => {

                    let part = match phase.as_str() {
                        "requires" => P::REQUIRES_REPORT,
                        "tasks"    => P::TASKS_REPORT,
                        "audits"   => P::AUDITS_REPORT,
                        _          => P::PRODUCE_REPORT,
                    };

                    let mut pairs = Self::values(cfg, &phase, &seat, bound);
                    pairs.push(( "phase", phase.clone() ));
                    pairs.push(( "duty", Self::duty_of(&phase) ));

                    ( format!("{} · report", Self::role_label(&phase)), Self::render(&[part.to_string()], &pairs) )

                }
            };

            blocks.push(( title, body ));

        }

        if manager && fragment == Stage::Report {

            blocks.push(( "manager · report".to_string(), Self::render(&[P::MANAGER_VERDICT.to_string()], &Self::values(cfg, "requires", "manager", None)) ));

        }

    }

    fn exemplars ( cfg: &Config, phases: &[&str] ) -> Vec<( String, String )> {

        phases.iter()
            .filter_map(|phase| cfg.roster(phase).first().map(|seat| ( (*phase).to_string(), seat.clone() )))
            .collect()

    }

    fn sample_task ( cfg: &Config, journey: &Journey ) -> Option<PathBuf> {

        let current = cfg.paths.tasks.join(&journey.current_task);

        if !journey.current_task.is_empty() && current.exists() { return Some(current); }

        Dir::markdown(&cfg.paths.tasks).into_iter().next()

    }

}
