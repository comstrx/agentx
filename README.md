# ✨ Agentx

Hierarchical multi-agent orchestrator.

    top manager
    ├── arch_supervisor  -> architects   (read requires/, write tasks/)
    └── exec_supervisor  -> executors    (implement tasks/, gated)

- **LangGraph** drives the hierarchy + durable state (sqlite checkpoint).
- **Claude Agent SDK** runs each worker with a persisted session (resume = full context
  across rounds on the same task; fresh session per new task; persistent session for the manager).
- **codex** workers run headless via `codex exec` / `codex exec resume`.
- The **gate** is deterministic (`GATE_CMD`) — not self-reported by the model.
- All durable work products live under `agents/` (contracts, overview, requires, tasks,
  reports, decisions, history) exactly like the bash version, so everything stays auditable.

## Run

    uv sync
    cp .env.example .env        # edit GATE_CMD + roster
    uv run orchestrate

Workers are stateless processes; context lives in the SDK session store and is restored
via `resume`. No tmux, no mtime polling.

## Notes

- Agent SDK option/field names (`resume`, `session_id`) may need aligning to your installed
  SDK version — see `workers.py`.
- `claude -p --resume` (raw CLI flag) has open context-loss bugs; this skeleton uses the
  **SDK** programmatically, which restores full session context.
