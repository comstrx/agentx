# Skill — the cascading permission engine (authority-locked, frontend-aware)

Hand-rolled, server-authoritative authorization, built fresh for this archetype — NOT a third-party package and
NOT a flat role→permission map. The crown of the access model: ONE permission resolves down an authority
cascade where the platform owner (`super`) can set a value for any tenant AND **lock** it, each tenant freely
governs everything `super` left open — per entity and per single item — and the resolver returns not just
"allowed?" but **"who decided, and may the tenant still change it?"** so the frontend can render a locked toggle
faded/read-only. Built on the Support DSL + cache; gated at the edge by `has:<permission>` middleware. Generalize
this — never reach for an external RBAC library (`tools.md`).

## Two tables only
- **`permissions`** — the platform **catalog** (vocabulary only): `(key unique, group, label)` — `view_products`,
  `edit_products`, `allow_comments`, `allow_payouts`, … Platform-owned; this table holds NO allow/deny — it is
  just the dictionary of what CAN be governed.
- **`permission_settings`** — ONE uniform table for **every** level of the cascade. Each row sets `allow` +
  `locked` at a scope, owned by an `authority`:
  `(tenant_id?, permission_id, scope ∈ {global, tenant, entity, item}, role?, target_type?, target_id?,
   user_id?, allow bool, locked bool, authority ∈ {super, tenant})`.
  - `global` — `tenant_id` NULL, `authority=super`: a platform-wide value for the permission.
  - `tenant` — a value for one whole tenant (super tuning a tenant, or the tenant's own baseline).
  - `entity` — a value for a `role` (admin/vendor/…) or a resource type (`target_type` = the model) within the
    tenant — "the tenant disables comments for ALL products".
  - `item` — a value for ONE record (`target_type`+`target_id`), optionally for ONE actor (`user_id`) —
    per-item, per-actor, or per-actor-on-this-item.
  `unique` per scope shape; every hot index leads with `tenant_id`. No more `entities`/`special_permissions`
  split — one cascade, one walk, trivial to extend with a new scope.

## The authority law (the priority — this is the genius)
`super` outranks `tenant`, always. Two flags carry it: `allow` (the value) and `locked` (may a lower authority
change it?).
- A `super` row with **`locked = true` is FINAL** at its scope — the tenant CANNOT override it. Its `allow`
  stands; the tenant UI shows that permission **faded / read-only**.
- A `super` row with `locked = false` is a **default**: the tenant may override it at a more specific scope
  (entity/item).
- A `tenant` may write `entity`/`item` (and its own `tenant` baseline) rows **only** for permissions no higher
  authority has locked. A tenant lock binds the tenant's own lower scopes (entity locks item), never `super`.
Priority is proximity to `super`: global-super-lock > tenant-super-lock > tenant's own settings.

## Resolution → `{ allow, locked, source }` (fail-closed, frontend-aware)
For `(tenant, permission, ?role, ?item, ?actor)` the resolver returns a struct, not a bool:
1. **Locked wins from the top:** the highest-authority `locked` row that matches (global → tenant → entity →
   item) decides — `allow` = its value, `locked = true`, `source` = that scope/authority.
2. **Else most specific:** the explicit `allow` at item+actor → item → entity → tenant → global, `locked = false`.
3. **Else** the catalog default (if any) → else **deny**. "No rule" is never "allowed".
Two consumers, two fields: **`allow` drives the `has:` gate** (access); **`locked` drives editability** (the
frontend greys out the toggle, and the tenant write-API rejects a change to a locked permission). `source` lets
the UI explain "locked by platform".

## The DNA (`app/Traits/Dna/Permissions/`)
A `HasPermissions` facade trait composes the cascade for a model/actor; the resolver itself lives in the Support
DSL (cacheable, stateless). The surface a model/actor exposes:
- `can(key, ?item): bool` — effective `allow` (what the gate uses).
- `setting(key, ?item): {allow, locked, source}` — the full tri-state for the frontend.
- `settings(scope): map` — the editable matrix a tenant admin renders (each entry carrying `locked`).
- tenant writes: `grant/revoke(key, scope, ref, lock?)` — refuses if a higher authority locked it.
- super writes: `force(key, scope, allow, lock=true)` — sets and locks across a tenant/global.
A model gains item-level governance by `use HasPermissions;`; a `User`'s effective set is resolved against its
roles. Resolution is **fail-closed** (`hasOrFail` → 403) and returns `$this` for chaining.

## Cache (woven in, tenant-scoped)
The resolver is HOT (every request, every list row) → cache the resolved set per `tenant + permission + scope`
via `support/cache` with tag invalidation; any write (`grant`/`revoke`/`force`/lock change) busts the affected
tag, never a full flush. Keys are tenant-prefixed.

## The `super` flow (cross-tenant control)
`super` (detected via `Context::isSuper()`, audited `withoutTenancy()`) writes `global` or per-`tenant`
`permission_settings` and chooses `locked`. A locked super setting is exactly "the tenant can't touch this" →
faded in their panel. Writes that target a tenant carry a **validated** `tenant_id` from the body. Authority is
read from `Context`, NEVER from the payload.

## The edge gate
`has:<permission>` middleware (`Has`) resolves effective `allow` through the cascade and 403s on deny — thin
pipeline, the cascade logic lives in the DNA/Support. `locked` is NOT consulted by the gate (it governs editing,
not access). Role membership (`Role` middleware) is the coarse gate; `has:` is the fine one. Both
`route:cache`-safe (string middleware, no closures).

## The leak / mistake vectors — hunt every one
- **Open default** — a missing rule treated as allowed. Resolution MUST fail closed.
- **Lock bypass** — a tenant write changing a permission a `super` row locked; the write-API must reject it, and
  resolution must let the super-lock win.
- **Confusing `allow` with `locked`** — gating on `locked`, or letting the tenant edit a faded permission.
- **Trusting the client** — reading role/permission/tenant from the body instead of `Context`.
- **Unscoped setting row** — a `permission_settings` row missing `tenant_id` for a non-global scope → cross-tenant
  grant. Only `scope=global` is `tenant_id` NULL.
- **Stale cache** — a grant/lock change not busting the tag → the old verdict (or stale faded-state) lingers.
- **N+1 on resolution** — resolving per row; resolve the set once per (tenant, entity), reuse across the page.
