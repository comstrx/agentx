use std::path::PathBuf;
use std::time::Instant;

use crate::config::base::consts::{BUSY_LABEL, CONSULT_FILE, DOCS_DIR, MD_EXT, PHASES, TOOL};
use crate::config::{Spec, Train};
use crate::core::error::AppError;
use crate::core::fs::{Dir, File, Path};
use crate::app::{Compose, Flow, Halt, Mark, Menu, Orchestrator, Ruling, Status, Ui};

impl Orchestrator {

    pub(super) fn prime_manager ( &mut self ) -> Flow<()> {

        if self.journey.primed { return Ok(()); }

        let model = self.cfg.manager().to_string();
        let fresh = !self.sessions.contains_key("manager");

        Ui::rule("priming · the manager trains and confirms before anyone else");

        if fresh { Ui::working(1, Mark::Study, "training the manager"); }

        let started = Instant::now();
        let brief = Compose::manager_brief(&self.cfg, &self.journey);
        self.prime_turn("manager", &model, &brief)?;

        if fresh { Ui::done(1, Mark::Ok, "manager trained", started); }

        Ui::blank();

        Ui::working(1, Mark::Study, "confirming the manager");

        let started = Instant::now();
        let confirm = Compose::reaffirm(&self.cfg, &model);
        self.call("manager", &model, &confirm)?;
        self.check_drain()?;

        Ui::done(1, Mark::Ok, "manager confirmed", started);

        Ok(())

    }

    pub(super) fn prime_team ( &mut self ) -> Flow<()> {

        if self.journey.primed { return Ok(()); }

        let opened = Instant::now();

        Ui::rule("priming · training the team");

        Ui::arrow(0, "lap 1 — teaching the project, the contracts, and each role");
        Ui::blank();

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);
            let role = Compose::role_label(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if self.dropped.contains(&key) { continue; }

                let fresh = !self.sessions.contains_key(&key);

                if fresh { Ui::working(1, Mark::Study, &format!("training {agent} · {role}")); }

                let started = Instant::now();
                let prompt = Compose::prime(&self.cfg, &self.journey, phase, agent);
                let turn = self.prime_turn(&key, agent, &prompt);

                if self.survive(phase, agent, 1, turn)? && fresh { Ui::done(1, Mark::Ok, &format!("{agent} trained"), started); }

                Ui::blank();

            }

        }

        Ui::blank();
        Ui::arrow(0, "lap 2 — active-recall confirmation of the invariants");
        Ui::blank();

        for phase in PHASES {

            if !self.active(phase) { continue; }

            let roster = self.cfg.roster(phase);
            let role = Compose::role_label(phase);

            for agent in &roster {

                let key = Self::key(phase, agent);

                if self.dropped.contains(&key) { continue; }

                let prompt = Compose::reaffirm(&self.cfg, agent);

                Ui::working(1, Mark::Study, &format!("confirming {agent} · {role}"));

                let started = Instant::now();
                let turn = self.call(&key, agent, &prompt);

                if self.survive(phase, agent, 1, turn)? { Ui::done(1, Mark::Ok, &format!("{agent} confirmed"), started); }

                Ui::blank();

                self.check_drain()?;

            }

        }

        self.journey.primed = true;
        self.save("primed")?;

        Ui::done(0, Mark::Cool, "team primed — opening the pipeline", opened);

        Ok(())

    }

    pub(super) fn prime_turn ( &mut self, key: &str, agent: &str, prompt: &str ) -> Flow<()> {

        if self.sessions.contains_key(key) { return Ok(()); }

        self.call(key, agent, prompt)?;

        self.check_drain()

    }

    pub(super) fn intake ( &mut self ) -> Flow<Option<Ruling>> {

        Ui::rule("intake · the manager verifies the tree, the binding, the gate, and the requirements");

        Dir::ensure(&self.cfg.paths.inbox)?;
        File::remove(&self.cfg.paths.conflict);

        self.check_project()?;
        self.check_inspire()?;
        self.check_gate()?;

        self.check_requirements()

    }

    fn check_inspire ( &mut self ) -> Flow<()> {

        if self.cfg.force { return Ok(()); }

        loop {

            let inspire = self.cfg.spec.inspire.trim().to_string();

            if !inspire.is_empty() && ( !Train::trace(&inspire).0.is_empty() || !Train::history_reports(&inspire).is_empty() || Train::available().iter().any(|name| name.eq_ignore_ascii_case(&inspire)) ) {

                Ui::tick(0, &format!("inspiration verified · {inspire}"));

                return Ok(());

            }

            let started = Instant::now();

            let objection = match inspire.is_empty() {
                true => "- no inspiration is bound — the run inherits no knowledge and its lessons join no kind | [project].inspire is empty | continue unbound, let the manager classify it, or bind one yourself".to_string(),
                false => format!("- inspiration '{inspire}' matches no project node and no history kind | the training center holds no such name | continue as a fresh kind, let the manager re-classify, or rebind it yourself"),
            };

            match self.objection_menu(&objection, started, Menu {
                paused: "intake paused — the inspiration binding needs a ruling".to_string(),
                headline: "the inspiration binding is not usable:".to_string(),
                proceed: "continue — run as-is; the training center lends nothing, lessons start a fresh line".to_string(),
                fix: "fix — the manager studies the project and classifies it into the training center now".to_string(),
                stop: format!("stop — I will bind it myself (`{TOOL} inspire <name|N>`), then `{TOOL} start`"),
                note: format!("stopped — bind the inspiration (`{TOOL} inspire`), then run `{TOOL} start`; intake re-checks every run"),
            })? {
                Ruling::Stop    => return Err(Halt::Paused),
                Ruling::Proceed => return Ok(()),
                Ruling::Fix     => {

                    if !self.discover_inspire()? { return Ok(()); }

                }
            }

        }

    }

    fn check_requirements ( &mut self ) -> Flow<Option<Ruling>> {

        if self.cfg.force { return Ok(None); }

        Ui::blank();
        Ui::working(0, Mark::Think, "the manager is judging the discovered requirements");

        let started = Instant::now();
        let model = self.cfg.manager().to_string();

        File::remove(&self.cfg.paths.conflict);

        let prompt = Compose::manager_requires_check(&self.cfg);
        self.deliver("manager", &model, "", 0, &prompt)?;
        self.check_drain()?;

        let objection = File::read(&self.cfg.paths.conflict).trim().to_string();
        File::remove(&self.cfg.paths.conflict);

        if objection.is_empty() {

            Ui::done(0, Mark::Ok, "requirements verified — no objection", started);

            return Ok(None);

        }

        match self.objection_menu(&objection, started, Menu {
            paused: "intake paused — the manager raised an objection to the requirements".to_string(),
            headline: "the manager sees a breaking conflict:".to_string(),
            proceed: "continue — the manager will settle each conflict as a visible `Assumption:` line in the backlog".to_string(),
            fix: "fix — the manager will resolve the conflicts with his own judgement, recorded as `Decision:` lines".to_string(),
            stop: format!("stop — I will sharpen the requirement file(s), then `{TOOL} start`"),
            note: format!("stopped — sharpen your requirement file(s) ({DOCS_DIR}/ or Requirements.md), then run `{TOOL} start`; intake re-checks every run"),
        })? {
            Ruling::Stop    => Err(Halt::Paused),
            Ruling::Proceed => Ok(Some(Ruling::Proceed)),
            Ruling::Fix     => Ok(Some(Ruling::Fix)),
        }

    }

    pub(super) fn convert ( &mut self, ruling: Option<Ruling> ) -> Flow<()> {

        if self.journey.intake_done { return Ok(()); }

        Ui::rule("requirements · the manager writes the final backlog");

        Dir::ensure(&self.cfg.paths.inbox)?;

        if self.cfg.force && ruling.is_none() { Ui::dot(0, "no-objections run — conflicts become recorded assumptions"); }

        Ui::working(0, Mark::Think, "the manager is converting the sources into the ordered backlog");

        let started = Instant::now();
        let model = self.cfg.manager().to_string();
        let prompt = Compose::manager_convert(&self.cfg, &self.journey, ruling);
        self.deliver("manager", &model, "", 0, &prompt)?;
        self.check_drain()?;

        let authored = Dir::markdown(&self.cfg.paths.inbox);

        if authored.is_empty() {

            let mut copied = 0;

            for source in self.sources() {

                match File::copy(&source, &self.cfg.paths.inbox.join(Path::name_of(&source))) {
                    Ok(_) => copied += 1,
                    Err(error) => Ui::cross(0, &format!("could not copy {} into the backlog — {error}", Path::name_of(&source))),
                }

            }

            if copied == 0 {

                return Err(Halt::Failed(AppError::message("intake produced no backlog — the manager wrote no requirement files and the sources could not be copied; fix the errors above, then run `start` to resume")));

            }

            Ui::bang(0, &format!("manager wrote no files — using {copied} discovered requirement file(s) as-is"));

        }
        else {

            Ui::done(0, Mark::Ok, &format!("{} ordered requirement file(s) ready", authored.len()), started);

        }

        self.journey.intake_done = true;
        self.save("intake")?;

        Ok(())

    }

    fn check_project ( &mut self ) -> Flow<()> {

        if self.cfg.force { return Ok(()); }

        let model = self.cfg.manager().to_string();

        loop {

            Ui::working(0, Mark::Think, "the manager is surveying the project foundation");

            let started = Instant::now();

            File::remove(&self.cfg.paths.conflict);

            let prompt = Compose::manager_project_check(&self.cfg);
            self.deliver("manager", &model, "", 0, &prompt)?;
            self.check_drain()?;

            let objection = File::read(&self.cfg.paths.conflict).trim().to_string();
            File::remove(&self.cfg.paths.conflict);

            if objection.is_empty() {

                Ui::done(0, Mark::Ok, "project foundation verified — no objection", started);
                Ui::blank();

                return Ok(());

            }

            match self.objection_menu(&objection, started, Menu {
                paused: "intake paused — the manager finds no workable project foundation".to_string(),
                headline: "the manager finds the project foundation missing or broken:".to_string(),
                proceed: "continue — build on the tree exactly as it stands; the team works within what exists".to_string(),
                fix: "fix — the manager builds or repairs the foundation himself now, then re-checks it".to_string(),
                stop: format!("stop — I will complete the project myself, then `{TOOL} start`"),
                note: format!("stopped — complete the project foundation, then run `{TOOL} start`; intake re-checks every run"),
            })? {
                Ruling::Stop    => return Err(Halt::Paused),
                Ruling::Proceed => return Ok(()),
                Ruling::Fix     => {

                    Ui::working(0, Mark::Think, "the manager is building the missing foundation");

                    let repair = Instant::now();
                    let fix = Compose::manager_project_fix(&self.cfg);
                    self.deliver("manager", &model, "", 0, &fix)?;
                    self.check_drain()?;

                    Ui::done(0, Mark::Ok, "foundation work done — re-checking", repair);

                }
            }

        }

    }

    fn check_gate ( &mut self ) -> Flow<()> {

        if self.cfg.force { return Ok(()); }

        let model = self.cfg.manager().to_string();

        loop {

            Ui::blank();
            Ui::working(0, Mark::Think, "the manager is verifying the quality gate");

            let started = Instant::now();

            File::remove(&self.cfg.paths.conflict);

            let prompt = Compose::manager_gate_check(&self.cfg);
            self.deliver("manager", &model, "", 0, &prompt)?;
            self.check_drain()?;

            let objection = File::read(&self.cfg.paths.conflict).trim().to_string();
            File::remove(&self.cfg.paths.conflict);

            if objection.is_empty() {

                Ui::done(0, Mark::Ok, "gate verified — no objection", started);

                return Ok(());

            }

            match self.objection_menu(&objection, started, Menu {
                paused: "intake paused — the manager finds the quality gate broken".to_string(),
                headline: "the manager finds the quality gate broken:".to_string(),
                proceed: "continue — keep this gate as it is; a broken gate can block every task later".to_string(),
                fix: "fix — the manager composes the corrected gate himself, saves it, then re-checks it".to_string(),
                stop: format!("stop — I will fix the gate (`{TOOL} gate '<command>'`), then `{TOOL} start`"),
                note: format!("stopped — fix the gate (`{TOOL} gate '<command>'` or [gate].command), then run `{TOOL} start`; intake re-checks every run"),
            })? {
                Ruling::Stop    => return Err(Halt::Paused),
                Ruling::Proceed => return Ok(()),
                Ruling::Fix     => {

                    Ui::working(0, Mark::Think, "the manager is composing the corrected gate");

                    let repair = Instant::now();
                    let answer = self.cfg.paths.configs.join(format!("{CONSULT_FILE}.{MD_EXT}"));
                    let target = Path::display(&answer);
                    let fix = Compose::manager_gate_fix(&self.cfg, &target);

                    match self.consult(&model, &answer, &fix, |body| Train::parse_line(body, "gate:"))? {
                        Some(command) => {

                            let mut document = Spec::document(&self.cfg.paths.config_file)?;
                            document.gate.command = command.clone();
                            document.save(&self.cfg.paths.config_file)?;
                            self.cfg.gate.command = command.clone();

                            Ui::done(0, Mark::Ok, &format!("gate · {command} — re-checking"), repair);

                        }
                        None => Ui::bang(0, "the manager wrote no gate line — unchanged, re-checking"),
                    }

                }
            }

        }

    }

    pub(super) fn sources ( &self ) -> Vec<PathBuf> {

        self.cfg.context.requires.clone()

    }

    fn objection_menu ( &mut self, objection: &str, started: Instant, menu: Menu ) -> Flow<Ruling> {

        Ui::loaded();

        Ui::done(0, Mark::Think, &menu.paused, started);
        Ui::blank();
        Ui::warn(&menu.headline);
        Ui::blank();

        for line in objection.lines() { Ui::item(line); }

        Ui::blank();

        let keys = Ui::keys();

        let options = vec![menu.proceed, menu.fix, menu.stop];

        let picked = Ui::choose(&format!("  the manager awaits your ruling   ({keys} · enter = continue)"), &options, 0)?;

        match picked {
            Some(2) => {

                self.journey.status = Status::Stopped;
                self.save("intake:objection")?;

                Ui::blank();
                Ui::point(0, &menu.note);
                Ui::detail("skip", &format!("`{TOOL} start --force` (or -f) never pauses — conflicts become recorded assumptions"));
                Ui::blank();

                Ok(Ruling::Stop)

            }
            Some(1) => {

                Ui::blank();
                Ui::ok("ruled: fix — the manager takes it from here");
                Ui::blank();

                Ui::loading(BUSY_LABEL);

                Ok(Ruling::Fix)

            }
            _ => {

                Ui::blank();
                Ui::ok("ruled: proceed — the objection is settled for this run");
                Ui::blank();

                Ui::loading(BUSY_LABEL);

                Ok(Ruling::Proceed)

            }
        }

    }

}
