# Dna

Opt-in capability traits — a model gains a whole feature by mounting one unit.

- A capability is a **trait/mixin a model mounts** to gain a feature end to end: files, search, cache, state machine, tenancy, roles, permissions, relations discovery, social engagements. Mounting it is the whole integration — no per-model wiring afterwards.
- **Self-contained:** the trait brings its own scopes, accessors, persistence hooks, derived endpoints, and a small config surface. Everything the feature needs travels with it.
- Built **on top of the support layer** — a dna trait orchestrates support capabilities into model behaviour; it never re-implements native/infra work inline.
- Opt-in surface is **minimal and hard to misuse**: sensible defaults, one obvious way to use it, explicit opt-out (a disabled-list the trait reads) — a model never opts out by editing the engine.
- A large cohesive dna subsystem (a permission ladder, an engagement suite) becomes its **own folder of focused traits composed by one facade trait** — the model still mounts a single name.
- Dna is where cross-cutting model behaviour lives; the base engine is where layer orchestration lives. Query DSL, scoping, derived maps → dna on the model; read pipelines, caching policy, CRUD orchestration → the layer engines. Keep the split clean.
- **A third family completes the trait system — the static utility vocabulary:** small stateless helpers (`Access::permits(...)`, `Auth::actorId()`, `Response::ok()`, exception/tenant/schema/locale utilities) callable from ANY layer — the short words pipelines speak between the engine's big verbs. Utilities never hold state, never touch storage directly, and never grow business rules; they are grammar, dna is capability, engines are orchestration.
- Dna traits declare **capability, never identity**: `HasFiles`, not `IsProduct`. Behaviour keys off what a model can do — the same anti-literal law as everywhere else.
- Every new dna trait is earned by the rule of two: the second model needing the feature is the signal to extract it as dna; the first lives concrete.
