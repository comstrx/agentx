# Hygiene

- **Dead code dies the moment it dies:** no commented-out blocks, no `_old`/`_backup` files, no unreachable branches "kept just in case", no feature flags whose feature shipped. Version control is the archive; the working tree holds only what runs.
- **One concept, one implementation, one home.** Two implementations of the same capability are a defect even when both work — one becomes the canon, the other is deleted, the callers converge. Duplication found during any task is reported and collapsed, never extended.
- **Every magic value has a named home:** directory names, file names, limits, timeouts, retry counts, key patterns, glyphs — all in the constants/config layer. A literal typed twice is a constant not yet extracted; a literal in two files is a drift bug scheduled.
- **Files stay small and single-concern:** a file is one responsibility at one altitude; crossing a few hundred lines is a split signal along its concerns. A "utils" dumping ground is a naming failure — every helper belongs to a named domain.
- **No orphans on the public surface:** every exported item, route, config key, and event has a real consumer today. Speculative surface is deleted, not documented; widening is cheap later, carrying dead promises is not.
- **Warnings are errors.** The analyzer/linter runs at its strictest with zero suppression — no ignore-comments, no baseline growth, no rule disabling to ship. A suppressed warning is a hidden defect wearing a signature; fix the cause.
- **Dependency hygiene:** every dependency earns its place against "the stdlib + what we own can do this", is pinned by the lockfile, sits behind an owned seam if it could ever be swapped, and is removed the release its last consumer goes. Never hand-edit lockfiles.
- **Symmetry is enforced:** parallel features look parallel — same file layout, same naming shape, same layering. A reader who learned one resource has learned them all; an asymmetric feature is either wrong or the new canon, decided explicitly.
- **The boy-scout rule, bounded:** code touched by a task is left cleaner (names, placement, dead branches) — but never reformat or refactor lines the task does not touch.
- **Names carry the meaning prose would:** rename and restructure until the code explains itself — clarity debt is paid at the name, never annotated around.
- Consistency beats preference: the established idiom of the repo outranks any newer taste — a better pattern arrives by migrating the whole family in a deliberate change, never by starting a second style beside the first.
