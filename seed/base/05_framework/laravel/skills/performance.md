# Performance

The measured-speed playbook — Octane gives the floor, discipline keeps it.

- **Measure first, always:** slow-query log + Horizon metrics + p95 latency are the truth; a performance claim without a number is a guess. Optimize the measured worst, re-measure, stop.
- **The caching ladder, tenant-scoped keys at every rung:** derived maps in immutable statics (paid once at worker boot) → read-pipeline results via `remember()` with tag invalidation → HTTP-level caching only for genuinely public payloads. Invalidation is designed with the cache, never bolted on: the write path fires the tags, the read path never serves a stale actor-scoped row.
- **Indexes match real queries:** composite indexes lead with `tenant_id` then the filter/sort columns the DSL actually sends; partial indexes for hot statuses; covering indexes for list projections. Every new index names the query it serves; unused indexes are dropped — writes pay for them.
- **Keyset everywhere growth lives:** offset pagination degrades linearly; keyset (`whereUuid > cursor` on UUIDv7's time-ordering) stays constant. UUIDv7 exists precisely so ids sort by time — exploit it for cursors and hot-partition locality.
- **The request path is lean by law:** no outbound HTTP, no mail rendering, no media work inline — the queue absorbs it. What remains: validate, decide, commit, respond; everything else is an event.
- **Octane-aware hot paths:** exploit the warm boot (config/route/map caches resolved once), avoid per-request container churn for stable services, and keep middleware cheap — it runs on every request of a worker that serves thousands.
- **Redis discipline:** logical DBs split by concern, pipelining for multi-key beats, no `KEYS` ever (`SCAN`), TTLs on everything, tenant-namespaced keys sized for memory math.
- **Payload hygiene:** explicit `select` on wide tables, resources shaping only what the client uses, gzip/brotli at the edge, counts via `withCount` not loaded relations.
- Load-test the endpoints that earn money before they meet traffic — a booking flow's p95 under concurrency is a launch gate, not a curiosity.
