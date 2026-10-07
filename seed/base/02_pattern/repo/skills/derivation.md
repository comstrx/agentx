# Derivation

Derive, never hand-wire — the craft that turns declarations into a full surface.

- **Declared relations → discovered map.** The model declares its relations once as normal methods; the engine discovers them by reflection at boot, caches the map safely, and derives from it: eager-loading, requested includes, and the nested-relation endpoints. You declare a relation once; you never wire it per endpoint.
- **Resource name → routes + permissions.** The resource segment drives the whole route set and the `view_/add_/edit_/delete_<resource>` permission quartet. Declare only the cross-cutting and special permissions; the engine derives the four.
- **Declared write-fields → the write shape.** The repository declares which input becomes which column; the engine maps request input through it. Undeclared input never reaches storage.
- **Declared columns → the filter/sort surface.** The request-derived filter DSL (`column@op` with in/notin/between/like/ranges, sort direction) is column-guarded: declare a column and filtering/sorting on it materializes; an unknown column is rejected, never passed through.
- One uniform read pipeline: index / show / statistics / related / export all build the same options struct (ids, text, page, limit, sort, filters, fields, scopes, permissions, callbacks) and funnel through one search call. New read features extend the struct once — every resource gains them simultaneously.
- N+1 dies structurally: auto eager-load from the discovered map plus requested includes, with a lazy-load tripwire in dev. Never fix N+1 per endpoint — fix the derivation.
- The derivation bar: **deterministic, inspectable, fail-closed.** Every derived behaviour can be printed (the map, the route set, the permission list) and every unknown key dies loudly. Magic the team cannot inspect is a bug factory.
- Anything hand-written twice per resource is a derivation candidate — hand-write it the second time only while extracting the rule that makes a third time impossible.
