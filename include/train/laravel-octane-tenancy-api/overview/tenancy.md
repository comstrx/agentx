# Tenancy — the spine of the system

Model: **shared database, shared schema, row-level isolation by `tenant_id`** — chosen because the
"business-on-top-of-business" model implies many small tenants. Two layers of defense:
- **`HasTenant` global scope (PRIMARY, fail-closed):** auto-constrains every query to the current
  tenant and auto-fills `tenant_id` on create. This is what the application relies on.
- **Postgres RLS (defense-in-depth):** transaction-local `set_config('app.tenant_id', ?, true)` (never a
  session `SET`), the app connects as a **non-owner** role, tables `FORCE ROW LEVEL SECURITY`. RLS catches
  what code forgets.

## Schema discipline
**UUIDv7** primary keys everywhere. `tenant_id` on every tenant-owned table; composite uniques
`unique(tenant_id, …)`; hot indexes lead with `tenant_id`. Migrations are central, reversible, additive-first.

## Carrying the tenant
Queued jobs carry `tenant_id` in the payload, restore the tenant context at job start, and reset after — a
job is never ambient. Redis cache, lock, and throttle keys are tenant-prefixed. `Context` is the single
source of truth for the active role / tenant / `super` tag.

## Panels & roles (multi-panel)
Each panel is its own API route file; one unified `XxxController` per resource serves all panels, behaviour
differing by the active role. Canonical actors:
- **`super`** — platform owner, **cross-tenant** (`tenant_id = NULL`): reads span/filter by tenant, writes
  target a tenant via a validated `tenant_id` in the body. The ONLY role that disables tenant scope, via an
  **audited `withoutTenancy()`** escape hatch — never an ambient default.
- **`admin`** — the tenant's admin; full control within one tenant.
- **`vendor`** — manages own products and their orders/bookings under a tenant.
- **`affiliate`** — marketing/referral URLs and referred clients.
- **`delivery`** — delivery ops.
- **`client`** — the public storefront buyer surface (NOT an admin panel).
- **`guest`** — public/unauthenticated (browse, register, login).

## RBAC & auth
**Hand-rolled RBAC:** roles + permissions + per-record special permissions; roles are many-to-many
(`user_roles`). The panel route group declares the expected role; middleware verifies membership (multi-role
users supported). **Never trust client-supplied permissions.** Roles carry **`is_super`/`is_supervisor`**:
each admin-side role type (admin/vendor/affiliate/delivery) is its own account hierarchy — one `is_super`
owner creates `is_supervisor`s and members beneath it; `client` is the buyer. Table shapes in
`overview/domain.md` + `contracts/data.md`. Auth is **Sanctum stateless Bearer** everywhere (no SPA cookie mode —
it breaks with custom domains); every token is bound to its tenant (middleware asserts
`token.tenant == domain.tenant`). v1 tenant resolution = subdomain + Bearer token; prefix `/v1`.

## The isolation invariant
No read or write crosses a tenant boundary; no vendor reads another vendor's rows; `super`'s cross-tenant
access is explicit and audited. Enforced by the fail-closed scope + RLS, and verified for every
tenant/vendor-scoped feature. A leak here is the most severe class of defect in this archetype.
