# Engine

Building the `HasBaseXxx` engine so concretes stay near-empty — declare, and everything materializes.

- **North star:** a new resource costs ~4 declarations — `migration + Model (use traits) + Repository::fields() + Service (overrides only)` — and gains a full gated, tenant-scoped, cached, N+1-free API for free. Copying logic to add a resource means the engine is missing it: grow the engine, never the concrete.
- **The split:** model DNA carries the query DSL as scopes/macros (`search(...)`, `whereScope(...)`, `getResource()/getStats()`, `remember()`, `hasColumn()`, `getWithRelations()`); base traits carry layer orchestration (controller assembles scopes, service builds the read pipeline + cache, repository does `fields()` + CRUD + boot hooks); the concrete is `fields()` + overrides only.
- **One uniform read pipeline:** index/show/statistics/related/download all build ONE options struct — ids, text, page, limit, sortBy, filter, field, scope, permission, callback — then one `search(...$opts)->getResource($resource, $one)` call. New read features extend the struct once; every resource gains them simultaneously.
- **The scopes thread:** the controller assembles default scopes from `Context` role — non-strict for reads, strict (owner/tenant) for writes — and threads scopes/permissions/callbacks down to the repository's `whereScope()`. One controller, behaviour per role.
- **Relation discovery:** the model declares normal Eloquent relation methods; the engine reflects them once at boot, caches the map in an immutable static (Octane-safe), and derives eager-loading, requested includes, and the `{relation}` nested endpoints from it. `__call`-based nested dispatch resolves against the map and 404s on unknown — dynamic with a printed inventory.
- **Boot hooks in the repository engine:** creating stamps `tenant_id`/UUIDv7/actor; updating guards immutables; deleting cascades what the domain owns — declared per-concrete only when they differ.
- **Cache built in:** read pipeline results wrapped by `remember()` with tenant-scoped keys and tag-based invalidation fired by the write path — a concrete never hand-caches.
- Growth discipline: every engine addition is driven by the second resource that needs it, lands in the right trait, and keeps the concrete's override surface small and named — the engine is powerful because it is curated, not because it is big.
