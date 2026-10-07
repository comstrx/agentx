# Orchestration

The service layer — where use cases live as named, transactional pipelines.

- **One service method = one use case**, named in domain language (`confirmBooking`, `settleVendor`, `transferOwnership`) — never CRUD verbs wearing business costumes. The method body reads as the use case's steps; a reviewer can read it aloud to the domain expert.
- **Services orchestrate, layers beneath execute:** the service sequences repositories, domain operations, and support capabilities; it holds zero data-access details and zero native primitives inline — the pipeline law applies here hardest.
- **The transaction boundary is the service method:** open around the use case's consistency scope, commit before the consequences fan out; events and jobs dispatch after commit. A repository opening its own transactions or a controller managing commits has stolen this role.
- **Typed shapes in, typed shapes out:** a service takes validated, typed input (the boundary already validated) and returns domain results — never transport shapes (requests, responses, session state). A service importing HTTP types has leaked the surface into the core.
- **Services stay stateless:** dependencies injected, no mutable fields accumulating between calls — the same instance must serve concurrent use safely.
- **Cross-entity logic lives here:** a rule spanning two aggregates/resources (booking + wallet, order + stock) belongs to the service orchestrating both, not to either model — models own their own invariants, services own the choreography.
- **Escalate to explicit workflow when the pipeline outgrows a method:** multi-step, resumable, compensating flows (payment sagas, provider provisioning) become state-machine-backed processes — the service starts them; it does not simulate them with nested try/catch.
- Depth discipline: services call downward only — a service calling another service is allowed one level for composition; a chain of services calling services is a pipeline that lost its owner. Extract the shared step downward instead.
