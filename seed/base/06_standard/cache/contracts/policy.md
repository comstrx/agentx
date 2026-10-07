# Policy

- **Cache behind one facade with a swappable driver** — callers speak the DSL (`remember`, `get/set`, `forget`, tags), never a backend client directly. The backend is config; swapping it is one driver file.
- **Every key is namespaced and owned:** `tenant:{id}:resource:{shape}` — tenant/actor scope baked into the key builder, never left to call-site discipline. A cache key built by string concatenation at a call site is a collision waiting.
- **Every entry has a TTL.** Immortal keys are memory leaks with a title; even "permanent" derived data gets a long TTL as the safety net under explicit invalidation.
- **Invalidation is designed with the write, never bolted on:** the write path fires the tags/keys it dirties in the same change that introduces the cached read. A cached read merged without its invalidation story is an incorrectness, not an optimization.
- **Cache aside, fail open on reads:** a cache miss or a cache outage degrades to the source of truth with a log — the cache is an accelerator, never a dependency the request dies on. Writes never depend on the cache succeeding.
- **Never cache across actors what was computed for one:** permission-shaped, tenant-shaped, or user-shaped payloads carry that scope in the key or do not enter the cache. The cross-actor leak is a security incident, not a staleness bug.
- **What earns caching:** measured hot reads, expensive derivations, third-party responses with tolerable staleness. What never does: uncommitted state, secrets/tokens beyond their natural store, anything whose staleness the domain cannot tolerate (balances at decision time).
- **Stampede protection on expensive keys:** lock-per-key recompute or stale-while-revalidate — a popular key expiring must not become a thundering herd against the database.
- The cache is observable: hit ratio and hot keys measurable; a cache nobody can inspect is a superstition layer.
