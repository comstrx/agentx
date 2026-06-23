# Overview — Laravel backend-API, multi-tenant multi-panel SaaS

The default shape for this archetype. It is **opinionated and singular**: ONE stack, ONE architecture, ONE
design, ONE path — no alternatives, no "or". The project's own `agents/` files override on conflict, and the
repo's `composer.lock`/`config/*` are always ground truth for versions and shape.

A **headless API only**: Laravel ships a versioned JSON API and nothing else — no Blade, no server-rendered
pages, no bundled SPA. Frontends, mobile, and integrations are separate consumers of the contract.

A **multi-tenant, multi-panel, multi-business SaaS** — infrastructure for operating and monetizing
businesses, not a store builder. One tenant gets a full branded ecosystem and owns its **entire chain**
(vendors, affiliates, clients, deliveries, products, orders, wallets) — all tenant-scoped, zero cross-tenant
leakage.

Read the companions first — they are the law of the land:
- `arch.md` — the layers, the Base engine, the Support std-lib, `Context`. **Where code lives.**
- `tenancy.md` — shared-DB `tenant_id` + RLS, the panels/roles, the isolation invariant.
- `domain.md` — the canonical data model: roles hierarchy, the `products` polymorphic spine, prices, locations, domains/zones.
- `stack.md` — the imposed technologies, the gate, the build-it-ourselves policy.
- `pattern.md` — schema-as-truth, DNA traits, swappable drivers, the per-resource DX, the "magic".

## Hard invariants (never violated, in any state)
- **Tenant isolation is absolute.** `HasTenant` fail-closed global scope is the PRIMARY isolation;
  Postgres **RLS** is defense-in-depth. No query, job, event, or cache key crosses a tenant. `super` is the
  only cross-tenant role, and only via an audited `withoutTenancy()`.
- **The API contract is the boundary.** Versioned `/v1`, a uniform `success`/`fail` JSON envelope, Resources
  only — never a raw model. A breaking change is a new version, never an edit to the live one.
- **Stateless Bearer (Sanctum); every token is bound to its tenant.**
- **UUIDv7 primary keys.** `tenant_id` on every tenant-owned table; composite uniques `unique(tenant_id, …)`.
- **Money is integer minor units on a double-entry ledger. Time is UTC. Closed sets are PHP enums.**
- **The magic lives in the engine.** Real logic is in the `HasBaseXxx` traits + DNA traits; concrete classes
  are near-empty declaration. Schema + naming are the single source of truth — routes, permissions, eager-loads,
  and write-fields are DERIVED, not hand-written. (A model still DECLARES its relation methods, casts, and
  fillable; "derived" means the engine builds the wiring — eager-loading, includes, nested-relation routes —
  FROM those declarations; you never hand-wire them per endpoint.)
- **Build it ourselves.** Validate untrusted input at the boundary; secrets never in code or logs; heavy work
  is queued, idempotent, and tenant-stamped.

## Definition of done
Acceptance criteria pass · tenant isolation holds (fail-closed scope + RLS, cross-tenant verified) · the
contract is honoured (envelope + `/v1` Resource shape) · the gate is green (Larastan max + `strict_types`) ·
no N+1 on any list endpoint · schema arrived via a reversible, additive-first migration · no secret leaked.
