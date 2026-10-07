# Experience

- **Modern, fluid, fast — all three, no trade.** A public web surface is judged in its first two seconds: content visible immediately, interaction ready without jank, motion that guides instead of decorates.
- **Performance budget is a contract:** LCP under 2.5s on mid-range mobile, CLS at ~zero, interaction latency imperceptible. JavaScript is the most expensive byte — ship the minimum, defer the rest, measure on real devices, not dev machines.
- **Design system, not ad-hoc styling:** tokens for spacing, type scale, palette, radii, shadows in one place; components consume tokens; a redesign is a token change, not a hunt. Light and dark are first-class from day one.
- **Motion with purpose:** entrance choreography, hover/press feedback, layout transitions — smooth, brief, physics-natural, and honoring `prefers-reduced-motion`. Animation that delays the user is a defect.
- **Accessibility is non-negotiable:** semantic HTML first, full keyboard flow, visible focus, labelled controls, contrast that passes. A mouse-only interface is a broken interface.
- **Responsive is mobile-first:** the phone layout is designed, not squeezed; touch targets sized; typography fluid; images art-directed per breakpoint.
- **SEO structurally:** server-rendered content, one h1, meaningful titles/meta/OG per page, canonical urls, structured data where the domain has it. Content that requires JS execution to exist does not exist for half the crawlers.
- **Every state designed:** loading (skeletons over spinners), empty (with the next action), error (with recovery), offline where it matters. The unhappy paths are most of the user's memory.
- Forms respect the human: validate inline on blur, keep everything typed on failure, label errors next to their field, never clear a form.
- Trust surfaces are visible: clear pricing, honest CTAs, no dark patterns, fast legal pages — conversion built on clarity, not tricks.
