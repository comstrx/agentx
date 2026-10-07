# Craft

The engineering behind a mobile app that feels first-party.

- **Navigation as a typed map:** the whole app is one declared route graph (stacks, tabs, modals) with typed params; deep links and notifications resolve through the same map — one source of truth for "where can the user be".
- **List mastery decides the app:** virtualized lists with fixed-size hints where possible, memoized rows, image thumbnails at display size, shimmer placeholders, incremental pagination via keyset cursors. The feed is the product; treat it like a hot path.
- **Sync architecture:** a server-cache layer (stale-while-revalidate, background refetch on focus/reconnect) over a mutation queue (persisted, idempotent, replayed in order on reconnect, conflict-resolved server-side). UI reads only from the cache — never from a network call directly.
- **The platform split, disciplined:** shared logic and screens by default; platform-specific files only where the conventions genuinely diverge (navigation feel, haptics, permissions copy). Branch on platform capability, never sprinkle OS conditionals through business logic.
- **Media pipeline:** pick/crop/compress on-device before upload, upload resumable in the background with progress, render via cached CDN sizes — original bytes never travel twice.
- **Notification craft:** categories the user controls, deep links that restore context, badge discipline, silent pushes to nudge sync — every notification earns its interruption.
- **Release engineering:** over-the-air updates for JS-level fixes, store builds for native changes; staged rollouts with crash monitoring watching the first hours; feature flags to decouple release from launch.
- **Observe the field:** crash reporting with symbolication, performance traces on startup/navigation/list scroll, analytics on the real funnels — a mobile bug you cannot reproduce is diagnosed by telemetry or not at all.
