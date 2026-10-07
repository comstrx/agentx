# Scaling

Scaling the monolith itself — the cheapest architecture to scale when it is built stateless.

- **Stateless processes first.** No request state in globals, statics, or process memory that outlives the request; sessions, cache, locks, and queues live in external stores. A stateless monolith scales horizontally by adding replicas behind a load balancer — that alone covers most real traffic stories.
- The offload ladder, in order of cheapness:
  1. **Cache** hot reads (with tenant/user-scoped keys and real invalidation).
  2. **Queue** everything slow or external (mail, media, webhooks, reports) — the request path does only what the response needs.
  3. **Read models / denormalized projections** for expensive aggregations, refreshed by events or schedule.
  4. **Database**: indexes matched to real queries, keyset pagination, connection pooling, then read replicas.
  5. Only after all of the above: split the hottest seam into its own service — evidence-driven, never fashion-driven.
- **Measure before optimizing.** Every scaling decision starts from a number: slow-query log, profiler flame, queue depth, p95 latency. A claim without a measurement is a guess.
- Background pressure is isolated: workers scale independently of web processes; one hot queue gets its own worker pool instead of starving the rest.
- Concurrency correctness before concurrency speed: idempotent jobs, distributed locks around non-idempotent sections, bounded retries with backoff, optimistic locking on contended rows.
- Big datasets stream: chunked reads, cursor iteration, bounded memory — the monolith must never load a table into RAM to "process" it.
- Deployment stays boring: one artifact, rolling restart, health checks, graceful drain. The monolith's superpower is operational simplicity — protect it.
