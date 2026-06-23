# Skill — Laravel Octane + FrankenPHP mastery

Octane runs the app as a **long-lived worker**: booted once, then thousands of requests share the same PHP
process and memory. That is the speed — and the trap. Everything here is correctness, not tuning: state that
survives a request is a cross-tenant breach waiting to happen.

## The mental model
A traditional Laravel request = boot, serve, die (clean slate every time). Octane = boot ONCE, then loop:
`handle → handle → handle` on the same process. **Anything you put in process memory persists to the next
request — including the previous tenant's data.** Write every line as if a different tenant runs the next
request on the same worker (they will).

## What SURVIVES between requests (the danger surface)
- **`static` properties** and static caches/memoization — persist and accumulate.
- **Singletons** (`$this->app->singleton(...)`) — resolved once, reused for every request. If a singleton holds
  request/tenant/user state, it leaks.
- **Container instances** bound at boot; service providers' `register`/`boot` run once.
- **Global helpers / facades that cache state**; closures captured at boot holding stale `$request`.
- **Event listeners registered at runtime** (inside a request) — they accumulate across requests and fire
  repeatedly. Register listeners in a service provider, never per-request.
- **Unbounded growth** — a `static array $cache` that only grows is a memory leak that eventually OOMs the
  worker.

## What is safe / what Octane resets
- Octane flushes some framework state between requests (the `Auth`, `Request`, and resolved `Request`-bound
  instances) via its reset hooks — but it does **NOT** clean YOUR singletons/statics. Don't rely on it for
  your own state.
- **DB and Redis connections persist** (a feature — no reconnect cost). Consequence: connection-level state
  persists too → RLS GUC must be **transaction-local**, never session `SET` (see `tenancy-playbook.md`).
- Stateless services (pure helpers, the Support DSL) are perfectly safe and ideal.

## Statics done right (the one great use)
Octane's persistence is a GIFT for **immutable, boot-derived, tenant-independent metadata** — the DB schema,
discovered relations, fillables, resolved config and env. Compute once, cache in a `static`/`const`, reuse
across every request with zero recompute. This is a major Octane win and exactly where the engine leans on
statics (`abstraction-engine.md`). The dividing line: **immutable + tenant-independent → static is ideal;
anything per-request or per-tenant → NEVER static.** The shape of the `products` table is identical for every
tenant → cache it statically; the current `tenant_id`/role/user is per-request → it lives in `Context`.

## The rules (non-negotiable)
- **Per-request state lives ONLY in `Context`** (request-scoped, Octane-aware) — never a static/singleton/
  container binding. The role/tenant/user tag is the canonical example (`arch.md §6`).
- **Reset tenant-scoped state on `RequestTerminated`** (`Context::forget()` and any per-request caches).
- **Register listeners/macros/bindings at boot only**, never inside a request handler.
- **Bound memoization** — if you memoize in a static, key it per-request or clear it on `RequestTerminated`;
  never let it grow unbounded.
- **Health/readiness + worker recycling** — set `max_requests` so workers recycle (defense against slow
  leaks); expose a readiness endpoint; drain in-flight requests on reload (graceful, no dropped work).

## The gotcha checklist (run it on every PR)
1. Any new `static` property holding state? → move to `Context` or clear on terminate.
2. Any `singleton` that stores request/tenant/user data? → make it stateless or request-scoped.
3. Any listener/macro/binding registered inside a controller/service/job? → move to a provider.
4. Any session-level DB setting (`SET ...`)? → transaction-local instead.
5. Any growing static cache? → bound it or clear on terminate.
6. Does tenant context get set AND reset for every entry point (HTTP, job, command, broadcast)?

A subtle Octane state bug is silent in dev (one worker, one tenant) and catastrophic in prod (one worker,
many tenants). Treat the survival surface as hostile by default.
