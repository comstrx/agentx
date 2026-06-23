# contracts/data.md — Data model & schema (LAW)

The canonical schema. Pairs with `arch.md §9` (DB conventions) and `overview/domain.md` (the shape). Re-read
every turn. Every table is tenant-scoped unless it is platform infrastructure.

## Universal rules
- **UUIDv7** primary key (`string`) on every table. **`tenant_id`** (UUIDv7, indexed, FK) on every
  tenant-owned table. FKs are `<singular>_id`.
- **Every business unique includes `tenant_id`**; composite uniques and hot indexes **lead with `tenant_id`**.
- Money is integer minor units; time is UTC; closed sets are PHP enums; soft-delete via a `deleted` flag /
  `deleted_at` where the domain needs it.
- RLS policy + `FORCE ROW LEVEL SECURITY` on every tenant-owned table (`skills/tenancy-playbook.md`).

## Platform-level tables (the deliberate exceptions to tenant-scoping)
Most tables are tenant-scoped; these are NOT (no `tenant_id`, or `tenant_id` **nullable** for shared / owned-by-
`super` rows). Keep this set explicit and small:
- **`tenants`** — the tenant registry itself.
- **platform / `super` users** (and their `roles`) — `tenant_id` null; the owner identity spans tenants.
- **`permissions`** (the catalog) — platform-owned vocabulary; and **`permission_settings` `scope=global`** rows
  (`tenant_id` null) = the platform defaults `super` authors.
- **`zones`** — provider (Cloudflare / Vercel) infrastructure. (`domains` ARE tenant-owned and carry `tenant_id`.)
- **shared reference `locations`** — `tenant_id` **nullable**: NULL = shared geography every tenant references,
  set = a tenant's own destination/attraction.
Everything else carries a non-null `tenant_id` and is RLS-forced.

## Identity
- **`users`** — `unique(email, tenant_id)`. **NEVER `unique(email)` alone** — the same email registers
  independently per tenant. Apply the same `(value, tenant_id)` rule to any other "unique" human identifier
  (phone, username).
- **`roles`** — `role` enum {admin, vendor, affiliate, delivery, client}, `is_super` bool, `is_supervisor`
  bool, `tenant_id`. Hierarchy per role-type: one `is_super` owner → `is_supervisor`s → members.
- **`user_roles`** — pivot (`user_id`, `role_id`, `tenant_id`), `unique(tenant_id, user_id, role_id)`.

## Authorization (the cascading permission engine — `skills/rbac-permissions.md`)
- **`permissions`** — the platform CATALOG (vocabulary only): `(key unique, group, label)` — `view_products`,
  `allow_comments`, … No allow/deny here; this is just what CAN be governed. Platform-owned.
- **`permission_settings`** — ONE uniform cascade table for every level; each row sets `allow` + `locked` at a
  scope, owned by an `authority`:
  `(tenant_id?, permission_id, scope ∈ {global, tenant, entity, item}, role?, target_type?, target_id?,
   user_id?, allow bool, locked bool, authority ∈ {super, tenant})`.
  - `global` (tenant_id NULL, authority super) · `tenant` (whole tenant) · `entity` (a `role` or a resource
    `target_type`) · `item` (a record `target_type`+`target_id`, optionally a `user_id` actor).
  - `unique` per scope shape; hot index leads with `tenant_id`. Only `scope=global` is `tenant_id` NULL.
- **Authority & lock (the priority rule):** `super` outranks `tenant`. A `super` row with `locked=true` is FINAL
  — the tenant cannot change it (its panel shows that permission **faded / read-only**). An unlocked `super` row
  is a default the tenant may override; the tenant writes `entity`/`item`/tenant-baseline rows only where no
  higher authority locked the permission. A tenant lock binds its own lower scopes only.
- **Resolution → `{allow, locked, source}`** (fail-closed): the highest-authority `locked` match wins; else the
  most specific allow (item+actor > item > entity > tenant > global); else the catalog default; else deny.
  `allow` drives the `has:` gate (access); `locked` drives editability (the frontend greys out locked toggles,
  the write-API rejects edits to them). Never store or trust a permission from the client.

## Catalog (`products` — single-table polymorphism)
- `type` enum {hotel, room, realstate, course, service, product, tour, visa, event} (indexed); `subtype` enum
  (indexed).
- `category_id` FK; `product_id` self-FK (parent, nullable).
- Type-specific columns are **nullable, sized to the richest target**; high-variance / rare / single-type
  attributes go in a **`jsonb attributes`** column with a GIN index — never a near-empty column per minor type.
- Validation is **per type** (a Form Request rule-set keyed on `type`); a Resource hides null type-columns.
- Indexes: `(tenant_id, type, subtype)`, `(tenant_id, category_id)`, `(tenant_id, product_id)`; **partial**
  indexes for hot `WHERE type=? AND active` paths; GIN on `attributes`.
- **`prices`** + **`product_prices`** pivot — `unique(tenant_id, product_id, price_id)`; many prices per product.

## Locations (`locations` + closure)
- ONE table; `type` ∈ {country, city, region, destination, attraction}; `parent_id` self-FK; **`tenant_id`
  nullable** (NULL = shared base geography all tenants reference; set = a tenant's own destination/attraction —
  see Platform-level); a **`location_closure`** table (`ancestor_id`, `descendant_id`, `depth`) for O(1)
  subtree queries. The same closure pattern serves deep `products` trees, and powers recommendations (link a
  product to a city/country).

## Content & social engagement (`skills/social-engagements.md`)
- **`blogs`** (+ `categories` shared with the catalog) — tenant-scoped content; engageable like any other model.
- **Engagement = ONE morph table per kind** (there is NO single "engagements" table). Each is polymorphic on
  `(engageable_type, engageable_id)` + `tenant_id` + `user_id`, indexed `(tenant_id, engageable_type,
  engageable_id)`:
  - **`views`** · **`favorites`** — presence toggles, `unique(tenant_id, *_type, *_id, user_id)`.
  - **`likes`** — a single toggle table with a `like` bool (`true` = like, `false` = dislike, **mutually
    exclusive — no `dislikes` table**), `unique(tenant_id, likeable_type, likeable_id, user_id)`.
  - **`comments`** · **`replies`** · **`reviews`** (`rating`) · **`reports`** (`reason`) · **`files`**
    (path/mime/size; images & qrcodes are the same table keyed by `kind`) — each carries its own fields.
  - Hot aggregates (`views`, `likes`, `dislikes`, `comments_count`, `rating_avg`) are denormalized columns,
    maintained atomically with the morph write (`hasColumn`-guarded), never recounted on read.
- **`notifications`** + **`notification_user`** pivot — fan-out recipients by bulk insert; delivery is queued,
  realtime broadcast immediate.

## Tenancy surface
- **`domains`** — (`tenant_id`, `host` globally unique, `kind` apex|subdomain, `zone_id`, `status`); resolves
  host → tenant.
- **`zones`** — `provider` enum {cloudflare, vercel}, provider zone id, `status`. Buy / transfer / subdomain
  provisioning runs through the swappable domain driver (`tools.md`), queued and idempotent.

## Migrations
Central, reversible, additive-first; UUIDv7; `tenant_id`; composite uniques leading with `tenant_id`. No
destructive drop in the same release as code that still reads the column.
