from __future__ import annotations

from .config import Config
from .gate import run_gate
from .io_files import all_shipped, archive, clear_files, next_stamp, parse_control, snapshot_one, sorted_md
from .prompts import arch_prompt, exec_prompt, manager_decision, manager_init, manager_review
from .workers import run_worker

class Orchestrator:

    def __init__ ( self, config: Config, cwd: str ):

        self.cfg = config
        self.cwd = cwd
        self.sessions: dict[str, str] = {}
        self.primed: set[str] = set()

    def _call ( self, agent: str, prompt: str ) -> str:

        session = self.sessions.get(agent)
        text, session = run_worker(agent, prompt, session, self.cwd)
        self.sessions[agent] = session

        return text

    def _worker_turn ( self, agent: str, prompt: str, reports_dir, history_dir ) -> None:

        self._call(agent, prompt)
        snapshot_one(reports_dir / f"{agent}.md", history_dir)

    def brief_manager ( self ) -> None:

        self._call(self.cfg.manager, manager_init())

    def run_arch_workers ( self, has_review: bool ) -> None:

        paths = self.cfg.paths
        rounds = 0

        while rounds < self.cfg.max_rounds:

            rounds += 1
            review = has_review and rounds == 1

            for agent in self.cfg.architects:

                init = agent not in self.primed
                self.primed.add(agent)

                self._worker_turn(agent, arch_prompt(agent, init, review), paths.reports_requires, paths.history_reports / "requires")

            if all_shipped(paths.reports_requires, self.cfg.architects):
                return

    def run_exec_workers ( self, has_review: bool ) -> None:

        paths = self.cfg.paths
        rounds = 0
        gate_ok = True

        while rounds < self.cfg.max_rounds:

            rounds += 1
            review = has_review and rounds == 1

            for agent in self.cfg.executors:

                init = agent not in self.primed
                self.primed.add(agent)

                self._worker_turn(agent, exec_prompt(agent, init, not gate_ok, review), paths.reports_tasks, paths.history_reports / "tasks")
                gate_ok, _ = run_gate(self.cfg.gate_cmd, self.cwd, paths.gate_log)

                fixes = 0

                while not gate_ok and fixes < self.cfg.max_rounds:

                    fixes += 1

                    self._worker_turn(agent, exec_prompt(agent, False, True, False), paths.reports_tasks, paths.history_reports / "tasks")
                    gate_ok, _ = run_gate(self.cfg.gate_cmd, self.cwd, paths.gate_log)

            if gate_ok and all_shipped(paths.reports_tasks, self.cfg.executors):
                return

    def _critic_loop ( self, mode: str, run_workers ) -> None:

        paths = self.cfg.paths
        rounds = 0

        while True:

            rounds += 1

            self._call(self.cfg.manager, manager_review(mode, rounds, self.cfg.max_rounds))
            action, _ = parse_control(paths.control)

            if action == "ship":
                return

            if rounds >= self.cfg.max_rounds:
                print(f"[agentx] {mode}: max_rounds reached, accepting as-is")
                return

            run_workers(has_review=True)

    def run_phase ( self, mode: str ) -> None:

        paths = self.cfg.paths

        if paths.review.exists():
            paths.review.unlink()

        if mode == "arch":
            self.run_arch_workers(has_review=False)
            self._critic_loop("arch", self.run_arch_workers)
        else:
            self.run_exec_workers(has_review=False)
            self._critic_loop("exec", self.run_exec_workers)

        if paths.review.exists():
            paths.review.unlink()

    def write_decision ( self ) -> None:

        paths = self.cfg.paths
        decision = paths.decisions / f"{next_stamp(paths.decisions)}.md"

        self._call(self.cfg.manager, manager_decision(str(decision)))

    def archive_run ( self ) -> None:

        paths = self.cfg.paths

        archive(sorted_md(paths.requires), paths.history_requires)
        archive(sorted_md(paths.tasks), paths.history_tasks)

        clear_files(paths.reports_requires, paths.reports_tasks)
        clear_files(paths.history_reports / "requires", paths.history_reports / "tasks")

        for extra in ( paths.review, paths.control, paths.gate_log ):

            if extra.exists():
                extra.unlink()
