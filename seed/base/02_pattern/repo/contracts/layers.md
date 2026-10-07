# Layers

- Build / dependency order (inner → outer): **support → traits → bases → repository → service → controller → request/boundary → route**.
- Runtime flow (outer → inner): **route → boundary validation → controller → service → repository → model (+ dna traits) → support**.
- Each layer depends only on layers inner to it. A controller never touches a model directly — it goes through its service, the service through its repository. Support depends on nothing but sibling support.

| Layer | Owns | Never does |
|-------|------|------------|
| **Support** | The project's private std-lib: pure native/infra capabilities, swappable drivers. | Business logic; knowing any upper layer exists. |
| **Model** | Schema mapping, casts, relations, dna traits, writable fields. | Query orchestration, transport, validation. |
| **Repository** | Data access: declared write-shape, query building, CRUD, scopes, persistence hooks. | Transport, authorization, response shaping. |
| **Service** | Business logic as a pipeline, orchestration across repositories, transactions, domain events. | Direct query building, native primitives, transport. |
| **Controller** | Thin translation: read actor context, assemble scopes/permissions, call the service, return a shaped resource. | Business logic, data access. |
| **Request / boundary** | Mandatory validation and authorization at the edge. | Business logic. |
| **Resource / response** | Output shaping into the uniform envelope. | Data access, side effects. |

- Structure is **flat per layer**, not modular domains: all models together, all repositories together, all services together. One unified controller per resource serves every actor; behaviour differs by context, not by parallel controller trees.
- Every request produces a uniform response envelope: `success → { status, data, …extra }`, `fail → { status, message, errors }` — one envelope, never a second shape.
- The layer set is closed: new behaviour finds its home inside an existing layer's engine, it never invents a new layer.
