use std::path::Path as StdPath;

use crate::config::{Spec, Train};
use crate::config::base::consts::{CONSULT_FILE, MD_EXT};
use crate::core::fs::{File, Path};
use crate::app::{Compose, Flow, Orchestrator, Ui};

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

            Ui::arrow(0, "the manager is classifying the project against the training center");

            let prompt = Compose::manager_discover(&self.cfg, &target);

            match self.consult(&model, &answer, &prompt, Train::parse_type)? {
                Some(kind) => {

                    let _ = Train::create(&kind);
                    document.project.inspire = kind.clone();
                    self.cfg.spec.inspire = kind.clone();
                    Ui::tick(0, &format!("archetype · {kind}"));

                }
                None => Ui::bang(0, "could not classify the project — staying unbound (set [project].inspire or pass --inspire)"),
            }

        }

        if want_gate {

            Ui::arrow(0, "the manager is composing the quality gate");

            let prompt = Compose::manager_gate(&self.cfg, &target);

            match self.consult(&model, &answer, &prompt, |body| Train::parse_line(body, "gate:"))? {
                Some(command) => {

                    document.gate.command = command.clone();
                    self.cfg.gate.command = command.clone();
                    Ui::tick(0, &format!("gate · {command}"));

                }
                None => Ui::bang(0, "no gate command set — the gate is skipped until you set [gate].command"),
            }

        }

        document.save(&self.cfg.paths.config_file)?;

        Ok(())

    }

    fn consult ( &mut self, model: &str, answer: &StdPath, prompt: &str, parse: impl Fn(&str) -> Option<String> ) -> Flow<Option<String>> {

        File::remove(answer);

        self.deliver("manager", model, "", 0, prompt)?;

        let body = File::read(answer);
        File::remove(answer);

        Ok(parse(&body))

    }

}
