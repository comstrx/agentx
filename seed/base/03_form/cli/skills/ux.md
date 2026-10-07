# Ux

Terminal experience that feels engineered — capability-aware, readable at a glance, honest.

- **Capability detection is three independent axes**, each self-guarding: screen control (real TTY, not dumb), color (respect `NO_COLOR`, `TERM=dumb`, force flags), glyphs (TTY + UTF-8 locale). Icons degrade to ASCII tokens, spinners to plain lines, menus to errors naming the flag — the tool never draws garbage at a pipe or a dumb terminal.
- **Errors name the fix:** what failed, why, and the exact flag/command that resolves it — `no interactive terminal for the menu; pass -i <node>` beats `invalid input`. An error message is a UI surface, write it like one.
- **Progress honestly:** spinner + elapsed time on TTY only; when piped, clean sequential log lines. Long operations show what is running now, not a frozen cursor. Durations printed on completion — numbers build trust.
- **Layout breathes:** blank lines separate logical beats, never stack; section rules and titles are consistent tokens; aligned columns for key/value output. The log should scan like a well-typeset page.
- **Full-screen views** (watch/status loops) use the alternate screen buffer, restore cursor and screen on exit, and die cleanly on Ctrl+C. Never flood scrollback with repaints.
- **Menus:** arrow keys + vim fallback, visible key hints, Enter selects the safe default, Escape cancels to it. Any interactive element inside a running operation suspends its spinner first — two writers on one line is corruption.
- **Help is a product page:** one-line about per command, usage with breathing, examples for the non-obvious. Ship shell completions and a man page; they are generated, not maintained.
- Respect the user's terminal state: restore everything you change (cursor, colors, screen), even on panic paths.
