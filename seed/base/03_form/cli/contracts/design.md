# Design

- **Library first, binary second.** Every capability is a callable function in the library; the executable is a thin shell that parses arguments, calls the library, and renders the result. The same crate/package serves both consumers — a feature that only works through the binary is a design defect.
- **stdout is the product, stderr is the narration.** Machine-consumable output (data, generated text, `--show` values) goes to stdout, pipe-pure with zero decoration; progress, warnings, and diagnostics go to stderr. Piping the tool must never mix the two.
- **Exit codes are API:** 0 = success, non-zero = distinct failure classes, stable across releases. A script must be able to branch on them.
- **The interactivity law:** menus and prompts only on a real interactive terminal. Headless with a required value missing → immediate clear error **naming the flag** that supplies it; headless with an optional value → silent default. Escape/cancel in any menu selects the default; destructive actions confirm on TTY and demand an explicit `-y` headless.
- **Flags over positional guessing:** every option has a long form; short forms for the frequent few. Precedence is explicit and documented: flags → config file → environment → defaults.
- **Deterministic:** same input, same output. No hidden network calls, no phoning home, no time-dependent behaviour that is not asked for.
- **Fail closed and resumable:** a crashed or interrupted run leaves valid state; locks are atomic with stale-takeover; children die with the parent (process groups) — no zombies, no orphaned work.
- Config lives in one obvious file with one obvious schema; the tool never scatters dotfiles. `init` scaffolds, is idempotent, and never overwrites user edits.
- Startup is instant: defer heavy work past argument parsing so `--help` and `--version` never pay for it.
