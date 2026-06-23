# Requirements — baseline for this archetype

Default requirements every project of this archetype must satisfy. Seeded by the tool owner as a FLOOR;
a project's own `requires/` (or `agents/requires/`) is appended after this and wins on conflict. The team
builds against the union of both. These are requirement-shaped (what every feature must achieve); the HOW
and the enforceable law live in `contracts/`.

## Every feature must
- Be exposed only through the versioned `/v1` API — no new server-rendered surface; inputs validated in a
  Form Request; outputs through an API Resource with an explicit shape, never a raw model.
- Enforce tenant isolation end to end (fail-closed `HasTenant` scope + Postgres RLS) and ship a
  cross-tenant probe proving no leak, for every tenant-scoped table or endpoint.
- Authorize server-side via the hand-rolled RBAC (`has:<permission>`) — never trust a client-supplied role.
- Arrive with schema via a reversible, additive-first migration: UUIDv7 keys, `tenant_id`, composite uniques
  leading with `tenant_id`.
- Leave the gate green — Larastan at max + `declare(strict_types=1)`, no suppression — with the cross-tenant
  probe (and any tests that have joined the gate) passing.
- Live at the right layer: business logic as a thin pipeline over the Support DSL and the trait engine, with
  no native PHP/Laravel inlined into a high layer.

## Non-functional floor
- List endpoints are keyset-paginated and free of N+1.
- Money is integer minor units on a double-entry ledger — never a float.
- Heavy or external work is queued, idempotent, tenant-stamped, and retry-safe.
- No per-request state in statics or singletons (Octane-safe); the role/tenant/user tag lives in `Context`.
- No secret reaches code, image, or logs.
