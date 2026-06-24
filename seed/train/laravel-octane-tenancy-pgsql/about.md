# Laravel Octane backend-API SaaS — multi-tenant, multi-vendor, multi-product-type

A headless, API-only Laravel backend for operating and monetizing many businesses on one platform:
multi-tenant (shared-DB + Postgres RLS), multi-panel, multi-vendor, with a single polymorphic catalog that
sells many product types. Opinionated and singular — one stack, one architecture, one path.

## Stack
- **PHP ^8.5 · Laravel ^13** — headless JSON API only (no Blade/Inertia/SPA), versioned `/v1`, uniform
  success/fail envelope, Resources only.
- **Octane + FrankenPHP** — long-lived workers; Octane-safe code is mandatory (per-request state in `Context`).
- **PostgreSQL 16** — jsonb, partial/GIN indexes, **Row-Level Security**, UUIDv7, real FKs/constraints.
- **Redis 8** — logical DBs split by concern (cache / queue / horizon / reverb / session / rate_limit / lock).
- **Sanctum** stateless Bearer (tenant-bound) · **Horizon** queues · **Reverb** realtime broadcasting.
- **Mailgun HTTP API** mail (always queued, never SMTP) · **s3 / MinIO** storage · **Cloudflare / Vercel** domains.
- Gate = **Larastan max + `declare(strict_types=1)`**; hand-written style, **no formatter**.
- **Build-it-ourselves** — hand-rolled RBAC/cache/search/throttle/idempotency/payments; no general-purpose
  third-party domain libraries (no Spatie permission/data/query-builder). First-party Laravel only.

## Architecture (the "magic")
Repository pattern, layered: `support → traits → bases → repository → service → controller → request → route`.
A `HasBaseXxx` engine + opt-in DNA traits + a Support std-lib carry all repeated work; concrete classes are
near-empty declaration; relations, routes, permissions, and fields are **DERIVED** from schema + naming.
Business code is a pure pipeline over the DSL — zero native PHP/Laravel inline. Two flagship DNA subsystems:
a **multi-level RBAC ladder** (global → tenant → entity → item, where `super` overrides and locks lower levels)
and a **polymorphic social/engagement engine** (like / view / comment / review / favorite / report / files /
notifications as one-line model traits over morph tables).

## Best fit when the project is
- A **multi-tenant SaaS / marketplace / booking or commerce platform** with multiple panels (super, admin,
  vendor, affiliate, delivery, client), vendors, clients, orders, wallets, and strict per-tenant isolation.
- An **API-only backend** consumed by separate web / mobile / partner clients (no server-rendered UI).
- Selling **many kinds of things from one catalog** — hotels, rooms, real-estate, courses, services, products,
  tours, visas, events — plus content (`blogs`) — the single polymorphic `products` table is the spine.
- Needing **fine-grained multi-level permissions, social engagement, money/ledger, realtime, and high
  throughput on Octane** at scale.

## Not the fit when
- A server-rendered monolith (Blade / Inertia / Livewire), a single-tenant app, a non-PHP stack, or a simple
  CRUD app that does not need the engine, the tenancy model, or the multi-panel / multi-vendor shape.
