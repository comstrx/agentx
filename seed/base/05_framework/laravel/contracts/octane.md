# Octane

The app runs on Octane (FrankenPHP) — a long-lived worker, not a per-request process. Everything request-scoped must be engineered for that, non-negotiably.

- **Plain Laravel vs Octane, the one-line difference:** classic PHP-FPM boots the framework per request and dies (state leaks are impossible by accident); Octane boots ONCE and loops requests through the same process (state leaks are the default failure). The rules below are what "Octane-ready" means — and they are NOT optional on plain Laravel either: queue workers, Horizon, and scheduled commands are long-lived processes in EVERY Laravel deployment, so code written to these rules is simply correct Laravel; code that ignores them merely crashes later and further from the cause.

- **NEVER store per-request state in:** long-lived singletons, static properties, request-bound global helpers, or container bindings resolved once and mutated. Octane bleeds them across requests — the next user inherits the previous user's tenant, role, or cart. This is the framework's one lethal trap.
- **The actor tag lives in request-scoped `Context`**, wrapped by `App\Support\Context` as the only accessor: `tenantId() · role() · panel() · userId() · isSuper() · set(...) (middleware only) · forget()`. Middleware sets it at request start; base layers read it; **reset on `RequestTerminated`** along with any other tenant-scoped state.
- **Safe statics are boot-derived and immutable:** a reflection-discovered relation map, a parsed config shape — computed once, never mutated per request. The test: if two concurrent users could see different values, it cannot be a static.
- **Memory is a budget:** no unbounded accumulating caches in worker memory, no giant collections held after the response; watch for growing arrays in singletons — the worker lives for thousands of requests.
- **Connections are long-lived:** database and Redis connections persist across requests — transactions must never leak open, session-level settings (`SET`) are forbidden in favour of transaction-local ones, and any `set_config`/lock acquired per request is released per request.
- Fresh state per request for anything actor-shaped: resolve through the container per request or read from `Context` — never "cache it on the class, it's just one request".
- `preventLazyLoading()` in dev, plus Octane's warm-boot advantage: heavy boot work (route map, discovered maps, config) is paid once — design derivations to exploit that deliberately.
- Every new global/static/singleton in review answers one question first: **what happens on request #2?**
