from __future__ import annotations
from . import compose
from .config import CONVERGENCE, Config
from .gate import run_gate
from .io_files import all_shipped, drain_requested, dump_prompt, harvest, harvest_file
from .io_files import make_run_dir, next_stamp, parse_control, snapshot_one, task_snapshot
from .workers import run_worker

class DrainSignal ( Exception ):

    pass

class Orchestrator:

    def __init__ ( self, config: Config, cwd: str ):

        self.cfg = config
        self.cwd = cwd
        self.sessions: dict[str, str] = {}
        self.primed: set[str] = set()
        self.blocked: list[str] = []
        self.frozen = sorted(task_snapshot(config.paths.tasks))

    def _check_drain ( self ) -> None:

        if drain_requested(self.cfg.paths.drain):
            raise DrainSignal()

    def _call ( self, key: str, agent: str, prompt: str ) -> str:

        dump_prompt(self.cfg.paths.prompts, key, prompt)

        session = self.sessions.get(key)
        text, session = run_worker(agent, prompt, session, self.cwd)
        self.sessions[key] = session

        return text

    def _worker_turn ( self, step: str, agent: str, prompt: str ) -> None:

        self._call(f"{step}-{agent}", agent, prompt)
        snapshot_one(self.cfg.paths.reports_of(step) / f"{agent}.md", self.cfg.paths.rounds_of(step))
        self._check_drain()

    def _is_init ( self, step: str, agent: str ) -> bool:

        key = f"{step}-{agent}"
        init = key not in self.primed
        self.primed.add(key)

        return init

    def _shipped ( self, step: str ) -> bool:

        return all_shipped(self.cfg.paths.reports_of(step), self.cfg.spec.roster(step), CONVERGENCE)

    def brief_manager ( self ) -> None:

        self._call("manager", self.cfg.manager(), compose.manager_brief(self.cfg))

    def run_arch ( self, has_review: bool ) -> bool:

        paths = self.cfg.paths
        rounds = 0

        while rounds < self.cfg.spec.max_rounds:

            rounds += 1
            review = has_review and rounds == 1

            for agent in self.cfg.spec.roster("arch"):

                init = self._is_init("arch", agent)
                critique = any(paths.reports_of("arch").glob("*.md"))
                prompt = compose.architect(self.cfg, agent, init, critique, review, self.frozen)

                self._worker_turn("arch", agent, prompt)

            if self._shipped("arch"):
                return True

        return self._shipped("arch")

    def run_work ( self, has_review: bool ) -> bool:

        paths = self.cfg.paths
        rounds = 0
        gate_ok = True

        while rounds < self.cfg.spec.max_rounds:

            rounds += 1
            review = has_review and rounds == 1

            for agent in self.cfg.spec.roster("work"):

                init = self._is_init("work", agent)
                prompt = compose.executor(self.cfg, agent, init, not gate_ok, review)

                self._worker_turn("work", agent, prompt)
                gate_ok, _ = run_gate(self.cfg.spec.gate_cmd, self.cwd, paths.gate_log)

                fixes = 0

                while not gate_ok and fixes < self.cfg.spec.max_fixes:

                    fixes += 1
                    prompt = compose.executor(self.cfg, agent, False, True, False)

                    self._worker_turn("work", agent, prompt)
                    gate_ok, _ = run_gate(self.cfg.spec.gate_cmd, self.cwd, paths.gate_log)

            if gate_ok and self._shipped("work"):
                return True

        return gate_ok and self._shipped("work")

    def run_test ( self, has_review: bool ) -> bool:

        rounds = 0

        while rounds < self.cfg.spec.max_rounds:

            rounds += 1
            review = has_review and rounds == 1

            for agent in self.cfg.spec.roster("test"):

                init = self._is_init("test", agent)
                prompt = compose.verifier(self.cfg, agent, init, review)

                self._worker_turn("test", agent, prompt)

            if self._shipped("test"):
                return True

        return self._shipped("test")

    def run_phase ( self, step: str ) -> None:

        paths = self.cfg.paths
        runners = {"arch": self.run_arch, "work": self.run_work, "test": self.run_test}
        run_workers = runners[step]

        if paths.review.exists():
            paths.review.unlink()

        converged = run_workers(False)
        rounds = 0

        while True:

            self._call("manager", self.cfg.manager(), compose.manager_review(self.cfg, step, rounds + 1))
            self._check_drain()
            action, _ = parse_control(paths.control)

            if action == "ship":
                break

            rounds += 1

            if rounds >= self.cfg.spec.max_rounds:
                print(f"[agentx] {step}: max_rounds reached")
                break

            converged = run_workers(True)

        if not converged:
            self.blocked.append(step)
            print(f"[agentx] {step}: NOT converged - open issues recorded")

        if paths.review.exists():
            paths.review.unlink()

    def write_decision ( self ) -> None:

        paths = self.cfg.paths
        paths.history.mkdir(parents=True, exist_ok=True)
        decision = paths.history / f"{next_stamp(paths.history)}.md"

        self._call("manager", self.cfg.manager(), compose.manager_decision(self.cfg, decision))

    def archive_run ( self ) -> None:

        paths = self.cfg.paths
        run_dir = make_run_dir(paths.runs)

        harvest(paths.requires, run_dir, "requires")
        harvest(paths.tasks, run_dir, "tasks")
        harvest(paths.reports, run_dir, "reports")
        harvest(paths.rounds, run_dir, "rounds")
        harvest(paths.tests, run_dir, "tests")
        harvest(paths.probes, run_dir, "probes")
        harvest(paths.prompts, run_dir, "prompts")

        for single in ( paths.review, paths.control, paths.gate_log ):
            harvest_file(single, run_dir)
