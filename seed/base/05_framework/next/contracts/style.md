# Style

- **Tailwind on a token system:** palette, spacing, radii, shadows, and type scale declared once as design tokens (CSS variables consumed by the Tailwind theme); components use semantic tokens (`bg-surface`, `text-muted`, `border-subtle`) — raw hex or arbitrary one-off values in class lists are defects. A rebrand is a token edit.
- **Dark mode is first-class from day one:** tokens defined per scheme, `class`-strategy switching, every component verified in both — a light-only component is half a component.
- **Variants over conditionals:** component appearance axes (size, tone, emphasis, state) are declared variant maps (cva-shaped), not ternary chains concatenating classes; `clsx`/merge utilities compose them safely. A boolean-prop explosion (`primary`, `small`, `danger`, …) is a variant map not yet written.
- **The primitives own the look:** buttons, inputs, cards, dialogs, tables exist once in `components/ui` and everywhere else composes them — a feature file styling its own button from scratch is a leak. Headless behaviour (focus, aria, keyboard) + house skin is the primitive recipe.
- **TypeScript strict in every component:** typed props (no `any`, no untyped spreads), variant types derived from the variant map, event handlers precisely typed. Components are the API of the design system — same rigor as any API.
- **Accessibility is part of the skin:** focus-visible rings on every interactive element, aria labelled by the primitive, contrast passing on both schemes, keyboard flow verified — baked into `components/ui` once so features inherit it.
- **Motion through one system:** durations and easings are tokens; micro-interactions (press, hover, enter) live in the primitives; page/list choreography uses the house motion library consistently and honors `prefers-reduced-motion`.
- Class strings stay readable: layout → spacing → color → state ordering, extracted to the variant map the moment they wrap twice; `@apply` is for genuinely global surfaces only.
- Fonts via the framework's font pipeline (subset, self-hosted, `swap`); icons from one set through one wrapper — never three icon packs in one bundle.
