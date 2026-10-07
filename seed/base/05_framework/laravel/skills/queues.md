# Queues

Horizon-driven background architecture — the request path does only what the response needs.

- **Everything slow or external leaves the request:** mail, media processing, webhooks, provider calls, reports, search indexing, notifications fan-out. The response returns when local truth is committed; the world catches up asynchronously.
- **Every job carries its context:** `tenant_id` and actor stamped at dispatch, restored at job start, reset at job end — a job body reads context exactly like a request body does; the queue Support domain owns the stamp/restore/reset cycle so no job hand-rolls it.
- **Idempotent by construction:** a redelivered job produces the same end state — guard with unique job ids, idempotency keys, or upsert-shaped writes; `ShouldBeUnique` where one-in-flight is the rule. Assume at-least-once delivery always.
- **Retry topology deliberately:** `tries`/`backoff` tuned per job class (exponential with jitter for provider calls), `retryUntil` for time-boxed relevance, `failed()` handling that records enough context to replay by hand; the dead-letter queue is monitored, not a landfill.
- **Dispatch after commit:** jobs touching data dispatched inside transactions use after-commit semantics — a job racing its own uncommitted transaction is the classic heisenbug.
- **Queue topology by pressure:** separate named queues for latency-sensitive (notifications), heavy (media), and bulk (reports) work; Horizon supervisors scale each independently; one hot queue never starves the rest. Balance strategy and worker counts derive from measured depth, not guesses.
- **Batches and chains for workflows:** `Bus::batch` with `then/catch/finally` for fan-out aggregation, chains for strict ordering — never jobs polling each other's side effects.
- **The saga rides the queue:** local-pending → provider job (retried, idempotent) → confirm via webhook job (verified, idempotent, re-orderable) → scheduled reconciler for stragglers. Provider chaos never corrupts local truth.
- Broadcast events that must feel live go `ShouldBroadcastNow`; everything else respects the queue — the latency budget decides, per event.
