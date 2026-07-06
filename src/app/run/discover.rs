use std::path::Path as StdPath;
use std::time::Instant;

use crate::config::{Spec, Train};
use crate::config::base::consts::{CONFIG_FILE, CONSULT_FILE, MD_EXT, TOOL};
use crate::core::fs::{File, Path};
use crate::app::{Compose, Flow, Mark, Orchestrator, Project, Ui};

impl Orchestrator {

    pub(super) fn discover_inspire ( &mut self ) -> Flow<bool> {

        Ui::working(0, Mark::Think, "the manager is classifying the project against the training center");

        let model = self.cfg.manager().to_string();
        let answer = self.cfg.paths.configs.join(format!("{CONSULT_FILE}.{MD_EXT}"));
        let target = Path::display(&answer);

        let mut document = Spec::document(&self.cfg.paths.config_file)?;

        let started = Instant::now();
        let prompt = Compose::manager_discover(&self.cfg, &target);

        let Some(( fresh, slug )) = self.consult(&model, &answer, &prompt, Train::parse_type)? else {

            Ui::bang(0, &format!("could not classify the project — staying unbound (`{TOOL} inspire` binds one, or set [project].inspire in {CONFIG_FILE})"));

            return Ok(false);

        };

        let known = Train::available().iter().any(|name| name.eq_ignore_ascii_case(&slug));

        document.project.inspire = slug.clone();
        self.cfg.spec.inspire = slug.clone();
        document.save(&self.cfg.paths.config_file)?;

        if fresh || !known {

            let _ = Train::history(&slug);
            Ui::done(0, Mark::Ok, &format!("history · {slug}"), started);
            Ui::detail("note", "no curated project node fit — a fresh accumulation line starts under history/; add a project node later if this kind recurs");

        }
        else {

            Ui::done(0, Mark::Ok, &format!("project node · {slug}"), started);

        }

        self.cfg.context = Project::discover(&self.cfg.paths, &self.cfg.spec);

        if !Train::trace(&slug).0.is_empty() || !Train::history_reports(&slug).is_empty() {

            Ui::arrow(1, &format!("composed knowledge for {slug} — re-briefing the manager"));

            let addendum = Compose::manager_addendum(&self.cfg);
            self.call("manager", &model, &addendum)?;
            self.check_drain()?;

        }

        Ok(true)

    }

    pub(super) fn consult <T> ( &mut self, model: &str, answer: &StdPath, prompt: &str, parse: impl Fn(&str) -> Option<T> ) -> Flow<Option<T>> {

        File::remove(answer);

        self.deliver("manager", model, "", 0, prompt)?;

        let body = File::read(answer);
        File::remove(answer);

        Ok(parse(&body))

    }

}
