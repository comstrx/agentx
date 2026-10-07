# Performance

The budgets and the levers — measured on real devices, enforced at build time.

- **The budget:** LCP < 2.5s on mid-range mobile, CLS ~0, INP imperceptible, initial client JS lean and accounted for. Field data (real-user metrics) outranks lab scores; optimize the measured worst, re-measure, stop.
- **The biggest lever is the boundary:** every `"use client"` pushed one level down removes its subtree from the bundle. Audit with the bundle analyzer per route; a heavy library in a client component serving static content is the classic self-inflicted wound.
- **Code-split the heavy tail:** dynamic import for below-the-fold widgets, editors, charts, and modals (`ssr: false` only when the DOM is truly required); the route's first paint never pays for what the user might open later.
- **Images decide LCP:** the framework image component always — sized, responsive, modern formats, `priority` on the hero only, lazy below the fold, remote patterns whitelisted. An unoptimized hero image is a failed budget on its own.
- **Fonts without flash or shift:** the font pipeline (self-hosted, subset, `display: swap`, fallback metrics tuned), two families maximum, variable fonts where weights multiply.
- **Streaming as perceived speed:** shell first, Suspense regions painting as data lands, skeletons matching final layout — the page *feels* ready before it is complete, at zero CLS cost.
- **Prefetch intent:** viewport-visible links prefetch by default — keep it; add hover/press prefetching on the money paths (list → detail) and prefetch the query cache alongside where a client table owns the data.
- **Re-render hygiene where it is measured:** memo/stable-props on hot lists, virtualize long tables, keys stable, context split so an updating value does not repaint the world. The profiler decides — never decorative memoization.
- Third-party scripts are guests: loaded via the script component with the right strategy, deferred, and audited — analytics must never cost more than the feature it measures.
