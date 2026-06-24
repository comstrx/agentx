use std::collections::HashMap;
use std::path::Path as StdPath;
use std::time::Duration;

use crate::config::consts::{AGENT_RETRIES, PHASES};
use crate::config::{Config, Train};
use crate::core::error::{AppError, AppResult};
use crate::core::support::fs::{Dir, File, Path};
use crate::core::support::parse::Json;
use crate::core::support::proc::Proc;
use crate::core::support::text::Text;
use crate::core::worker::{Fault, Worker};
use super::arch::{Compose, Flow, Halt, Journey, Orchestrator, Phase, Project, Status, Ui};

impl<'a> Orchestrator<'a> {

    pub(crate) fn new ( cfg: &'a Config ) -> Self {

        let journey = Journey::load(&cfg.paths.state);
        let sessions = Self::load_sessions(&cfg.paths.sessions);

        Self { cfg, journey, sessions }

    }

    pub(crate) fn run ( &mut self ) -> AppResult<()> {

        self.boot();

        match self.cycle() {
            Ok(()) => {

                self.report_outcome();

                Ok(())

            }
            Err(Halt::Drained) => {

                Ui::blank();
                Ui::ok(&format!("drained at phase {:?}, round {} — state saved; `start` resumes", self.journey.phase, self.journey.current_round));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Stopped) => {

                Ui::blank();
                Ui::warn(&format!("interrupted — stopped at phase {:?}, round {}; state saved, `start` resumes", self.journey.phase, self.journey.current_round));
                Ui::blank();

                Ok(())

            }
            Err(Halt::Failed(error)) => {

                if Proc::aborted() {

                    if !self.journey.journey_id.is_empty() {

                        self.journey.status = Status::Stopped;
                        let _ = self.journey.save(&self.cfg.paths.state);

                    }

                    Ui::blank();
                    Ui::warn(&format!("interrupted — stopped at phase {:?}, round {}; state saved, `start` resumes", self.journey.phase, self.journey.current_round));
                    Ui::blank();

                    return Ok(());

                }

                if !self.journey.journey_id.is_empty() {

                    self.journey.status = Status::Failed;
                    let _ = self.journey.save(&self.cfg.paths.state);

                }

                Err(error)

            }
        }

    }

    fn boot ( &self ) {

        let kind = if self.cfg.spec.inspire.is_empty() { "(unbound)".to_string() } else { self.cfg.spec.inspire.clone() };
        let gate = if self.cfg.gate.command.is_empty() { "(none — gate skipped)".to_string() } else { self.cfg.gate.command.clone() };

        Ui::blank();
        Ui::title("agentx · orchestration server");
        Ui::blank();
        Ui::step("starting up — readying the team and the pipeline");
        Ui::field("project", &Path::display(&self.cfg.root));
        Ui::field("type", &kind);
        Ui::field("gate", &gate);
        Ui::field("team", "");
        Ui::role("manager", self.cfg.manager());
        Ui::role("architects", &self.cfg.roster("requires").join(" "));
        Ui::role("executors", &self.cfg.roster("tasks").join(" "));
        Ui::role("testers", &self.cfg.roster("tests").join(" "));
        Ui::blank();

    }

    fn cycle ( &mut self ) -> Flow<()> {

        if self.journey.is_resumable() {

            Ui::blank();
            Ui::step(&format!("resuming journey {} at phase {:?}", self.journey.journey_id, self.journey.phase));
            self.journey.status = Status::Running;
            self.save("resume")?;

        }
        else {

            if self.cfg.context.requires.is_empty() {

                return Err(AppError::message("nothing to do: add a requirement (Requirements.md at the root, or agents/requires/) then run start").into());

            }

            self.start_fresh()?;

        }

        self.prime()?;

        self.intake()?;

        if self.journey.phase <= Phase::Requires { self.phase_requires()?; }

        if self.journey.phase <= Phase::Tasks { self.phase_tasks()?; }

        if self.journey.phase <= Phase::Tests { self.phase_tests()?; }

        if self.journey.phase <= Phase::Finalize { self.finalize()?; }

        self.journey.phase = Phase::Completed;
        self.journey.status = Status::Completed;
        self.save("completed")?;

        Ok(())

    }

    fn start_fresh ( &mut self ) -> Flow<()> {

        self.journey = Journey::fresh();
        self.sessions.clear();

        Project::reset_runtime(&self.cfg.paths);

        self.save("start")?;

        Ok(())

    }

    fn prime ( &mut self ) -> Flow<()> {

        if self.journey.primed { return Ok(()); }

        let model = self.cfg.manager().to_string();

        Ui::rule("priming · training the team before any work");

        Ui::arrow(0, "lap 1 — teaching the project, the contracts, and each role");

        if !self.sessions.contains_key("manager") { Ui::arrow(1, "training the manager"); }

        let brief = Compose::manager_brief(self.cfg);
        self.prime_turn("manager", &model, &brief)?;

        for phase in PHASES {

            let roster = self.cfg.roster(phase);
            let role = Self::role_of(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if !self.sessions.contains_key(&key) { Ui::arrow(1, &format!("training {agent} · {role}")); }

                let prompt = Compose::prime(self.cfg, phase, agent);
                self.prime_turn(&key, agent, &prompt)?;

            }

        }

        Ui::arrow(0, "lap 2 — active-recall confirmation of the invariants");

        Ui::arrow(1, "confirming the manager");
        let confirm = Compose::reaffirm("manager");
        self.call("manager", &model, &confirm)?;
        self.check_drain()?;

        for phase in PHASES {

            let roster = self.cfg.roster(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);
                let prompt = Compose::reaffirm(agent);

                Ui::arrow(1, &format!("confirming {agent}"));
                self.call(&key, agent, &prompt)?;
                self.check_drain()?;

            }

        }

        self.journey.primed = true;
        self.save("primed")?;

        Ui::tick(0, "team primed — opening the pipeline");

        Ok(())

    }

    fn prime_turn ( &mut self, key: &str, agent: &str, prompt: &str ) -> Flow<()> {

        if self.sessions.contains_key(key) { return Ok(()); }

        self.call(key, agent, prompt)?;

        self.check_drain()

    }

    fn intake ( &mut self ) -> Flow<()> {

        if self.journey.intake_done { return Ok(()); }

        Ui::rule("intake · the manager turns the discovered requirements into an ordered backlog");

        Dir::ensure(&self.cfg.paths.inbox)?;

        Ui::arrow(0, "the manager is analysing the discovered requirements");

        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_intake(self.cfg, &self.journey);
        self.deliver("manager", &model, "", 0, &prompt)?;
        self.check_drain()?;

        let authored = Dir::markdown(&self.cfg.paths.inbox);

        if authored.is_empty() {

            for source in self.sources() {

                let _ = File::copy(&source, &self.cfg.paths.inbox.join(Path::name_of(&source)));

            }

            Ui::bang(0, "manager wrote no files — using the discovered requirements as-is");

        }
        else {

            Ui::tick(0, &format!("{} ordered requirement file(s) ready", authored.len()));

        }

        self.journey.intake_done = true;
        self.save("intake")?;

        Ok(())

    }

    fn sources ( &self ) -> Vec<std::path::PathBuf> {

        let kind = &self.cfg.spec.inspire;

        let mut sources = if kind.is_empty() { Vec::new() } else { Train::requires(kind) };
        sources.extend(self.cfg.context.requires.iter().cloned());

        sources

    }

    fn phase_requires ( &mut self ) -> Flow<()> {

        self.enter(Phase::Requires)?;

        Ui::rule("phase 1/3 · requires · architects shape the task plan");

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

    fn phase_tasks ( &mut self ) -> Flow<()> {

        self.enter(Phase::Tasks)?;

        Ui::rule("phase 2/3 · tasks · executors build the plan, one task at a time");

        let tasks = Dir::markdown(&self.cfg.paths.tasks);

        if tasks.is_empty() {

            Ui::cross(1, "no tasks — the architects produced an empty plan");
            self.mark_blocked("tasks: architects produced no tasks");
            self.save("tasks:empty")?;

            return Ok(());

        }

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

    fn phase_tests ( &mut self ) -> Flow<()> {

        self.enter(Phase::Tests)?;

        Ui::rule("phase 3/3 · tests · verifiers attack the finished result");

        let roster = self.cfg.roster("tests");
        let shipped = self.run_phase("tests", &roster, None)?;

        if shipped {

            Ui::tick(1, "tests passed — the result holds");

        }
        else {

            Ui::cross(1, "tests blocked — unresolved defects remain");
            self.mark_blocked("tests");
            self.save("tests:blocked")?;

        }

        Ok(())

    }

    fn finalize ( &mut self ) -> Flow<()> {

        self.enter(Phase::Finalize)?;

        Ui::rule("finalize · the manager records the journey");
        Ui::arrow(0, "manager writing the journey record");

        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_summary(self.cfg);
        self.deliver("manager", &model, "", 0, &prompt)?;

        let summary = File::read(&self.cfg.paths.summary());
        let has_summary = !summary.trim().is_empty();

        let record = if has_summary {

            summary

        }
        else {

            let blocked = if self.journey.blocked.is_empty() { "none".to_string() } else { self.journey.blocked.join(", ") };
            format!("Journey {}\n\nThe manager wrote no summary for this run. Open issues: {blocked}.\n", self.journey.journey_id)

        };

        if has_summary {

            Ui::tick(0, &format!("summary written → {}", Path::relative_one(&self.cfg.paths.summary(), &self.cfg.root)));

        }
        else {

            Ui::bang(0, "manager wrote no summary — recorded a stub instead");

        }

        let title = Text::first_line(&record);
        let slug = match Text::slug(title) {
            value if value.is_empty() => "journey".to_string(),
            value => value,
        };

        let kind = &self.cfg.spec.inspire;

        if kind.is_empty() {

            Ui::dot(0, "project is unbound — no training-center record written");

        }
        else {

            let _ = Train::learn(kind, &slug, &record);
            Ui::tick(0, &format!("recorded to the training center · {kind}"));

        }

        Ok(())

    }

    fn run_phase ( &mut self, phase: &str, roster: &[String], task: Option<&StdPath> ) -> Flow<bool> {

        let depth = if task.is_some() { 2 } else { 1 };
        let max = self.cfg.spec.max_rounds;
        let mut round = 0;

        self.journey.current_round = 0;

        loop {

            round += 1;
            self.journey.current_round = round;
            self.save("round")?;

            Ui::beat(depth, &format!("round {round}/{max}"));

            let gate_ok = self.roster_pass(phase, roster, task, round > 1)?;
            self.check_drain()?;

            let action = self.manager_review(phase, task, round)?;
            self.check_drain()?;

            if action == "ship" && ( phase != "tasks" || gate_ok ) { return Ok(true); }

            if round >= max {

                Ui::bang(depth, &format!("reached the round limit ({max}) without convergence"));
                return Ok(false);

            }

        }

    }

    fn roster_pass ( &mut self, phase: &str, roster: &[String], task: Option<&StdPath>, has_review: bool ) -> Flow<bool> {

        let mut gate_ok = true;

        let depth = if task.is_some() { 3 } else { 2 };
        let verb = Self::verb_of(phase);
        let max_fixes = self.cfg.spec.max_fixes;

        let pending: Vec<String> = roster.iter().filter(|name| !self.journey.agents_done.iter().any(|done| done == *name)).cloned().collect();
        self.journey.agents_pending = pending;
        self.save("round:agents")?;

        for agent in roster {

            if self.journey.agents_done.iter().any(|done| done == agent) { continue; }

            Ui::arrow(depth, &format!("{agent} · {verb}"));

            let prompt = self.build_prompt(phase, agent, task, !gate_ok, has_review);
            self.worker_turn(phase, agent, task, &prompt)?;

            Ui::tick(depth, &format!("{agent} wrote {}", Path::relative_one(&self.cfg.paths.report_of(phase, agent), &self.cfg.root)));

            if phase == "tasks" {

                gate_ok = self.gate_step(depth)?;
                let mut fixes = 0;

                while !gate_ok && fixes < max_fixes {

                    fixes += 1;
                    Ui::bang(depth, &format!("gate red — {agent} repairing (fix {fixes}/{max_fixes})"));

                    let repair = self.build_prompt(phase, agent, task, true, false);

                    self.worker_turn(phase, agent, task, &repair)?;
                    gate_ok = self.gate_step(depth)?;

                }

            }

            self.journey.agents_done.push(agent.clone());
            self.journey.agents_pending.retain(|pending| pending != agent);
            self.save("agent:done")?;

        }

        self.journey.agents_done.clear();
        self.save("round:done")?;

        Ok(gate_ok)

    }

    fn build_prompt ( &self, phase: &str, agent: &str, task: Option<&StdPath>, gate_failed: bool, has_review: bool ) -> String {

        match phase {
            "requires" => Compose::architect(self.cfg, agent, has_review),
            "tasks" => Compose::executor(self.cfg, agent, task.unwrap_or_else(|| StdPath::new("")), gate_failed, has_review),
            "tests" => Compose::verifier(self.cfg, agent, has_review),
            _ => String::new(),
        }

    }

    fn worker_turn ( &mut self, phase: &str, agent: &str, task: Option<&StdPath>, prompt: &str ) -> Flow<()> {

        self.journey.current_agent = agent.to_string();
        self.archive_report(phase, agent, task)?;

        let depth = if task.is_some() { 3 } else { 2 };
        let key = Self::key(phase, agent);
        self.deliver(&key, agent, phase, depth, prompt)?;

        self.journey.last_action = format!("{phase}_report_written");
        self.save("report")?;

        self.check_drain()

    }

    fn archive_report ( &self, phase: &str, agent: &str, task: Option<&StdPath> ) -> AppResult<()> {

        let report = self.cfg.paths.report_of(phase, agent);

        if !report.exists() { return Ok(()); }

        let dir = match task {
            Some(path) => self.cfg.paths.task_rounds(&Path::stem_of(path)),
            None => self.cfg.paths.rounds_of(phase),
        };

        Dir::ensure(&dir)?;

        let target = dir.join(format!("{agent}-{}.md", Dir::next_sequence(&dir)));
        File::rename(&report, &target)

    }

    fn dispatch ( &mut self, key: &str, agent: &str, prompt: &str ) -> AppResult<()> {

        self.dump_prompt(key, prompt)?;

        let ( model, effort ) = self.cfg.engine(agent);

        let mut runner = Worker::new(agent);
        runner.cwd(&self.cfg.root).timeout(self.cfg.agent.timeout).pid_file(&self.cfg.paths.active);
        runner.engine(&model, &effort);

        if let Some(session) = self.sessions.get(key) && !session.is_empty() {

            runner.set_session(session);

        }

        let session = runner.start(prompt)?;

        if !session.is_empty() {

            self.sessions.insert(key.to_string(), session);
            self.persist_sessions()?;

        }

        Ok(())

    }

    fn call ( &mut self, key: &str, agent: &str, prompt: &str ) -> Flow<()> {

        let mut tries = 0;

        loop {

            self.check_drain()?;

            let error = match self.dispatch(key, agent, prompt) {
                Ok(()) => return Ok(()),
                Err(error) => error,
            };

            self.check_drain()?;

            tries += 1;

            if Worker::fault(&error) == Fault::Transient && tries <= AGENT_RETRIES {

                Ui::bang(1, &format!("{agent} — hiccup ({}); retrying {tries}/{AGENT_RETRIES}", Self::reason(&error)));
                self.backoff(tries)?;

                continue;

            }

            Ui::cross(1, &format!("{agent} failed to initialise — {}; stopping", Self::reason(&error)));

            return Err(Halt::Failed(error));

        }

    }

    fn deliver ( &mut self, key: &str, agent: &str, phase: &str, depth: usize, prompt: &str ) -> Flow<()> {

        let mut tries = 0;
        let mut reprimed = false;

        loop {

            self.check_drain()?;

            let error = match self.dispatch(key, agent, prompt) {
                Ok(()) => return Ok(()),
                Err(error) => error,
            };

            self.check_drain()?;

            match Worker::fault(&error) {
                Fault::Exhausted => {

                    Ui::cross(depth, &format!("{agent} — provider usage/quota exhausted; stopping, `start` resumes once it resets"));

                    return Err(Halt::Failed(error));

                }
                Fault::Fatal => {

                    Ui::cross(depth, &format!("{agent} — unrecoverable: {}; stopping", Self::reason(&error)));

                    return Err(Halt::Failed(error));

                }
                Fault::Transient => {

                    tries += 1;

                    if tries <= AGENT_RETRIES {

                        Ui::bang(depth, &format!("{agent} — hiccup ({}); retrying {tries}/{AGENT_RETRIES}", Self::reason(&error)));
                        self.backoff(tries)?;

                        continue;

                    }

                }
                Fault::Session => {}
            }

            if reprimed {

                Ui::cross(depth, &format!("{agent} — did not recover after re-priming; stopping ({})", Self::reason(&error)));

                return Err(Halt::Failed(error));

            }

            Ui::bang(depth, &format!("{agent} — recovering on a fresh session: re-train, confirm, then resume"));

            self.reprime(key, agent, phase)?;

            reprimed = true;
            tries = 0;

        }

    }

    fn reprime ( &mut self, key: &str, agent: &str, phase: &str ) -> Flow<()> {

        self.sessions.remove(key);
        self.persist_sessions()?;

        let brief = if key == "manager" {

            Compose::manager_brief(self.cfg)

        }
        else {

            Compose::prime(self.cfg, phase, agent)

        };

        self.call(key, agent, &brief)?;

        let label = if key == "manager" { "manager" } else { agent };

        let confirm = Compose::reaffirm(label);

        self.call(key, agent, &confirm)

    }

    fn backoff ( &self, attempt: u32 ) -> Flow<()> {

        let seconds = ( 1u64 << attempt.min(4) ).min(15);

        for _ in 0..seconds {

            if Proc::aborted() { return Err(Halt::Stopped); }

            std::thread::sleep(Duration::from_secs(1));

        }

        Ok(())

    }

    fn reason ( error: &AppError ) -> String {

        let raw = match error {
            AppError::Timeout { secs, .. } => return format!("timed out after {secs}s"),
            AppError::Command { stderr, .. } if !stderr.trim().is_empty() => Text::first_line(stderr).to_string(),
            other => other.to_string(),
        };

        raw.chars().take(90).collect()

    }

    fn run_gate ( &self ) -> Flow<bool> {

        let gate = &self.cfg.gate;
        let log = &self.cfg.paths.gate_log;

        if gate.command.is_empty() {

            File::write(log, "no gate command set; gate skipped")?;
            return Ok(true);

        }

        let output = Proc::shell_in(&gate.command, &self.cfg.root, gate.timeout)?;
        File::write(log, &format!("{}{}", output.stdout, output.stderr))?;

        Ok(output.code == 0 && !output.timed_out)

    }

    fn manager_review ( &mut self, phase: &str, task: Option<&StdPath>, round: u32 ) -> Flow<String> {

        let depth = if task.is_some() { 2 } else { 1 };

        let review = self.cfg.paths.review_of(phase);
        File::write(&review, "")?;

        self.journey.manager_review = "pending".to_string();
        self.save("review:pending")?;

        Ui::arrow(depth, "manager reviewing the round");

        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_review(self.cfg, phase, task, round);
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

    fn gate_step ( &self, depth: usize ) -> Flow<bool> {

        if self.cfg.gate.command.is_empty() {

            self.run_gate()?;
            Ui::dot(depth, "gate skipped — no gate command set");

            return Ok(true);

        }

        Ui::arrow(depth, &format!("running gate · {}", self.cfg.gate.command));

        let ok = self.run_gate()?;

        if ok { Ui::tick(depth, "gate green"); }
        else  { Ui::cross(depth, "gate red — see .agentx/gate.log"); }

        Ok(ok)

    }

    fn enter ( &mut self, phase: Phase ) -> Flow<()> {

        if self.journey.phase != phase {

            self.journey.phase = phase;
            self.journey.current_round = 0;
            self.journey.current_task.clear();
            self.journey.agents_done.clear();

        }

        self.save("phase")?;

        Ok(())

    }

    fn mark_blocked ( &mut self, name: &str ) {

        if !self.journey.blocked.iter().any(|item| item == name) {

            self.journey.blocked.push(name.to_string());

        }

    }

    fn check_drain ( &mut self ) -> Flow<()> {

        if Proc::aborted() {

            self.journey.status = Status::Stopped;
            let _ = self.journey.save(&self.cfg.paths.state);

            return Err(Halt::Stopped);

        }

        if self.cfg.paths.drain.exists() {

            self.journey.status = Status::Drained;
            let _ = self.journey.save(&self.cfg.paths.state);

            return Err(Halt::Drained);

        }

        Ok(())

    }

    fn save ( &mut self, action: &str ) -> Flow<()> {

        self.journey.last_action = action.to_string();
        self.journey.save(&self.cfg.paths.state)?;

        Ok(())

    }

    fn dump_prompt ( &self, label: &str, prompt: &str ) -> AppResult<()> {

        let dir = &self.cfg.paths.prompts;
        Dir::ensure(dir)?;

        let target = dir.join(format!("{}-{label}.md", Dir::next_sequence(dir)));
        File::write(&target, prompt)

    }

    fn persist_sessions ( &self ) -> AppResult<()> {

        let body = Json::to_string_pretty(&self.sessions)?;
        File::write_atomic(&self.cfg.paths.sessions, &body)

    }

    fn report_outcome ( &self ) {

        Ui::blank();

        if self.journey.blocked.is_empty() {

            Ui::ok("journey complete — all phases shipped");

        }
        else {

            Ui::warn(&format!("journey complete with open issues: {}", self.journey.blocked.join(", ")));

        }

        Ui::blank();

    }

    fn load_sessions ( path: &StdPath ) -> HashMap<String, String> {

        let body = File::read(path);

        if body.trim().is_empty() { return HashMap::new(); }

        let map: HashMap<String, String> = Json::parse(&body).unwrap_or_default();

        map.into_iter().filter(|( _, id )| !id.is_empty()).collect()

    }

    fn key ( phase: &str, agent: &str ) -> String {

        format!("{phase}-{agent}")

    }

    fn role_of ( phase: &str ) -> &'static str {

        match phase {
            "requires" => "architect",
            "tasks"    => "executor",
            "tests"    => "tester",
            _          => "agent",
        }

    }

    fn verb_of ( phase: &str ) -> &'static str {

        match phase {
            "requires" => "architecting the task plan",
            "tasks"    => "implementing the task",
            "tests"    => "verifying the result",
            _          => "working",
        }

    }

}
