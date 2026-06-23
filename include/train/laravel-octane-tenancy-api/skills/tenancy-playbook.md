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
- Policy: `USING (tenant_id = current_setting('app.tenant_id', true)::uuid)` (the `true` = don't error if unset).
- Set the GUC **transaction-local**: `set_config('app.tenant_id', ?, true)` at request/job start. **NEVER**
  `SET app.tenant_id` (session-level) — under Octane the DB connection is reused across requests, so a session
  GUC bleeds into the next tenant. Transaction-local (`true`) is the only safe form.

## Carrying the tenant everywhere
- **Jobs:** stamp `tenant_id` into the payload on dispatch; restore `Context` at `handle()` start; reset on
  finish (`support/queue/Tenant`). An unstamped job runs with a null/leftover tenant → corruption.
- **Cache / lock / throttle:** every key is `"{tenant}:{...}"`. An unprefixed key serves tenant A's value to
  tenant B.
- **`super`:** the only cross-tenant role. Reads span tenants; writes target a **validated** `tenant_id` from
  the body. Bypass is an **audited** `withoutTenancy()` helper only — never an ambient default, never left on.

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
