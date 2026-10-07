# Scopes

One controller, every actor — behaviour threaded as scopes, never forked as copies.

- A single unified controller serves all panels/actors of a resource. Behaviour differs by the **actor context** (role, tenant, ownership, capability) read from one request-scoped context accessor — never by parallel per-actor controllers.
- The controller assembles **default scopes from context** and threads `scopes / permissions / callbacks` down through the service into the repository, which applies them to the query. The thread is one-directional and explicit.
- **Non-strict for reads, strict for writes:** listing may relax to what the actor may see; mutating enforces the full owner/tenant constraints. A write path is always fail-closed — no scope match, no write.
- Scopes compose: tenant scope + ownership scope + status scope stack cleanly on one query. Each scope is a small named unit; a behaviour difference between actors should read as a different scope list, not a different code path.
- Permission checks ride the same thread: route middleware gates the action, the scope thread gates the rows, the field/write shape gates the columns. Three gates, one declaration source.
- The context accessor is the **single source of actor truth** — set once by middleware at the edge, read everywhere, reset at request end. Never a second way to ask "who is acting".
- Callbacks are the escape hatch for the rare per-resource read tweak: a named closure threaded with the scopes, applied by the engine — used sparingly, never a second query path.
- Adding an actor to the system = a new scope set + permission grants. If it requires touching controllers, the thread is broken somewhere — repair the thread, not the symptom.
