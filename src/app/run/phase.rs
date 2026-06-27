use std::path::Path as StdPath;

use crate::config::base::consts::MD_EXT;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Flow, Orchestrator, Phase, Ui};

impl Orchestrator {

    pub(super) fn phase_requires ( &mut self ) -> Flow<()> {

        self.enter(Phase::Requires)?;

        Ui::rule("phase · requires · architects shape the task plan");

        let roster = self.cfg.roster("requires");
        let shipped = self.run_phase("requires", &roster, None)?;

        if shipped {

            Ui::tick(1, "requires shipped — the task plan is ready");

        }
        else {

            Ui::cross(1, "requires blocked — no converged plan");
            self.mark_blocked("requires");
            self.save("requires:blocked")?;

        }

        Ok(())

    }

    pub(super) fn phase_tasks ( &mut self ) -> Flow<()> {

        self.enter(Phase::Tasks)?;

        Ui::rule("phase · tasks · executors build the plan, one task at a time");

        let dir = self.cfg.paths.tasks.clone();

        if Dir::markdown(&dir).is_empty() {

            Ui::cross(1, "no tasks — the architects produced an empty plan");
            self.mark_blocked("tasks: architects produced no tasks");
            self.save("tasks:empty")?;

            return Ok(());

        }

        self.run_tasks(&dir)

    }

    pub(super) fn run_tasks ( &mut self, dir: &StdPath ) -> Flow<()> {

        let tasks = Dir::markdown(dir);
        let total = tasks.len();
        let roster = self.cfg.roster("tasks");

        for ( index, task ) in tasks.iter().enumerate() {

            let name = Path::name_of(task);

            if self.journey.task_status.get(&name).map(String::as_str) == Some("shipped") {

                Ui::dot(1, &format!("task {}/{total} · {name} — already shipped, skipping", index + 1));
                continue;

            }

            Ui::beat(1, &format!("task {}/{total} · {name}", index + 1));

            self.journey.current_task = name.clone();
            self.journey.current_round = 0;
            self.journey.agents_done.clear();
            self.journey.task_status.insert(name.clone(), "executing".to_string());
            self.save("task:start")?;

            let shipped = self.run_phase("tasks", &roster, Some(task.as_path()))?;
            let outcome = if shipped { "shipped" } else { "blocked" };

            self.journey.task_status.insert(name.clone(), outcome.to_string());

            if shipped {

                Ui::tick(1, &format!("task {name} shipped"));

            }
            else {

                Ui::cross(1, &format!("task {name} blocked"));
                self.mark_blocked(&name);

            }

            Ui::blank();

            self.journey.current_task.clear();
            self.save("task:done")?;

        }

        Ok(())

    }

    pub(super) fn phase_audit ( &mut self ) -> Flow<()> {

        if self.journey.phase > Phase::Audit { return Ok(()); }

        self.enter(Phase::Audit)?;

        if !self.cfg.option.audits {

            Ui::dot(0, "skipping the audit phase — [option].audits is off");

            return Ok(());

        }

        Ui::rule("phase · audit · auditors hunt integration & quality defects across the whole system");

        let max = self.cfg.agent.max_audits;
        let roster = self.cfg.roster("audits");
        let audit_dir = self.cfg.paths.audit.clone();

        loop {

            if self.journey.current_audit >= max {

                Ui::bang(0, &format!("reached the audit limit ({max}) — proceeding with the system as it stands"));
                break;

            }

            self.journey.current_audit += 1;
            self.journey.current_round = 0;
            let round = self.journey.current_audit;
            self.save("audit:round")?;

            Ui::beat(1, &format!("audit round {round}/{max}"));

            Dir::clear_files(&audit_dir);

            self.run_phase("audits", &roster, None)?;

            let pending = Dir::markdown(&audit_dir);

            if pending.is_empty() {

                Ui::tick(1, "audit clean — no remediation needed, the system holds");
                break;

            }

            Ui::arrow(1, &format!("audit raised {} remediation task(s) — handing them to the executors", pending.len()));

            for task in &pending { self.journey.task_status.remove(&Path::name_of(task)); }

            self.run_tasks(&audit_dir)?;
            self.promote_audit_tasks()?;

        }

        Ok(())

    }

    fn promote_audit_tasks ( &self ) -> Flow<()> {

        let mut next = Dir::markdown(&self.cfg.paths.tasks).iter().filter_map(|task| Self::task_number(task)).max().unwrap_or(0);

        for task in Dir::markdown(&self.cfg.paths.audit) {

            next += 1;
            let stem = Self::clean_name(&task);
            let dest = self.cfg.paths.tasks.join(format!("{next:04}-{stem}.{MD_EXT}"));

            File::rename(&task, &dest)?;

        }

        Ok(())

    }

    fn task_number ( path: &StdPath ) -> Option<u32> {

        Path::stem_of(path).split_once('-').and_then(|( head, _ )| head.parse().ok())

    }

    pub(super) fn phase_produce ( &mut self, name: &str, phase: Phase, enabled: bool ) -> Flow<()> {

        if self.journey.phase > phase { return Ok(()); }

        self.enter(phase)?;

        if !enabled {

            Ui::dot(0, &format!("skipping the {name} phase — [option].{name} is off"));

            return Ok(());

        }

        Ui::rule(&Self::produce_banner(name));

        let roster = self.cfg.roster(name);
        let shipped = self.run_phase(name, &roster, None)?;

        if shipped {

            Ui::tick(1, &format!("{name} phase shipped — the result holds"));

        }
        else {

            Ui::cross(1, &format!("{name} phase blocked — unresolved issues remain"));
            self.mark_blocked(name);
            self.save(&format!("{name}:blocked"))?;

        }

        Ok(())

    }

    fn produce_banner ( name: &str ) -> String {

        let what = match name {
            "tests"    => "testers exercise and attack the executed work",
            "benches"  => "benchers measure the executed work",
            "examples" => "examplers write runnable examples for the executed work",
            "fuzzes"   => "fuzzers drive randomized, adversarial inputs at the executed work",
            _          => "the team exercises the executed work",
        };

        format!("phase · {name} · {what}")

    }

    pub(super) fn clean_name ( path: &StdPath ) -> String {

        let stem = Path::stem_of(path);

        match stem.split_once('-') {
            Some(( head, rest )) if !rest.is_empty() && head.chars().all(|c| c.is_ascii_digit()) => rest.to_string(),
            _ => stem,
        }

    }

}
