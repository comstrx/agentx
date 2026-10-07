# Resilience

A server earns trust by how it behaves when things go wrong.

- **Every outbound call has a timeout.** No exceptions — HTTP, database, cache, queue, DNS. An unbounded wait is an outage multiplier.
- **Bounded retries with exponential backoff + jitter**, only on idempotent operations or under an idempotency key. Repeated failure trips a short circuit: fail fast, recover on probe, never hammer a dying dependency.
- **The local-first saga for third parties:** record intent locally as pending → call the provider → confirm via response or webhook → reconcile stragglers by schedule. Provider chaos never corrupts local truth; webhooks are verified, idempotent, and re-orderable.
- **Queues absorb the world:** anything slow, external, or bursty (mail, media, webhooks, reports, provider calls) leaves the request path. Jobs are idempotent, carry their full context (tenant, actor), and retry with backoff to a dead-letter end state that a human can inspect.
- **Graceful shutdown:** stop accepting, drain in-flight requests and jobs, release locks, then exit. Deploys and scale-downs must be invisible to clients.
- **Health honestly split:** liveness = the process runs; readiness = dependencies answer. A failing readiness pulls the instance from traffic without killing in-flight work.
- **Observability as a feature:** structured logs with a correlation id across request → job → provider call; metrics on the hot paths (latency p95, queue depth, error rate); a slow-query log that someone reads. You cannot fix what you cannot see at 3 AM.
- Degrade deliberately: cache the last-good read, serve partial data with a flag, queue the write for later — a designed degradation beats an honest 500 when the domain allows it.
- Concurrency correctness: locks around non-idempotent critical sections, optimistic version checks on contended rows, no shared mutable state without a guard.
