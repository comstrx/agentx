# Growth

- A new feature costs **declarations, not plumbing**: declare the entity, its fields, its capabilities — and the engine materializes the repeated 95% (CRUD, routing, permissions, caching, scoping). If adding a feature means copying an existing feature's code, the engine is missing a piece: grow the engine, never the copy.
- The engine grows **downward and inward**: new shared behaviour lands in the engine layer; new primitives land in the foundation. Over the life of the project the foundation gets richer while the domain stays the same size.
- **Extension points are earned, never pre-built:** zero consumers = not built; one = concrete where it is needed; two = pushed down into the engine/foundation, named well. No plugin systems, no config surface, no "future-proof" indirection nothing uses today.
- Keep every module **extract-ready** without extracting it: capability behind an interface, wiring at the composition root, communication through public surfaces and events. Extraction is a deliberate, evidence-driven act — never a default.
- Migrations and schema growth are additive-first and reversible; destructive changes never ride in the same release as code that still reads the old shape.
- Growth is measured at the call site: if the tenth feature is not dramatically cheaper than the first, the architecture is failing — stop and fix the engine before feature eleven.
