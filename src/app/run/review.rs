use std::path::Path as StdPath;

use crate::config::base::consts::REVIEW_SUFFIX;
use crate::core::error::AppResult;
use crate::core::fs::{Dir, File};
use crate::core::text::Text;
use crate::app::{Compose, Flow, Orchestrator, Ui};

impl Orchestrator {

    pub(super) fn manager_review ( &mut self, phase: &str, task: Option<&StdPath>, round: u32 ) -> Flow<String> {

        let depth = if task.is_some() { 2 } else { 1 };

        self.archive_review(phase)?;

        let review = self.cfg.paths.review_of(phase);
        File::write(&review, "")?;

        self.journey.manager_review = "pending".to_string();
        self.journey.current_agent.clear();
        self.save("review:pending")?;

        Ui::arrow(depth, "manager reviewing the round");

        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_review(&self.cfg, phase, task, round);
        self.deliver("manager", &model, "", depth, &prompt)?;

        self.journey.manager_review = "done".to_string();
        self.save("review:done")?;

        let ( action, _ ) = Text::parse_control(&File::read(&review));

        match action.as_str() {
            "ship"   => Ui::tick(depth, "manager verdict · ship"),
            "revise" => Ui::bang(depth, "manager verdict · revise — sending it back"),
            ""       => Ui::bang(depth, "manager left no verdict — treating as revise"),
            other    => Ui::bang(depth, &format!("manager verdict · {other}")),
        }

        Ok(action)

    }

    fn archive_review ( &self, phase: &str ) -> AppResult<()> {

        let review = self.cfg.paths.review_of(phase);

        if !review.exists() || File::read(&review).trim().is_empty() { return Ok(()); }

        let dir = self.cfg.paths.manager_rounds();
        Dir::ensure(&dir)?;

        let target = dir.join(format!("{}-{phase}{REVIEW_SUFFIX}", Dir::next_stamp(&dir)));

        File::rename(&review, &target)

    }

}
