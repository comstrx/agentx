# Patterns

The engines a serious admin panel is built from — declare a resource, materialize its screens.

- **Table engine:** column definitions derive from the resource map (type → renderer: date, money, badge, relation link); server-driven pagination/sort/filter wired to the API's DSL; column visibility + saved views per user; bulk selection with gated actions; export through the same filtered query. One engine, every resource.
- **Form engine:** a field map (name, type, rules, options-source, dependencies) renders inputs, mirrors server validation, tracks dirty state, disables on submit, and maps API field errors back inline. Conditional fields declare their dependency; relation fields get async searchable selects with create-inline where permitted.
- **Detail engine:** a resource page derives its tabs from the relation map (orders of a user, items of an order), each tab an embedded gated table; header shows identity + status + primary actions.
- **Stats engine:** dashboards compose stat tiles and charts from a statistics endpoint, filtered by the same DSL (date ranges, scopes) — never client-side aggregation over raw rows.
- **Permission-aware rendering as one primitive:** a single `can(permission)` gate wraps buttons, columns, fields, menu entries, and routes; locked-from-above settings render faded with a lock. No scattered role conditionals — one gate, driven by the server's permission payload.
- **State discipline:** server data lives in a query cache keyed by resource + params (invalidated on mutation); UI state (filters, page, tab) lives in the URL so every screen is shareable and refresh-safe; global client state approaches zero.
- **Flow accelerators:** optimistic toggles for cheap booleans, inline row editing for single fields, command palette / keyboard nav for power users, toasts with undo where the domain allows soft-delete.
- Charts and tiles obey one visual system: shared palette, shared formatting (money, percent, compact numbers), shared empty/loading skeletons.
