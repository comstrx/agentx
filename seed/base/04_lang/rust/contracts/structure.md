# Structure

- **Module = folder.** Every module is a directory of small files: `arch.rs` holds the module's own type definitions (structs, enums, aliases — no central mega-types file), `mod.rs` holds wiring only (`mod` + `pub use`, even for a single line), and the implementation lives in `base.rs` when one file suffices or in **concern-split files** when it does not — the same type's `impl` blocks spread across files by responsibility.
- Files stay small; a file crossing a few hundred lines is a split waiting to happen along its concerns.
- **The layer DAG is strict:** foundation crate/modules → configuration → application → binary shell. Dependencies point one way; a lower layer importing an upper one is a build error in spirit even when the compiler allows it.
- **`pub(crate)` is the default visibility.** `pub` is reserved for the real public surface (the library API and what tests consume); everything else stays internal. Widening visibility is a reviewed decision, not a convenience.
- **Consts in one place:** every dir name, file name, magic number, glyph, timeout, and default lives in a single consts module. Legitimate literals outside it: match-pattern vocabulary anchored on those consts, CLI help text, and tests. A string typed twice is a const not yet extracted.
- **Errors are one domain type per layer boundary** with `From` conversions inward; the binary maps them to exit codes and human messages at the very edge.
- The prelude re-exports the working set; modules import the prelude, not twenty paths. Cross-module reach goes through the target module's `mod.rs` surface.
- **Tests are external:** integration tests in `tests/` over the public surface only; `src` stays test-free. What cannot be tested through the surface is either not worth guaranteeing or a sign the surface is missing something.
- No `unsafe` without a documented proof obligation; no build scripts doing work a const or generic could do.
