# Architecture

- **App Router, RSC-first:** every component is a Server Component until interactivity forces `"use client"` — and the directive lands at the interactive **leaf**, never on a layout or page wholesale. Client bundles carry only what genuinely needs the browser.
- **The layer stack under `src/`, imports pointing down only — `lib → hooks → components → features → app`:**
  - `app/` — routing shell only: layouts, pages, loading/error boundaries, route handlers. Pages compose features; raw HTML and `className` live below this layer.
  - `features/<name>/` — the real product code, one folder per domain: a feature assembles components + hooks + api + permissions for ONE domain, **exports exactly one component**, and **never imports another feature** — cross-feature need names a missing lower-layer capability.
  - `components/` — the design system, internally ordered `ui → custom → layout`: `ui/` primitives (headless base + house skin), `custom/` house composites over them (dropzones, pickers, charts), `layout/` shells. Zero business logic, zero data fetching — props in, pixels out.
  - `hooks/` + `lib/` — the foundation: generic hooks and the project's private std-lib (`lib/std` domains: str, list, date, net, parse, validate, …), no domain nouns; a lib module is born with a sibling test.
- **Cross-cutting modules ride beside the stack, consumed by it, never part of it:** `api/` (the typed endpoint registry + client), `stores/` (ephemeral UI state), `i18n/`, `proxy.ts`, `styles/` — single-purpose surfaces any layer may import, none importing upward.
- **Missing capability? Create it at the right layer first** (a lib module, a ui primitive, an api entry), then consume it — inlining lower-layer logic higher up is the violation the layer law exists to catch.
- **Panels are dynamic, not duplicated:** actor systems (`systems/super`, `systems/admin`, shared) resolve behind a catch-all route driven by the permission map — one codebase, every panel; branch on permission, never on a panel literal.
- **Colocation is law:** a component's styles, subcomponents, and tests live beside it; a thing used by one route lives in that route's feature, promoted to `components/` only by the second consumer.
- **Server/client boundary is explicit and audited:** server-only modules (db, secrets, heavy SDKs) are marked `server-only`; anything crossing to the client passes through serializable props or server actions. A secret reaching a client bundle is an incident, not a bug.
- Route handlers exist only for genuine HTTP surfaces (webhooks, OG images, exports); data for pages flows through RSC fetches and server actions, not self-fetching from your own API.
- Middleware/proxy at the edge stays thin: auth gate, tenant/locale resolution, redirects — never business logic.
