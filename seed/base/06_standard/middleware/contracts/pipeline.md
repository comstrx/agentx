# Pipeline

- **Middleware is the edge's assembly line:** each unit does exactly one cross-cutting job — authenticate, resolve tenant, stamp context, throttle, localize, log — and passes on. A middleware doing two jobs is two middleware; business logic in middleware is misplaced code, always.
- **Order is a contract, declared once:** identity → tenant/domain resolution → context stamping → authorization gates → throttling → the handler. Each unit states what it assumes (auth ran) and what it guarantees (context set); reordering is a reviewed change, not a shuffle.
- **Middleware writes context, layers read it:** the request-scoped context (actor, tenant, panel, locale, correlation id) is *set* here through the one accessor and *read* everywhere below — no layer re-parses headers or re-resolves the tenant. One writer, many readers.
- **Declarative attachment:** middleware binds to route groups/scopes by name (`has:view_products`, `throttle:plan`), parameterized where it varies — never invoked manually inside handlers, never duplicated per route by copy-paste.
- **Fail closed at every gate:** missing token, unresolvable tenant, mismatched token↔domain, absent permission — each refuses with the uniform envelope and the correct status; a gate that "lets it through for now" is a hole with a TODO.
- **Cheap by obligation:** middleware runs on every request — no queries a cache can answer, no allocation-heavy work, no outbound calls except the auth/limits stores it exists for. The pipeline's cost is a budget measured like any hot path.
- **Symmetric on the way out:** units that alter state (context, locks, timers) restore/reset on response and on failure both — request-end cleanup is part of the unit, not an afterthought elsewhere.
- Exception translation is the outermost ring: domain errors → envelope + status in one translator; nothing inside the pipeline crafts error responses by hand.
