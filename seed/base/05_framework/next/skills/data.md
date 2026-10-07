# Data

The data-flow craft: server truth, instant feel.

- **Read pattern:** the page's Server Components fetch in parallel (tree parallelism, `Promise.all` for same-component fans), tagged for invalidation, deduplicated by the framework. Auth/tenant context resolves once at the layout boundary and flows down — never re-resolved per widget.
- **Mutation pattern:** a server action validates (schema at the boundary → typed shape), authorizes server-side, performs the write against the backend API, then `revalidateTag` exactly what changed and returns a typed result `{ ok, data | errors }`. The client maps field errors inline via `useActionState`; success paths navigate or let revalidation repaint.
- **Optimistic craft:** `useOptimistic` for the instant toggle/append with the server answer reconciling after; the rollback path is written with the optimism, not discovered in QA. Cheap booleans and counters are optimistic; money and bookings are not — they wait for the receipt.
- **Pagination shapes:** static-ish catalogs → ISR pages per cursor; interactive tables/feeds → a client query cache with keyset cursors (`useInfiniteQuery`-shaped), prefetching the next page on scroll intent.
- **Live data honestly:** websocket/poll only the regions that need it (chat, notifications), feeding the same cache the rest of the UI reads — never a parallel state universe for "live" versions of the same entity.
- **The API client is one module:** base URL, auth header, tenant header, error envelope parsing, and retry policy in one place; features call typed endpoint functions, never raw `fetch` scattered with duplicated headers.
- **Endpoints are a declared registry, not scattered strings:** a `resource(name, { schema })` factory materializes the CRUD entry set (action, method, path, required permission, response schema) and a registry composes them with the bespoke entries — the client-side mirror of declare→materialize; the entry's schema validates the response at the boundary, its permission key drives the `can()` gates, and a URL typed inline in a feature is a defect.
- **Errors flow as values:** the envelope's machine key maps to UX (toast, inline, redirect-to-login) in one translator; a raw provider error string reaching a user is a leak.
- Debounce the human (search-as-you-type through an abortable fetch), never the data; stale-while-revalidate hides network wobble — a spinner on refetch of visible data is a regression.
