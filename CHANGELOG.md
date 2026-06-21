# Changelog

## v0.1.0 — 2026-06-21

First release. Foundations laid; the orchestration loop runs end to end.

### Added
- Layered Rust crate (`core` → `config` → `app` → `cli`) shipping as both a library and a binary.
- Orchestration cycle: **arch → work → test**, each driven to convergence (`ship it`) under a reviewing **manager** and a green **gate**.
- Roster expansion (`[claude, claude, codex]` → `claude_1 claude_2 codex_1`), each a persistent, independently-briefed agent.
- External CLI workers: `claude` and `codex`, with per-agent sessions persisted to `sessions.json`.
- Commands: `init`, `start`, `stop`, `drain`, `clean` — with process-group isolation, drain checkpoints, and run archival to `.agentx/runs/`.
- Neutral `support` std-lib (fs, parse, proc, text, time, thread, rt, …) and a global app context.
