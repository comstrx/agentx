# Errors

- **No `unwrap`, no `panic!`, no `todo!` in production paths.** `expect` survives only on a provable invariant with the proof written into the message; anything reachable by input, environment, or IO returns `Result`.
- **`?` is the flow**, not `match`-and-ignore. An error is either propagated, handled meaningfully, or converted — never swallowed with `let _ =` unless the operation is genuinely best-effort teardown, and then that is a deliberate, reviewed exception.
- **One error enum per layer boundary**, thiserror-shaped: variants carry the context a human needs (the path, the name, the phase), `From` impls convert inward, and the display text **names the fix** — `no interactive terminal for the menu; pass -i <node>` is the bar, `invalid state` is a defect.
- **Fail closed:** on ambiguity choose refusal — a missing permission denies, an unparseable state refuses to guess, a half-written file is removed not trusted. Fallbacks that silently mask a failure are worse than the failure.
- **Fallibility is honest in signatures:** functions that touch IO, parse, or lock return `Result`; infallible helpers stay infallible. A function that can fail but returns a default is lying to every caller.
- Recoverable vs fatal is decided at the edge: the library returns everything; the binary decides what halts, what retries, what degrades — with bounded retries and backoff where retrying is legal.
- Interrupted work leaves valid state: atomic writes (write-then-rename), locks with stale takeover, cleanup on the drop path. A crash at any line must not corrupt what the next run reads.
- Error paths are code, not afterthought: they get the same naming, the same structure, and the same review pressure as the happy path — most production incidents live there.
