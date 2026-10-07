# Api

- **Minimal surface: every public item is a promise.** Export the smallest set that serves the use cases; everything else stays private. Widening later is free, narrowing is a breaking change — default to private.
- **Hard to misuse:** make invalid states unrepresentable in the types; required data in constructors, optional data in builders/defaults; one obvious way per task. If the compiler/type-checker can catch a misuse, it must.
- **Errors are values**, typed and matchable. A library never prints, never exits, never logs to the consumer's streams uninvited. It returns what happened; the application decides what to do about it.
- **Zero side effects at import/load** and zero required configuration for the happy path: sensible defaults everywhere, escape hatches beneath. First call in the README works verbatim.
- **No global mutable state.** Instances own their state; anything shared is explicit in the signature. Two instances in one process must never interfere.
- **Stability honesty:** semantic versioning enforced — any observable behaviour a consumer can depend on is part of the contract, including error variants and defaults. Deprecate with a path, remove on majors only.
- Dependencies are the consumer's burden: keep them minimal, boring, and optional where heavy (feature-gated). A utility library dragging a runtime is a defect.
- Naming symmetry across the surface: `open/close`, `read/write`, `encode/decode`, consistent parameter order, consistent verb vocabulary. The API should be guessable after learning one corner of it.
- Blocking and non-blocking variants are explicit, never secretly one wrapping the other on a hot path; thread-safety is documented per type, not assumed.
