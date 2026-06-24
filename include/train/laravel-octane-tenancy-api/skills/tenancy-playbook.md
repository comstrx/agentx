# Skill — multi-tenant isolation playbook (single-DB + RLS)

The craft behind `overview/tenancy.md` and `arch.md §9–10`: how to implement fail-closed isolation, and the
exact ways it leaks. Tenant isolation is the #1 correctness concern of this archetype — a leak here is a breach.

## The two layers, implemented
**1. `HasTenant` (PRIMARY, application, fail-closed).** A DNA trait that, on `boot`:
- adds a global scope `where tenant_id = Context::tenantId()`;
- on `creating`, auto-fills `tenant_id` from `Context::tenantId()`;
- **fail-closed:** if there is no tenant in `Context` AND the request is not `super`, the scope resolves to a
  value that matches nothing (or throws) — NEVER returns unscoped rows. "No tenant" must mean "no data", never
  "all data".

```php
protected static function bootHasTenant (): void {

    static::addGlobalScope('tenant', function ( Builder $q ): void {

        if ( Context::isSuper() ) return;

        $q->where($q->getModel()->getTable().'.tenant_id', Context::tenantId() ?? '00000000-0000-0000-0000-000000000000');

    });
    static::creating(function ( Model $m ): void {

        $m->tenant_id ??= Context::tenantId();

    });

}
```

**2. Postgres RLS (defense-in-depth, DB-enforced).** Catches what code forgets:
- App connects as a **non-owner** role (table owners bypass RLS — never connect as owner).
- `ALTER TABLE x ENABLE ROW LEVEL SECURITY; ALTER TABLE x FORCE ROW LEVEL SECURITY;`
- Set the GUC **transaction-local**: `set_config('app.tenant_id', ?, true)` at request/job start. **NEVER**
  `SET app.tenant_id` (session-level) — under Octane the DB connection is reused across requests, so a session
  GUC bleeds into the next tenant. Transaction-local (`true`) is the only safe form.
- **Handle the empty / `super` GUC — `''::uuid` THROWS.** For a `super` / no-tenant request the GUC is unset or
  empty; `current_setting('app.tenant_id', true)` returns `''`, and `''::uuid` is a hard Postgres error that
  breaks *every* query on the table. Apply both fixes: (a) NEVER set the GUC to `''` — set the nil sentinel, or
  leave it unset for `super`; (b) harden the policy to tolerate the empty GUC and admit platform rows:
  `USING (tenant_id IS NULL OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::uuid)`. Without
  this, RLS passes on sqlite/in-memory tests and detonates the first time it runs on Postgres — a silent,
  latent break the gate won't catch if tests don't run on pgsql.

## Carrying the tenant everywhere
- **Jobs:** stamp `tenant_id` into the payload on dispatch; restore `Context` at `handle()` start; reset on
  finish (`support/queue/Tenant`). An unstamped job runs with a null/leftover tenant → corruption.
- **Cache / lock / throttle:** every key is `"{tenant}:{...}"`. An unprefixed key serves tenant A's value to
  tenant B.
- **`super`:** the only cross-tenant role. Reads span tenants; writes target a **validated** `tenant_id` from
  the body. Bypass is an **audited** `withoutTenancy()` helper only — never an ambient default, never left on.

## Reading tenant + platform rows together (the sanctioned cross-cut)
Some `HasTenant` models legitimately must read **their tenant's rows AND the platform's NULL-`tenant_id` rows**
in one query — the RBAC resolver reading a tenant's `permission_settings` plus the `scope=global` (NULL)
defaults, or a query over shared `locations`. The global scope hides the NULL rows, and `withoutTenancy()` is
**`super`-only** (the wrong tool — it would expose ALL tenants). The correct, leak-safe pattern is an explicit
two-sided filter:
```php
Model::withoutGlobalScope('tenant')->where(function ( Builder $q ): void {

    $q->where('tenant_id', Context::tenantId())->orWhereNull('tenant_id');

})->get();
```
It returns ONLY this tenant's rows + shared platform rows, **never** another tenant's. Reserve it for models
that genuinely carry platform/global rows; never use it to dodge the scope on ordinary tenant data.

## The leak vectors — hunt every one
1. **Unscoped query** — `withoutGlobalScope`, raw `DB::`, or query builder on a table whose model lacks the
   trait. Audit any `DB::`/`withoutGlobalScope` usage; in this archetype business code shouldn't touch `DB::`
   at all (`design.md §1`).
2. **Octane state bleed** — tenant held in a static/singleton, not reset → next request inherits it. Tenant
   lives ONLY in `Context`; reset on `RequestTerminated` (see `laravel-octane.md`).
3. **Session-level RLS GUC** on a pooled Octane connection → leaks. Must be transaction-local.
4. **Unstamped job** → wrong/empty tenant at execution.
5. **Unprefixed cache/lock/throttle key** → cross-tenant read.
6. **Cross-tenant relation/FK** — a `belongsTo` that resolves across tenants; validate FKs with a
   tenant-constrained `exists` rule (`Rule::exists(...)->where('tenant_id', Context::tenantId())`).
7. **`super` write without re-scoping** — `withoutTenancy()` left on for a write that should target one tenant.
8. **Existence/validation leak** — an unscoped `exists`/`unique` rule reveals or accepts another tenant's row.

## The probe (the proof — write it per tenant-scoped model)
```php
it('never leaks across tenants', function () {

    $a = Tenant::factory()->create();  $b = Tenant::factory()->create();
    actingForTenant($b, fn () => Product::factory()->create());

    actingForTenant($a, function () {

        expect(Product::count())->toBe(0);

    });

});
```
Five minutes per model; it is the difference between "we believe it's isolated" and "it is". The cross-tenant
probe is the first test that earns its keep (`tools.md`).
