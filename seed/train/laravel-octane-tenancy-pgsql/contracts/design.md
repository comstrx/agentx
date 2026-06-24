# contracts/design.md — Design pattern & abstraction philosophy (LAW)

How to think. `arch.md` says where things live; this says how the code must be shaped. The whole point of
this archetype is **abstraction at the highest level**: declare, and the engine materializes. Re-read every turn.

## 1 — Business code is pure pipeline (the headline law)
A service / lifecycle file expresses business intent as a **pipeline of calls into the Support DSL and the
trait engine** — and **NOTHING native** inline. No native PHP (`array_*`, `preg_*`, hand loops for transforms,
`json_*`) and no native Laravel (`DB::`, `Http::`, `Cache::`, `Storage::`, `Mail::`, ad-hoc query builder)
sitting in a business method. When a feature needs a native/infra capability:

1. Reach for the matching Support domain (`arr`, `cache`, `http`, `database`, `num`, …).
2. If it does not exist yet, **ADD it there** (or add a capability as a trait) — built once, named well.
3. Then call it from the business code.

Over the life of the project the **Support std-lib and the trait DNA GROW** with each feature's needs, while
the business/lifecycle files stay thin, declarative pipelines that **read like the use case**. **If a
lifecycle file contains native programming, that logic is in the wrong layer — push it down.** This is how
every feature distributes itself across the layers instead of piling raw code into one place.

## 2 — Repository pattern, layered — responsibilities
| Layer | Owns | Never does |
|-------|------|-----------|
| **Model** | Schema mapping, casts, relations, DNA traits, fillable. | Query orchestration, HTTP, validation. |
| **Repository** | Data access: `fields()`, query building, CRUD, scopes, boot hooks. | HTTP, authorization, response shaping. |
| **Service** | Business logic as a pipeline, orchestration across repositories, transactions, domain events. | Direct query building, native primitives, HTTP. |
| **Controller** | Thin: read role/tenant from `Context`, assemble scopes/permissions, call the service, return a Resource. | Business logic, data access. |
| **Request** | Mandatory validation + authorization at the boundary. | Business logic. |
| **Resource** | Output shaping into the uniform envelope. | Data access, side effects. |

The base engine (`arch.md` §3) carries the repeated work for **every** layer, so concrete classes are
near-empty. **A concrete class exists to declare differences, not to re-implement the pattern.**

## 3 — Schema & conventions are the single source of truth
Routes, permissions, eager-loads, and write-fields are **DERIVED, not hand-written.**
- The model **declares** its relation methods (normal Eloquent); the engine **auto-discovers** them by
  reflection, caches the map in a `static` (Octane-safe, boot-derived), and derives eager-loading, requested
  includes, and the nested-relation endpoints FROM that map. You declare a relation once; you never hand-wire it
  per endpoint.
- The resource name drives the route set and the `view_/add_/edit_/delete_<resource>` permissions (`naming.md`).
- `Repository::fields()` is the declared write-shape; the engine maps request input through it.

DX target per new resource: `migration + Model (use traits) + Repository::fields() + Service (overrides only)
→ full auto API` (the concrete engine — uniform read pipeline, role-driven scopes thread, `fields()`+boot
hooks, request-derived filter DSL, `__call` nested dispatch — is the playbook `skills/abstraction-engine.md`).
The engine materializes CRUD / search / stats / files / permissions / relations / nested
relations / tenant-scope. **You declare; the engine materializes.** Hand-writing the same shape twice means it
belongs in the engine.

## 4 — Engine holds behaviour; concrete is declaration
Generic behaviour is pushed **DOWN** (`support → traits → bases`); concrete classes are pure declaration
(overrides + `fields()`). New shared behaviour goes in the `HasBaseXxx` trait, never duplicated into a
concrete class. **Rule of two:** needed by two systems → engine-level; by one → a concrete override. Abstract
the **second** time you need a thing, not the first (`tolerance.md`).

Consequence: MOST systems (`categories`, `products`, `users`, `coupons`, `blogs`, …) are **near-empty
declaration** — a migration, a Model with its traits, `Repository::fields()`, a Service with overrides only.
Genuine domain logic the engine cannot derive — `auth`, the `order`/payment **lifecycle**, a settlement or
pricing rule — lives in THAT system's Service as an explicit pipeline. It is the deliberate exception, not the
norm, and even then it is pipeline business logic over the Support DSL + traits, never native primitives
(`§1`). If a "simple" CRUD system grows real logic, first ask whether it belongs in the engine.

## 5 — DNA traits (opt-in capability)
A capability is a **trait a model `use`s** to gain a whole feature: `HasFiles`, `HasSearch`, `HasState`, …
- **Self-contained:** it brings its own scopes, accessors, boot hooks, and a small config surface — mounting
  it is the WHOLE integration. `use HasFiles;` means that model now has files end to end, no per-model wiring.
- Built **on top of the Support DSL**; never re-implements native/infra work.
- Opt-in surface **minimal and hard to misuse**. A model opts out explicitly (a `disabled` list the trait
  reads), never by editing the engine.

## 6 — Swappable drivers (every infrastructure capability)
`cache · lock · throttle · queue · event · storage · payments · search · ai` are adapters behind a neutral
interface:
```
support/<domain>/
├── index.php       // manager/facade: App\Support\<Domain> — the ONLY thing callers touch
├── Driver.php      // the interface (the contract)
├── RedisDriver.php // a concrete backend
└── …
```
Adding/replacing a backend = **ONE new Driver file + config**. Callers never reference a concrete backend;
business code never names a broker/provider directly. **Keep the neutral interface even when only one backend
exists today** — the interface is what makes the swap a one-file change later. Defaults: events → Redis/Horizon
(outbox-ready, add an `outbox` table when durable delivery is needed); payments → a gateway contract +
`StripeDriver` + webhook pipeline + lifecycle state machine; AI → Claude/Anthropic latest (`tools.md`).

## 7 — The "magic" policy (deterministic automation, never obscurity)
Schema-derived relations, auto eager-load, the reusable route blocks, relation dispatch, the auto resource
action set — **welcome and intended**. The bar:
- **Deterministic automation the team understands**, not accidental obscurity.
- **Octane-safe** — no per-request state in singletons/statics; the tag lives in `Context` (`arch.md` §6).
- **`route:cache`-safe** — explicit reusable route blocks, no route closures, no boot-time `glob()`.
- **Fail loud and closed** — tenant scope is fail-closed; an unknown relation 404s. Never silently wrong.

N+1 is eliminated by auto eager-load from auto-discovered relations + requested includes, with
`Model::preventLazyLoading()` as the dev tripwire.

## 8 — Simple, hard-to-misuse public surfaces
Minimal public API per Support facade / DSL; explicit over implicit; stable contracts. No speculative
abstraction, no premature generality — abstract a thing the **second** time you need it. Delete before adding;
reuse what exists before introducing a new pattern or dependency (`tolerance.md`). A new capability earns its
abstraction only when a real system needs it. Build on demand.
