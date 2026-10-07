# Design

- **The panel is derived, not hand-crafted per resource.** One schema/permission map drives menus, tables, forms, filters, and actions; adding a resource to the panel costs a declaration, not a new page tree. Hand-building the same table for the tenth resource is the defect.
- **Branch on permission, never on a panel literal.** One codebase serves every actor's panel (super, admin, vendor, staff, …); what differs is the permission set and scope, resolved per actor — `if panel == 'vendor'` is the same rot as `if type == 'hotel'`.
- **The UI is a hint; the server is the law.** A permission the actor lacks fades or hides its control, but the request is enforced server-side regardless. A locked-from-above setting renders read-only with its lock visible — never silently absent.
- **One interaction grammar everywhere:** list → filter/sort/search/paginate → bulk or row actions → detail with tabs → create/edit form → confirm destructive. A user who learns one resource has learned the whole panel.
- **Server-driven data:** tables page, filter, and sort on the server, mirroring the API's filter DSL one-to-one; the panel never re-implements query logic client-side or loads a table into memory to search it.
- Forms mirror the server's validation rules — same shapes, same messages — so a rejected write is a repeat of what the form already said, never a surprise.
- Destructive actions state their stake (what dies, how many), default to cancel, and stay auditable.
- Dense-data craft: readable at scale — sticky headers, saved views, column control, empty/loading/error states designed. Admin users live here 8 hours a day; speed and keyboard flow are features, not polish.
- The panel consumes the same public API as any client — zero private backdoors; if the panel needs data the API cannot express, fix the API.
