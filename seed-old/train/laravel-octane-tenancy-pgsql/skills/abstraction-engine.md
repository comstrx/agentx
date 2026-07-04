# Skill — the abstraction engine (declare → the engine materializes)

How to build the `HasBaseXxx` engine so concrete classes are near-empty: declare `fields()` + traits, and
CRUD / search / stats / relations / files / permissions all materialize. The craft behind `design.md §1–4`
and `arch.md §3` — Octane-safe, UUIDv7, fail-closed, `route:cache`-safe.

**North star (the measurable bar):** a new resource should cost ~4 declarations — `migration + Model (use
traits) + Repository::fields() + Service (overrides only)` — and gain a full gated, tenant-scoped, cached,
N+1-free API for free. If adding a resource makes you COPY logic, the engine is missing it: grow the engine,
never the concrete. Every line that lives high (controller/service) is a line that could not be derived — keep
that set as small as the problem truly allows.

## The split (where the power lives)
- **Model DNA** (`app/Traits/Dna/*`) carries the **query DSL** as scopes/macros: `search(...)`,
  `whereScope(...)`, `getResource()/getStats()/getItems()`, `remember()/successRemember()`, `hasColumn()`,
  `getWithRelations()`, `isOneRelation()`.
- **Base engine** (`app/Traits/Bases/HasBaseXxx`) carries **layer orchestration**: controller assembles
  scopes, service builds the read pipeline + cache, repository does `fields()` + CRUD + boot hooks.
- **Concrete class** = `fields()` + overrides only. A concrete that grows real logic is a smell — push it down.

## One uniform read pipeline (every read funnels through it)
index / show / statistics / download / related all build ONE options struct, then one search call:
```php
protected function buildParams ( array $params = [], array $scopes = [], array $permissions = [], array $callbacks = [] ): array {

    return [
        'id'         => parse($params['ids'] ?? null),
        'text'       => Cast::string($params['search'] ?? null),
        'page'       => Cast::int($params['page'] ?? null),
        'limit'      => Cast::int($params['limit'] ?? null),
        'sortBy'     => Cast::string($params['sort'] ?? null),
        'filter'     => parse($params['filters'] ?? []),
        'field'      => parse($params['fields'] ?? []),
        'scope'      => $scopes,
        'permission' => $permissions,
        'callback'   => $callbacks,
    ];

}
```
then `$query->search(...$opts)->getResource($resource, $one)`. The request-derived filter DSL (`column@op`
with `>= in notin between like`, `sort@asc`/oldest/newest, aggregates) is **`hasColumn`-guarded**: declare a
column and you get filtering/sorting on it for free.

## The scopes thread (one controller, behaviour per role)
The controller assembles **default scopes from `Context` role** — non-strict for reads, strict for writes
(owner/tenant constraints) — and threads `scopes`/`permissions`/`callbacks` down to the repository's
`whereScope($scope)`:
```php
protected function defaultScopes ( bool $strict = false ): array {

    return match ( Context::role() ) {
        'admin'  => [],
        'vendor' => $strict ? ['active' => true, 'vendor_id' => Context::userId()] : ['active' => true],
        default  => $strict ? ['active' => true, 'user_id' => Context::userId()] : ['active' => true, 'allow' => true],
    };

}
```
**Read role/tenant/user from `Context` — NEVER request-bound global helpers** (`user_id()`/`user_role()` are
the Octane-unsafe anti-pattern; see `laravel-octane.md`).

## `fields()` + boot hooks (declarative lifecycle)
The repository declares the write-shape in `fields()`; lifecycle extension via optional
`createBoot`/`updatedBoot`/`deletedBoot`/`booted`, dispatched by `method_exists`. Concretes add only the hook
that differs — the engine runs the rest.

## Cache woven into reads
`successRemember(tag, key: [...$opts, 'type' => 'resource'], callback)` keys the cache by the **query shape**;
every write calls `deleteCache($tag)` — tag/index invalidation, never a full flush. All keys are
tenant-prefixed.

## Nested relations via `__call` (fail-closed, `route:cache`-safe)
`relatedRooms` / `showRelatedRoom` resolve the relation from the method name and dispatch to a real controller
action against the model's **derived** relations; an unknown relation **fails closed (404)**. No route
closures (cacheable), and **no empty `catch`** — validate the relation explicitly, log a genuine error, 404
the unknown.

## The upgrades over a naive engine (do these, they are correctness)
- **Octane:** per-request state (tenant/role/user) lives ONLY in `Context`. Immutable boot-derived metadata
  (schema, discovered relations, fillables, config, env) is cached in **statics** — that's the Octane win;
  never put request/tenant state in a static (`laravel-octane.md`).
- **UUIDv7 `string`** ids in every signature — never `int $id`.
- **Fail-closed, never silent:** a skipped bad filter is logged, not swallowed by an empty `catch`; tenant
  scope and relation dispatch fail closed.
- **Business code stays pipeline** — the engine and DSL hold the native/infra work; a service reads as intent
  (`design.md §1`).
