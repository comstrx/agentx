# Architecture

- **The lockfile decides the world:** Expo or bare, navigation library, state/query layers — read `package.json` before assuming any API shape; the ecosystem shifts faster than memory. New Architecture (Fabric) era assumptions only when the project's versions say so.
- **One typed navigation map is the spine:** stacks, tabs, and modals declared once with typed params; screens navigate through the map's types — a stringly `navigate('Detail', {...})` with untyped params is the crash that ships. Deep links and push notifications resolve through the SAME map.
- **Screens are thin; features own the logic:** a screen composes feature components and hooks — the features/components/hooks split holds exactly as on web; React component law and hook craft apply unchanged (the `react` node is the base layer of this one).
- **The platform split is surgical:** shared by default; `.ios/.android` files or platform selects only where conventions genuinely diverge (haptics, headers, permissions copy) — OS conditionals sprinkled through business logic are the mobile flavour of the type-literal rot.
- **The bridge/JSI boundary is a budget:** heavy data crossing per frame, giant JSON payloads, chatty native calls — all measured costs; animations on the native driver (Reanimated worklets/`useNativeDriver`), gestures through the gesture library, never JS-thread-driven frame work.
- **Server cache + mutation queue is the data law:** reads from a query cache (refetch on focus/reconnect), writes through a persisted offline queue, UI reads only the cache — the offline-first contract of the mobile form, implemented here.
- **Storage tiers deliberately:** secure storage for tokens, fast key-value (MMKV-shaped) for preferences/cache, SQLite for real relational offline data — AsyncStorage for anything hot or sensitive is a defect.
- **Config and secrets per channel:** environment-driven config resolved at build/OTA time, never hardcoded; API base/tenant resolution mirrors the backend's domain rules.
- Error boundaries per navigator + a crash reporter wired from day one — a silent native crash with no trail is an unfixable bug.
