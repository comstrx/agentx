use std::path::Path as StdPath;
use std::time::Instant;

use crate::config::{Spec, Train};
use crate::config::base::consts::{CONSULT_FILE, MD_EXT};
use crate::core::fs::{File, Path};
use crate::app::{Compose, Flow, Mark, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn discover ( &mut self ) -> Flow<()> {

        let want_inspire = self.cfg.spec.inspire.trim().is_empty();
        let want_gate = self.cfg.gate.command.trim().is_empty();

        if !want_inspire && !want_gate { return Ok(()); }

        Ui::rule("discovery · the manager classifies the project and sets the gate");

        let model = self.cfg.manager().to_string();
        let answer = self.cfg.paths.configs.join(format!("{CONSULT_FILE}.{MD_EXT}"));
        let target = Path::display(&answer);

        let mut document = Spec::document(&self.cfg.paths.config_file)?;

        if want_inspire {

            Ui::working(0, Mark::Think, "the manager is classifying the project against the training center");

            let started = Instant::now();
            let prompt = Compose::manager_discover(&self.cfg, &target);

            match self.consult(&model, &answer, &prompt, Train::parse_type)? {
                Some(( fresh, slug )) => {

                    let known = Train::available().iter().any(|name| name.eq_ignore_ascii_case(&slug));

                    document.project.inspire = slug.clone();
                    self.cfg.spec.inspire = slug.clone();

                    if fresh || !known {

                        let _ = Train::history(&slug);
                        Ui::done(0, Mark::Ok, &format!("history · {slug}"), started);
                        Ui::detail("note", "no curated project node fit — a fresh accumulation line starts under history/; add a project node later if this kind recurs");

                    }
                    else {

                        Ui::done(0, Mark::Ok, &format!("project node · {slug}"), started);

                    }

                }
                None => Ui::bang(0, "could not classify the project — staying unbound (set [project].inspire or pass --inspire)"),
            }

        }

        if want_gate {

            Ui::working(0, Mark::Think, "the manager is composing the quality gate");

            let started = Instant::now();
            let prompt = Compose::manager_gate(&self.cfg, &target);

            match self.consult(&model, &answer, &prompt, |body| Train::parse_line(body, "gate:"))? {
                Some(command) => {

                    document.gate.command = command.clone();
                    self.cfg.gate.command = command.clone();
                    Ui::done(0, Mark::Ok, &format!("gate · {command}"), started);

                }
                None => Ui::bang(0, "no gate command set — the gate is skipped until you set [gate].command"),
            }

        }

        document.save(&self.cfg.paths.config_file)?;

        Ok(())

    }

    fn consult <T> ( &mut self, model: &str, answer: &StdPath, prompt: &str, parse: impl Fn(&str) -> Option<T> ) -> Flow<Option<T>> {

        File::remove(answer);

        self.deliver("manager", model, "", 0, prompt)?;

        let body = File::read(answer);
        File::remove(answer);

        Ok(parse(&body))

    }

}
