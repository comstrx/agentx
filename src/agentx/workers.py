from __future__ import annotations

import asyncio
import json
import subprocess


def tool_of(agent: str) -> str:
    return agent.rsplit("_", 1)[0]


async def _run_claude(prompt: str, session_id: str | None, cwd: str) -> tuple[str, str]:
    # NOTE: align option/field names with your installed claude-agent-sdk version.
    from claude_agent_sdk import ClaudeAgentOptions, query

    options = ClaudeAgentOptions(
        cwd=cwd,
        resume=session_id,            # None -> fresh session
        permission_mode="acceptEdits",
    )

    text: list[str] = []
    sid = session_id or ""

    async for message in query(prompt=prompt, options=options):
        got = getattr(message, "session_id", None)
        if got:
            sid = got
        content = getattr(message, "content", None)
        if isinstance(content, str):
            text.append(content)
        elif isinstance(content, list):
            for block in content:
                t = getattr(block, "text", None)
                if t:
                    text.append(t)

    return "".join(text), sid


def run_claude(prompt: str, session_id: str | None, cwd: str) -> tuple[str, str]:
    return asyncio.run(_run_claude(prompt, session_id, cwd))


def run_codex(prompt: str, session_id: str | None, cwd: str) -> tuple[str, str]:
    if session_id:
        cmd = ["codex", "exec", "resume", session_id, "--json", "--cd", cwd, prompt]
    else:
        cmd = ["codex", "exec", "--json", "--cd", cwd, prompt]

    proc = subprocess.run(cmd, capture_output=True, text=True)

    text: list[str] = []
    sid = session_id or ""

    for line in proc.stdout.splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        sid = event.get("session_id", sid)
        if event.get("type") in ("message", "assistant", "agent_message"):
            text.append(str(event.get("text") or event.get("content") or ""))

    return "\n".join(text), sid


def run_worker(agent: str, prompt: str, session_id: str | None, cwd: str) -> tuple[str, str]:
    if tool_of(agent) == "codex":
        return run_codex(prompt, session_id, cwd)
    return run_claude(prompt, session_id, cwd)
