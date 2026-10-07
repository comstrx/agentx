# Boundary

- **Validate at the boundary; trust inside.** Every write is validated by a dedicated request unit at the edge — types, shapes, sizes, whitelists. Past the boundary, input is typed and trusted; re-validation deep in the stack means the boundary leaked.
- **Untrusted input is hostile:** parameterize every query, whitelist filterable/sortable columns, bound every page size and payload, guard outbound URLs against SSRF, escape at output. Never weaken auth or escaping to make a feature work.
- **Fail closed.** Missing permission denies, unknown relation 404s, unmatched scope returns empty, ambiguous auth rejects. The default answer to "not sure" is no.
- **Authorization is server-authoritative:** real RBAC evaluated on the server per request; client-supplied roles, permissions, or ids of other actors are display hints at best, never inputs to a decision.
- **One uniform response envelope** for success and failure across the whole API; errors carry a stable machine key + a human message, and never leak internals (stack traces, SQL, paths, versions).
- **Stateless process:** request state dies with the request; sessions, cache, locks live in external stores. Any instance can serve any request — that is the scaling and deploy contract.
- **Idempotency on money and mutation-sensitive endpoints:** an idempotency key makes retries safe; duplicate delivery is a certainty, not an edge case.
- Pagination is keyset on anything that grows; search is bounded and whitelisted — never an open query surface.
- Secrets never appear in code, logs, errors, or responses; configuration comes from the environment, read once at boot.
- The API contract is maintained **in the same change** as the behaviour: whatever mirror the project uses (collection files, schema), a drifted contract is a broken build.
