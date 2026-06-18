from __future__ import annotations

import asyncio
import json
import subprocess

def tool_of ( agent: str ) -> str:

    return agent.rsplit("_", 1)[0]

async def _run_claude ( prompt: str, session_id: str | None, cwd: str ) -> tuple[str, str]:

    from claude_agent_sdk import ClaudeAgentOptions, query

    options = ClaudeAgentOptions(
        cwd=cwd,
        resume=session_id,
        permission_mode="acceptEdits",
    )

    parts: list[str] = []
    session = session_id or ""

    async for message in query(prompt=prompt, options=options):

        current = getattr(message, "session_id", None)

        if current:
            session = current

        content = getattr(message, "content", None)

        if isinstance(content, str):
            parts.append(content)

        elif isinstance(content, list):

            for block in content:

                text = getattr(block, "text", None)

                if text:
                    parts.append(text)

    return "".join(parts), session

def run_claude ( prompt: str, session_id: str | None, cwd: str ) -> tuple[str, str]:

    return asyncio.run(_run_claude(prompt, session_id, cwd))

def run_codex ( prompt: str, session_id: str | None, cwd: str ) -> tuple[str, str]:

    if session_id:
        command = ["codex", "exec", "resume", session_id, "--json", prompt]
    else:
        command = ["codex", "exec", "--json", prompt]

    result = subprocess.run(command, cwd=cwd, capture_output=True, text=True)

    parts: list[str] = []
    session = session_id or ""

    for line in result.stdout.splitlines():

        try:
            event = json.loads(line)
        except ValueError:
            continue

        if event.get("type") == "thread.started":
            session = event.get("thread_id", session)

        if event.get("type") == "item.completed":

            item = event.get("item", {})

            if item.get("type") == "agent_message":
                parts.append(str(item.get("text", "")))

    return "\n".join(parts), session

def run_worker ( agent: str, prompt: str, session_id: str | None, cwd: str ) -> tuple[str, str]:

    if tool_of(agent) == "codex":
        return run_codex(prompt, session_id, cwd)

    return run_claude(prompt, session_id, cwd)
