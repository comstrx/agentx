# Rendering

- **Choose the rendering mode per route, deliberately:** static for marketing/docs, ISR (`revalidate`) for catalog-shaped content, dynamic for actor-scoped screens — and say why in the PR. "Whatever it defaulted to" is not a strategy.
- **Caching is explicit, never assumed:** fetches are uncached unless you opt in — every data read declares its cache life (`revalidate`, tags, or per-request). An actor-scoped or tenant-scoped response must never be cached across actors; a public response must never be refetched per request.
- **Invalidate by tag, not by prayer:** tag every cached read (`next: { tags: [...] }`), and every mutation calls `revalidateTag`/`revalidatePath` for exactly what it changed — cache invalidation is designed with the write path, the same law as the backend.
- **Stream the shell, suspend the slow:** layouts and above-the-fold render immediately; every slow data region sits in its own `<Suspense>` with a skeleton matching final layout (zero shift). One slow query must never hold the whole page hostage.
- **Parallel by construction:** independent fetches start together (component tree parallelism or `Promise.all`), request-deduplicated by the framework; a sequential await chain of independent data is the classic hidden 3x.
- **`loading.tsx` / `error.tsx` / `not-found.tsx` on every route group:** a failed widget degrades inside its boundary; a blank white screen is a rejected deliverable.
- **Params are async — await them;** dynamic APIs (`params`, `searchParams`, cookies, headers) are awaited per current framework contract, and touching them makes the route dynamic: know the cost when reaching for a cookie in a layout.
- **generateMetadata on every public page:** titles, descriptions, OG per entity; sitemap and robots generated from real data. SEO is a rendering concern here, not an afterthought.
- Preview/draft flows use the framework's draft mode — never a parallel "staging hack" render path.
