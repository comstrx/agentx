# Skill — the polymorphic catalog (one `products` table, many types)

ONE `products` table unifies hotel / room / realstate / course / service / product / tour / visa / event. The
craft of doing it without a table-per-type explosion and without a swamp of always-null columns. Schema law in
`contracts/data.md`.

## The shape
- **`type`** (the kind) + **`subtype`** (online / offline / digital / physical / hotel / room / …). Both
  indexed, both PHP enums.
- **`category_id`** (taxonomy) + **`product_id`** (self-parent: hotel → rooms, product → child products,
  service → child services).
- **Shared columns** common to all (name, slug, status, currency, `tenant_id`, …).
- **Type-specific columns sized to the richest target** (hotels: `checkin`, `checkout`, `stars`, …). A type
  that doesn't need a column leaves it `null` — sparse columns are cheap in Postgres.

## Column vs jsonb — the dividing rule
- **Column** when the attribute is stable, shared across several types, and **queried/filtered/sorted**.
- **`jsonb attributes`** (GIN-indexed) when it is rare, volatile, or **single-type** — never add a near-empty
  column used by one minor type with a couple of fields. Expression-index a hot jsonb key
  (`((attributes->>'k'))`).

## Behaviour per type without sprawl
- **Validation per type:** the Form Request returns a rule-set keyed on `type` — never one rule-set for all
  (that lets garbage in).
- **Resource per type:** the envelope exposes only the relevant fields; null type-columns are hidden.
- **Business per type:** resolve a `type → strategy` (a handler map / DNA trait), never a giant `match`
  smeared across every service. Adding a type = one strategy + its rules + its columns/jsonb.

## Parent / children & hierarchy
`product_id` self-FK gives parent ↔ children (hotel ↔ rooms). For deep trees use a **closure table**
(`ancestor_id`, `descendant_id`, `depth`) — the same pattern as `locations` — for O(1) subtree reads.

## Pricing
`prices` + `product_prices` pivot — many prices per product (seasons / tiers / currencies). Integer minor
units, never a float. `unique(tenant_id, product_id, price_id)`.

## Indexing (Postgres)
`(tenant_id, type, subtype)` for the hot list filters · `(tenant_id, category_id)` · `(tenant_id, product_id)`
for children · **partial** indexes per hot type (`WHERE type='hotel' AND active`) · GIN on `attributes`.

## Pitfalls (hunt these)
- A column added for one type, null for 90% of rows, queried by no one → should be jsonb.
- A `unique` that forgot `tenant_id` → cross-tenant collision.
- Type logic as a sprawling `match` repeated in every service → use a `type → strategy` map.
- `type`/`subtype` missing from the list index → seq scans on the hottest endpoint.
- One validation rule-set for all types → garbage data; validate per type.
