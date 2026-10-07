# Components

- **A component is a pure function of its props until proven otherwise.** Presentation components take data and callbacks, render, and know nothing about where data lives; the containers/hooks above them own fetching and state. The prop boundary is the layer boundary.
- **Composition over configuration:** `children` and slots over prop-flags; compound components sharing their own context over a god-component with thirty props. A boolean prop that changes the whole shape (`isModal`, `asTable`) is two components fused.
- **Hooks are the engine layer:** reusable behaviour (data access, subscriptions, form logic, timers) extracts into `useX` hooks — the trait/mixin of this world; components stay declarative shells over them. Duplicated `useEffect` clusters across components are a hook not yet written.
- **State minimal and placed low:** local `useState` in the owner; lift only on a real shared need; derive instead of storing (a stored value computable from props/state is drift scheduled). Server data belongs to its cache layer, never mirrored into component state.
- **Effects are escape hatches, not architecture:** an effect synchronizes with an external system (DOM, subscription, timer) — it is not for transforming data (derive it), not for reacting to state with more state (compute it), not for data fetching where a data layer exists. Every effect states its dependencies honestly and returns its cleanup.
- **Keys are identity:** stable domain ids, never array indexes on anything that reorders or mutates — index keys are the classic silent state-swap bug.
- **Unidirectional flow, no backdoors:** data down, events up; a child mutating a parent's object prop, or two siblings synchronizing through refs, is architecture rot.
- **Error boundaries wrap features:** a failed widget degrades inside its boundary with a designed fallback — one component's crash never blanks the page.
- Rules of hooks are absolute (top level, no conditionals), exhaustive-deps is fixed not silenced, and a component stays small enough to read in one screen — extract by responsibility the moment it does not.
