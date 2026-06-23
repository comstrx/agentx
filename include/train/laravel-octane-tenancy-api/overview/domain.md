# Overview — the canonical domain model

The reference data shape of this archetype: a multi-tenant, multi-business commerce/booking SaaS. Strongly
opinionated defaults; a concrete project adapts via its own `agents/`. **Every table is tenant-scoped**
(`tenant_id`, UUIDv7). The enforceable schema law is `contracts/data.md`.

## Identity & roles
- **`users`** — one identity per tenant. Email is unique **per tenant** (`unique(email, tenant_id)`), so the
  same email can register and log in across different tenants independently.
- **`roles`** — a `role` ∈ {`admin`, `vendor`, `affiliate`, `delivery`, `client`} plus flags **`is_super`**
  and **`is_supervisor`**. Each admin-side role type (admin / vendor / affiliate / delivery) forms its OWN
  account hierarchy: one `is_super` (the account owner) who creates `is_supervisor`s and ordinary members
  beneath it. `client` is the buyer surface.
- **`user_roles`** — links `users` ↔ `roles` (many-to-many; a user may hold roles across panels). The active
  panel + role drive the request's scopes/permissions through `Context` (`arch.md §6`).

## Authorization — the cascading permission engine
Hand-rolled RBAC as an authority cascade over **four scopes**: **global** (platform value set by `super`) →
**tenant** (per-tenant) → **entity** (per role / per resource type, set by the tenant) → **item** (one record,
optionally one actor). Two tables: a `permissions` **catalog** (vocabulary) + a uniform `permission_settings`
cascade, each row carrying `allow` + `locked` + `authority{super,tenant}`. **`super` outranks `tenant`:** a
super-`locked` setting is final — the tenant cannot change it and it renders **faded/read-only** in their panel;
an unlocked super value is a default the tenant may override per entity or per item. Resolution returns
`{allow, locked, source}` (fail-closed): `allow` gates access, `locked` gates editability. The full craft is
`skills/rbac-permissions.md`.

## Catalog — one `products` table, many types (the polymorphic spine)
ONE table unifies every sellable thing via **`type`** ∈ {hotel, room, realstate, course, service, product,
tour, visa, event}. The genius is one brilliantly-shaped table instead of a table per type:
- **`subtype`** ∈ {online, offline, digital, physical, hotel, room, …} — the nature of the item.
- **`category_id`** (taxonomy) and **`product_id`** — a **self-parent** (hotel → rooms, a product → child
  products, a service → child services).
- Type-specific columns are sized to the **richest target** (e.g. hotels carry `checkin`/`checkout`/…); a type
  that doesn't need a column simply leaves it `null`. Sparse columns are cheap; rare/volatile attributes go in
  `jsonb`. Full craft + indexing in `skills/polymorphic-catalog.md`.

## Pricing
**`prices`** + a **`product_prices`** pivot — a product / hotel / room carries **many prices** (tiers,
seasons, currencies). Money is integer minor units, never a float.

## Locations
ONE **`locations`** table for countries / cities / regions / destinations / attractions, hierarchical via a
**closure table** — O(1) ancestor/descendant queries without recursive joins, and the join that powers
recommendations (link a product to its city/country/destination).

## Content & social engagement
- **`blogs`** (sharing `categories` with the catalog) — tenant-scoped content, engageable like any model.
- **Engagement is polymorphic DNA:** any model gains like / dislike / view / comment / reply / review /
  favorite / report / share / file / image / qrcode / audit-log / notification by `use`-ing a trait from
  `app/Traits/Dna/Social/` — backed by **one morph table per kind** (`views`, `likes` (a `like` bool toggle —
  dislike is `like:false`, no `dislikes` table), `favorites`, `comments`, `replies`, `reviews`, `reports`,
  `files`, …), each **gated by the permission ladder** (`allow_*`). Counters are denormalized columns
  maintained atomically; notifications fan out queued + realtime. The craft is `skills/social-engagements.md`.

## Tenancy surface — domains & zones (Cloudflare / Vercel)
- **`domains`** — maps a domain or subdomain to a tenant; backs custom-domain resolution + the token↔tenant
  binding (`tenancy.md`).
- **`zones`** — the provider zone backing a domain.
- Features: **buy a domain, transfer a domain, create subdomains** on the platform's root domain — all via
  swappable provider drivers (Cloudflare, Vercel), `design.md §6`. Provisioning is queued and idempotent.
