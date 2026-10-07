# Eloquent

Eloquent driven at engineering depth — the ORM as an engine component, not a query toy.

- **N+1 is dead structurally:** auto eager-load from the discovered relation map + requested includes; `Model::preventLazyLoading(!app()->isProduction())` as the tripwire; `withCount`/`withSum` aggregates over loading collections to count them. Fixing N+1 per endpoint is treating the symptom.
- **Scopes are the query vocabulary:** small named scopes compose the DSL (`->tenant()`, `->visible($role)`, `->search($text)`); a raw `where` chain repeated twice is a scope not yet named. Global scopes for invariants (tenancy) — fail-closed, with an audited escape hatch only.
- **Casts centralize shape:** the `casts()` method maps enums, money (minor-unit integer casts), dates, encrypted fields, and value objects; `$fillable` + `Repository::fields()` are the write gates — `$guarded = []` is forbidden.
- **Chunk everything bulk:** `chunkById`/`lazyById` for walks (keyset under the hood — plain `chunk` with mutations skips rows), `cursor()` for read streams, `upsert()` for bulk writes, explicit `select()` on wide tables in hot lists.
- **Transactions at the service level:** wrap the pipeline, not individual repository calls; `DB::transaction` with retry rounds for deadlock-prone sections; `lockForUpdate()` on contended rows (wallets, stock); after-commit dispatch for events/jobs that must see committed truth (`afterCommit`).
- **Relations honestly modelled:** morphs are UUID morphs with a closed morph-map (no class-name strings in the DB); pivot models when a pivot grows fields; `whereHas` over collection filtering; `whereBelongsTo` for readability.
- **Attribute accessors stay cheap:** derived-on-read attributes never hide queries; anything touching a relation or IO belongs in the pipeline, not an accessor.
- Measure with the query log in dev and slow-query log in production: every list endpoint's query count is constant regardless of rows — that is the definition of done for reads.
