# Pattern — the building blocks and the automation philosophy

The vocabulary of this archetype. This file describes WHAT each pattern is and WHEN to reach for it; the
enforceable rules live in `contracts/`. Use these — do not invent a parallel mechanism for the same job.

## The "magic" — deterministic automation, abstracted at the highest level
This is the highest-leverage idea in the codebase — master it and the rest follows. The aim is **maximal
abstraction with zero obscurity**: declare intent, and the engine materializes the machinery.
The **schema + naming conventions are the single source of truth**: relations, routes, permissions, and
fields are DERIVED, not hand-written. Generic behaviour is pushed DOWN into the engine
(`support → traits → bases`); concrete classes are **pure declaration**, not implementation. Every resource
exposes the **same uniform surface** → predictable, self-similar code and API. "Magic" is welcome ONLY when
it is deterministic automation the team understands (auto resource routes, schema-derived relations) — never
accidental obscurity — and it MUST be Octane-safe and `route:cache`-safe.

## The per-resource DX (what you actually write)
```
migration  +  Model (use traits)  +  Repository::fields()  +  Service (overrides only)  →  full auto API
```
The engine that materializes this — the uniform read pipeline, the role-driven scopes thread, `fields()`+boot
hooks, the request-derived filter DSL, cache-by-query-shape, `__call` nested-relation dispatch — is the
playbook `skills/abstraction-engine.md`.
The engine materializes CRUD / search / stats / files / permissions / relations / nested relations /
tenant-scope. Concrete classes stay near-empty.

## DNA traits (opt-in capability)
A model gains a whole capability by `use`-ing one trait: `HasTenant`, `HasRoles`, `HasPermissions`, `HasFiles`,
`HasSearch`, `HasCache`, `HasRelations`, `HasState` (state machine), … — each built on the Support DSL. Two
large subsystems live as their own trait groups under `Dna/`: **`Dna/Permissions/`** — the multi-level RBAC
ladder global→tenant→entity→item (`skills/rbac-permissions.md`); **`Dna/Social/`** — engagement traits
(`HasLikes`/`HasViews`/`HasComments`/`HasReviews`/`HasFavorites`/`HasFiles`/…) over polymorphic morph tables,
each gated by the ladder (`skills/social-engagements.md`).

## Swappable drivers (every infrastructure concern)
`cache · lock · throttle · queue · event · storage · search · ai · payments` = a `Driver` interface +
concrete driver(s) + a manager in `index.php`. Callers never reference a backend directly. Adding
Kafka/Redpanda/SQS, a new payment provider, or a search engine = ONE new `Driver` file + config; business
code never changes.

## Recurring patterns
- **Uniform envelope:** `success → {status:true, data, …}`, `fail → {status:false, message, errors}`. All
  output flows through `BaseResource` / a Support response helper — never an ad-hoc array.
- **Idempotency:** an `Idempotency-Key` header → middleware stores `key→response` under a lock, scoped per
  tenant+user+endpoint → safe retries, zero duplicate side effects.
- **State machines:** orders, and the payments lifecycle (intent→authorize→capture→refund→dispute→payout),
  are explicit state machines; transitions emit domain events through the events abstraction.
- **N+1 elimination:** auto eager-load from auto-discovered relations + requested includes;
  `preventLazyLoading()` as the dev tripwire. An N+1 on a list endpoint is a defect, not a tuning note.
- **Pagination is keyset; search is bounded and allow-listed.**
- **Events:** publish via `App\Support\Event::publish(event, payload, key)` behind a swappable `Driver`
  (Redis/Horizon default); add a transactional `outbox` when durable delivery is needed.

## Smallest correct piece (systems-first)
Build **systems, not layers** — pick a vertical and drive it through the layers; never pre-build a whole
layer. Reach for a new abstraction ONLY when a concrete seam demands it. Build only what the current system
needs. Delete before you add; reuse an existing trait/service before writing a new one.
