# Stack — imposed technologies, the gate, build-it-ourselves

Opinionated, current, **single path**. Version policy: target the latest stable major; the floors below are
minimums; `composer.json`/`composer.lock` is ground truth — match it, never fight it.

## Core
- **PHP `^8.5`** — typed everywhere: enums, `readonly`, constructor promotion, first-class callable. No
  untyped arrays where a value object/DTO fits.
- **Laravel `^13`** — slim skeleton (`bootstrap/app.php`), API-only install.
- **Octane + FrankenPHP** — the runtime. ALL code must be Octane-safe (no per-request state in
  statics/singletons) and `route:cache`-safe (no route closures).
- **PostgreSQL 16** — `jsonb`, partial & GIN indexes, RLS, UUIDv7, real FKs and DB-level constraints.
- **Redis 8** — logical DBs split by concern: cache / queue / horizon / reverb / session / rate_limit / lock.

## API, realtime, auth
- **Sanctum** stateless Bearer tokens; prefix `/v1`; uniform `success`/`fail` envelope; Resources only.
- **Reverb** for realtime (broadcast immediately, `ShouldBroadcastNow`, presence + shared tenant channels).
- **Horizon** supervises the Redis queues.

## Mail & storage
- **Mail is ALWAYS queued** (`ShouldQueue`), sent via the **Mailgun HTTP API** transport — **never SMTP**.
- One `Storage` abstraction, the **`s3` driver everywhere** (AWS in prod, MinIO in dev — identical driver,
  only endpoint/creds differ). Object keys are tenant-namespaced, private by default, signed `TemporaryUrl`.

## The gate (the floor)
- **Larastan / PHPStan at max level** + `declare(strict_types=1)` in every file. Green = zero errors, no
  `@phpstan-ignore`, no baseline growth, no casts/`any`-style widening — fix the root cause.
- **NO formatter.** The hand-written style IS the standard (see `contracts/`); a formatter would fight it.
- Tests join the gate as the spine stabilizes; until then the gate is Larastan + `strict_types`.

## Build it ourselves
Hand-roll the domain DSL (RBAC, cache, search, throttle, idempotency, the payments pipeline) in `app/Support`
+ traits. **No general-purpose third-party domain libraries** (no Spatie permission/data/query-builder).
First-party Laravel only (Sanctum / Horizon / Reverb / Octane). The single "don't roll your own" exception:
**cryptography and money rounding/ledger primitives** — use core / battle-tested code there.

## Swappable subsystems (add one file = a new backend)
`cache · lock · throttle · queue · event(broker) · storage · search · ai · payments · domains` — each is a
`Driver` interface + concrete driver(s) + a manager in `index.php`. Defaults: events → Redis/Horizon
(outbox-ready), search → light/DB driver (ES/OpenSearch later), **AI → Claude / Anthropic latest**, payments →
`StripeDriver`, **domains/DNS → Cloudflare + Vercel** (buy / transfer / subdomain provisioning, queued &
idempotent).

## Ops & contract
- **Horizon** is served headless (web, **no app auth**) — protected by the proxy/nginx layer, never a public
  route (this is a headless API; the proxy owns dashboard auth).
- **No OpenAPI / doc generators** — the abstraction engine's dynamic surface doesn't fit them. The live API
  contract is the **`routes/collections/` mirror** (`contracts/arch.md §8`): every API change updates its
  collection entry in the same change, so the frontend sees it immediately.

## Deliberately rejected (state the reason to deviate)
Blade/Inertia/Livewire (headless API) · DB-per-tenant (shared-DB + RLS is the model) · SMTP (Mailgun API) ·
external RBAC/query/data libraries (hand-built) · a formatter (the hand-style is law) · **OpenAPI/doc
generators (the `routes/collections/` mirror is the contract)** · floats for money · raw SQL strings ·
`env()` outside config files.
