# Mastery

The Rust instincts that separate engineered code from compiling code.

- **Ownership is the design tool:** borrow where you only read (`&str`, `&[T]` in every parameter that can), own where you keep; a `clone()` is a decision with a reason, not a compiler-silencer. `Cow` for the sometimes-owned; `into()`-style generic params where the API should absorb both.
- **Iterators over index loops:** chain `filter/map/find/fold` into one pass; no collect-then-iterate-again; `with_capacity` when the size is known; return `impl Iterator` when the caller might not need a Vec at all.
- **Traits as engines:** shared behaviour lives in default trait methods; implementors declare only their differences — the same shell-and-engine shape as every other layer of the stack. Generics for hot static dispatch, `dyn` for pluggable backends chosen at runtime (worker/driver seams).
- **Types make invalid states unrepresentable:** enums over boolean webs, newtypes over raw `String`/`u64` ids, exhaustive `match` with no wildcard arm on domain enums — adding a variant must break every place that should care.
- **Concurrency the boring way:** channels move data, threads own their world; shared mutability is a last resort behind a narrow guard. `OnceLock` for lazy globals; scoped threads for structured joins; process groups + kill-on-drop for child processes so nothing ever outlives its parent.
- **Allocation awareness on hot paths:** no format!-then-parse round-trips, no intermediate Vecs feeding one consumer, `&'static str` for the fixed vocabulary — measured, not superstition.
- **std-first:** the standard library is enormous; a dependency is added only when it earns its place (async runtime, serde, signal handling) and never for what twenty lines of std can do.
- Release engineering: fat LTO, stripped binaries, panic behaviour chosen deliberately; clippy at `-D warnings` across all targets is the floor, never silenced with `#[allow]` — fix the cause.
