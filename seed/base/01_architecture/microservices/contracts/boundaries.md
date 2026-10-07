# Boundaries

- **A service is a business capability with an owner, not a folder with an API.** Cut services along domain seams (payments, catalog, identity) — never along technical layers (a "database service", an "API service") and never along team org-charts alone.
- **Data ownership is absolute:** each service owns its store; no other service reads or writes its tables — ever. Shared databases are a distributed monolith with network taxes. Another service's data arrives through its API or its published events, and is cached locally as a **read model the consumer owns**.
- **The contract is the service:** every API and event schema is versioned, explicit, and consumer-compatible — additive changes freely, breaking changes behind a new version with a migration window. A service is free to rewrite its internals; the contract is the promise.
- **Branch on capability here too:** a service asks "what can this actor/entity do" via claims and permissions carried in the request context — never re-implements another service's role literals.
- **Size by cohesion, not fashion:** a service is as big as its invariants require — entities that must change in one transaction live in ONE service. If two services deploy together, release together, and fail together, they are one service wearing two costumes: merge them.
- **Cross-service transactions do not exist.** Consistency across services is sagas + events + reconciliation — designed as workflows with compensation, never simulated with distributed locks or two-phase hope.
- **The extraction rule:** services are carved from a working monolith on evidence (independent scaling need, independent team velocity, isolation requirement) — never designed as confetti up front. The first cut is the seam the monolith already made extract-ready.
- Every service is independently deployable, testable against its contracts alone, and killable — if taking one service down takes three others with it, the boundaries are lies.
