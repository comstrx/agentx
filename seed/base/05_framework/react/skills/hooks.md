# Hooks

The hook craft — behaviour as a composable engine.

- **Design hooks like APIs:** a `useX` returns a small named surface (`{ data, status, actions }`), hides its machinery, and composes from smaller hooks — the same shell-and-engine shape; a component using five raw primitives in a tangle is one custom hook in disguise.
- **The state-shape ladder:** `useState` for independent values → `useReducer` when transitions have rules (the state-machine instinct: events in, states out) → context when a subtree shares it → external store/cache only when the tree cannot own it. Climb only when forced.
- **`useReducer` for lifecycles:** multi-step flows (forms, wizards, upload pipelines) modeled as explicit states + actions make illegal transitions unrepresentable and the flow testable as a pure function — the domain state-machine law in miniature.
- **Referential stability where it is measured:** `useCallback`/`useMemo` stabilize what crosses a memoized boundary or a dependency array — not decoration on every function; profiler first, memo second. `memo` on hot list rows with stable props is the one habitual exception.
- **Context without re-render storms:** split state and dispatch into separate contexts, keep values memoized, keep fast-changing values OUT of context entirely — a context ticking every second repaints its whole subtree.
- **Refs are for identity, not data flow:** DOM handles, latest-value escape hatches in long-lived callbacks, imperative instance state that must not re-render — a ref carrying render data is state hiding from React.
- **Subscription hooks own their whole lifecycle:** subscribe in the effect, return the teardown, key the dependency array by the real identity — `useSyncExternalStore` for stores that live outside React.
- **Async inside hooks respects unmount:** abort controllers wired through effects, state set only while mounted/current — the stale-closure race (slow response overwriting a newer one) is guarded by request identity, not hope.
- Test hooks as units where they carry logic (`renderHook`), through components where they carry wiring — a hook too tangled to test alone is two hooks.
