# Components

Component engineering — a design system that compounds instead of a pile that rots.

- **The primitive recipe:** headless behaviour (focus trap, aria, keyboard, positioning) + house skin (tokens, variants) = one primitive in `components/ui`, reused everywhere. Build on a headless base where the behaviour is hard (dialogs, menus, comboboxes); hand-roll where it is trivial (badges, cards).
- **Composition over configuration:** slots and `children` over prop-flags — `<Card><Card.Header/><Card.Body/></Card>` beats `<Card title icon footer …>`. Compound components share state through their own context; the consumer composes freely and the API stays small.
- **Variants as the only appearance axis:** a cva-shaped map declares size/tone/emphasis; the component exposes `variant` props typed from the map; `className` pass-through merges safely (tw-merge) for the rare local exception — which, seen twice, becomes a variant.
- **Server-friendly by default:** primitives render as Server Components when they carry no interactivity; the interactive core (`"use client"`) is isolated inside so a Button with a click handler does not drag a whole card tree into the bundle.
- **Forms as a feature engine:** one form field wrapper owning label/error/help layout, inputs registered through the house form hook, validation schema shared with the server action — a new form is field declarations, not layout plumbing. The same derived-not-handwritten law as everywhere.
- **Tables as the flagship composite:** column defs in, everything else materializes — sorting/filtering wired to the server DSL, virtualization past a threshold, permission-gated columns/actions, skeleton rows, empty states with actions. Ship it once in the design system; features declare columns.
- **Every state designed in the primitive:** loading, disabled, error, empty, focus, invalid — features get correct states for free; a feature hand-drawing a disabled style is a leak.
- Naming: components are nouns (`UserMenu`), hooks are `useX`, handlers are `onX` in props / `handleX` locally; files match their default export. One component per file; subcomponents live beside their parent until a second consumer promotes them.
