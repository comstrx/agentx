# Catalog

The polymorphic catalog craft — one table, many types, no null-swamp, no branch-sprawl.

- **The shape:** `type` + `subtype` (both indexed, both enums at the app layer), `category_id` (taxonomy), self-parent FK (hotel → rooms, product → variants), the shared columns every type uses, and type-relevant columns sized to the richest targets — a type that ignores a column leaves it null; sparse columns are cheap.
- **Column vs JSON, the dividing rule:** a real column when the attribute is stable, shared by several types, and queried/filtered/sorted; the indexed JSON `attributes` when it is rare, volatile, or single-type. Never a near-empty column for one minor type's two fields; expression-index the hot JSON key when it earns queries.
- **Validation per type, from data:** the boundary's rule-set resolves from the type lookup — one universal rule-set lets garbage in; hardcoded per-type rule classes drift from the lookup. The lookup declares, the boundary enforces.
- **Output per type:** resources expose only the fields the type declares; null type-columns never leak into payloads — the API of a visa never shows `stars`.
- **Behaviour per type = strategy map:** `capability → handler` resolved once, never a giant match smeared across services. Adding a type = its lookup row + its columns/JSON keys + (only if it brings a NEW fulfillment archetype) one strategy.
- **Hierarchy:** self-FK for parent↔children; a closure table for deep trees needing O(1) subtree reads (locations, nested categories) — same pattern, reused.
- **Pricing as satellites:** many prices per item (seasons, tiers, currencies) in the price tables, integer minor units, scope-unique — never a mutable `price` column pretending to be pricing.
- **Indexing for the real lists:** `(tenant_id, type, subtype)` leads the hot filters; `(tenant_id, category_id)`, `(tenant_id, parent)` for children; partial indexes per hot type+active; GIN on the JSON attributes.
- **Hunt these in review:** a one-type column queried by no one (→ JSON) · a unique missing the tenant scope · type logic as repeated matches (→ strategy map) · the list index missing type/subtype · one validation set for all types.
