# State

- **The form decides the server-state mode, declared once per project — never mixed per feature:**
  - **Web/public mode (RSC-first):** reads happen in Server Components and arrive as props; mutations go through server actions that revalidate what they changed; a client query cache appears only for genuinely client-driven regions (infinite feeds, live search) and never duplicates what RSC already fetched — the same entity fetched both ways is a drift bug.
  - **Panel/app mode (query-cache-first):** the client query cache (TanStack Query-shaped) owns ALL server state — every read a typed query keyed by resource + params, every mutation invalidating exactly what it changed, optimistic where the domain allows; RSC serves the shell and the session, not the data. The interaction-dense, permission-gated, always-mutating panel earns this mode; a public page does not.
- **In either mode the laws are identical:** one cache owns each entity, keys are structured (resource + params), invalidation is designed with the mutation, and no state duplicates the cache into components or stores.
- **The URL is the state manager for anything shareable:** filters, page, sort, tab, open modal on deep-linkable screens — `searchParams` in, links out. Refresh-safe and shareable by construction; local `useState` mirroring the URL is drift scheduled.
- **Global client stores approach zero.** Legitimate residents: theme, session identity snapshot, transient UI (sidebar, toasts). Server data in a global store is an anti-pattern here — the cache layers above already own it.
- **Forms go through server actions:** `useActionState` for pending/error state, validation errors returned as typed shapes and mapped inline to fields, the action revalidating on success. Client-only form state is for UX (dirty tracking, multi-step), never for the source of truth.
- **Optimistic updates with receipts:** `useOptimistic` (or the query cache's rollback) for instant feel — every optimistic write has a rollback path and reconciles with the server's answer.
- **Context is for composition, not state management:** theme, form scope, feature flags read-only — anything updating frequently through context is a re-render storm; restructure instead.
- Ephemeral state stays local (`useState` in the component that owns it); lifting state is a deliberate act with a named reason, not a habit.
- One direction: server truth → caches → components → actions → server truth. Any state flowing sideways between siblings without passing a cache or the URL is architecture debt.
