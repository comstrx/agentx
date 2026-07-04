# Architecture — layers, the Base engine, the Support std-lib

**Repository pattern, layered.** ONE architecture, two views of the same stack:
- **Build / dependency order (inner → outer):**
  `support → traits → bases → repository → service → controller → request → middleware → route`
- **Runtime request flow (outer → inner):**
  `route → middleware → request (validation) → controller → service → repository → model (+ traits) → support`

Code structure is **FLAT, not modular domains**: `app/Models/*`, `app/Repositories/*Repository.php`,
`app/Services/*Service.php`, `app/Http/Controllers/*Controller.php`, `app/Http/Requests/*`,
`app/Http/Resources/*`. A single unified `XxxController` serves all panels; behaviour differs by the active
panel role read from `Context`.

## The Base engine (this IS the magic)
The real logic of each layer lives in a `HasBaseXxx` **trait** under `app/Traits/Bases/`. Each layer has a
thin `BaseXxx` **shell class** that does nothing but `use` its trait; concrete classes **extend** the shell
and stay near-empty — declaring only what is unique to them (`fields()`, an override). New shared behaviour
goes in the **trait**, NEVER in a concrete class. One per layer: `HasBaseModel`, `HasBaseRepository`,
`HasBaseService`, `HasBaseController`, `HasBaseRequest`, `HasBaseResource`, `HasBaseCommand`.

## The Support std-lib (`app/Support/`)
Native / infrastructure power ONLY — **ZERO business logic**. Folder-per-domain; each folder's `index.php` is
the public facade (`App\Support\<Name>`), internal PascalCase files hold the pieces. Even a one-file helper
becomes `folder/index.php`. Every trait in `Bases/` and `Dna/` is built ON TOP OF this DSL and never
re-implements native/infra work — and business code reaches native/infra power ONLY through it (never raw
PHP/Laravel inline). The canonical domain map (the binding contract for naming/shape) lives in
`contracts/arch.md §5`.

## Traits — two folders only
- `app/Traits/Bases/` — the `HasBaseXxx` engine traits (the reusable logic of every layer).
- `app/Traits/Dna/` — opt-in model **DNA**: a capability a model gains by `use`-ing the trait (`HasRoles`,
  `HasPermissions`, `HasFiles`, `HasSearch`, `HasCache`, `HasRelations`, `HasState`, …).

## Context (Octane-safe)
The active panel role + `tenant_id` + `super` flag live in request-scoped Laravel `Context`, set by the panel
middleware and read by the base controller/service to compute scopes and permissions; `database/Rls` and
`queue/Tenant` read from it. NEVER hold per-request state in long-lived singletons, statics, or container
bindings (Octane). Reset tenant-scoped state on `RequestTerminated`.

## Boundaries that must hold
Controller is **thin** — authorize → resolve a Form Request → call one Service method → return a Resource; no
query, no business branch. Service = orchestration + business. Repository = data access (`fields()` +
overrides). Model = domain + DNA traits + `HasTenant`. Support = power, zero business. Routes are
**explicit reusable blocks per panel**, `route:cache`-safe — no closures, no boot-time globbing.
