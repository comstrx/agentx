# Craft

The build techniques behind a web surface that feels expensive.

- **Composition over configuration in components:** small components with slots/children over god-components with thirty props; variants as a first-class axis (size, tone, emphasis) instead of boolean explosions; behaviour hooks separated from presentation so a redesign never touches logic.
- **State has three homes and no more:** server data in a request/query cache (deduplicated, revalidated, invalidated on mutation), shareable UI state in the URL, ephemeral state local to the component. A global store is the last resort, not the default.
- **Loading is choreographed:** skeletons match the final layout (no shift), content streams in meaningful order (shell → primary content → secondary), optimistic updates make mutations feel instant with rollback on failure.
- **Images and fonts are engineered:** responsive sizes, modern formats, lazy below the fold, priority above it; fonts subset, preloaded, `swap`-displayed, and limited to two families. These two categories decide most of LCP.
- **Micro-interactions sell quality:** button press states, hover lifts, focus rings, animated numbers, staggered list entrances — each under 200ms, each interruptible, all driven by the same easing tokens.
- **Scroll craft:** sticky navigation that condenses, sections that reveal on entry, anchored deep links — never scroll-jacking; the wheel always belongs to the user.
- **Error and edge engineering:** retry with backoff behind the scenes before showing failure, stale-while-revalidate to hide network wobble, boundary components that isolate a failed widget instead of blanking the page.
- **Measure what users feel:** field metrics (real-user LCP/CLS/INP) over lab scores, error tracking with source maps, analytics on the funnels the business actually runs — then optimize the measured worst, not the guessed one.
