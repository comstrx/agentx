# Layers

- One deployable, many strict layers. Four altitudes, inner to outer: **foundation** (the project's private std-lib: pure capabilities, zero business) → **engine** (shared behaviour every feature reuses) → **domain** (business decisions as thin pipelines) → **surface** (transport: HTTP, CLI, UI, jobs, schedulers).
- Dependencies point **inward only**. An outer layer may call anything beneath it; an inner layer never knows an outer one exists. No cycles, no sideways imports across features, ever.
- Runtime flow is the reverse of build order: input enters at the surface, crosses the domain, reaches data through the engine, touches primitives in the foundation.
- **No layer skipping.** The surface never touches storage or native primitives directly; it translates and delegates. The domain never parses transport shapes; it receives typed input.
- Wiring happens **once, at the composition root** (bootstrap/entry). Construction, configuration, and driver selection live at the edge; everything beneath receives its dependencies and stays constructor-injected, testable, and ignorant of the environment.
- Each layer has one job: foundation = power, engine = reuse, domain = decisions, surface = translation. A file doing two layers' jobs is two files.
- Cross-cutting concerns (auth context, tenancy, logging, cache) are foundation/engine capabilities that the domain *uses* — never re-implemented per feature.
- The dependency direction is what keeps the monolith splittable: any module whose edges respect it can be extracted into a service later without rewriting business logic. Guard the direction as a gate concern, not a taste preference.
