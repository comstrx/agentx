# Delivery

Making events arrive, once in effect, whatever the network does.

- **The outbox loop in practice:** insert the event row in the business transaction → a relay polls/streams the table (ordered by the time-sortable id) → publishes to the broker → marks published. Crash anywhere = replay, and consumer idempotency absorbs the duplicates; the pair outbox + idempotent consumer IS exactly-once *in effect*, which is the only exactly-once that exists.
- **Idempotency implementation ladder:** natural idempotency (upsert-shaped handlers) → processed-event-id table checked in the handler's transaction → version/state guards on the aggregate (`apply only if status = pending`). Pick the cheapest that holds; record the choice in the handler.
- **Partition/order by entity key:** events of one aggregate route by its id so they arrive in order *per entity*; global ordering is never assumed. A handler needing cross-entity ordering is a workflow hiding — model it as a saga instead.
- **Retry topology per consumer:** transient failures retry with backoff + jitter; poison events dead-letter with full context after bounded attempts; the DLQ is monitored and replayable — a consumer that blocks its whole stream on one bad event is an outage machine.
- **Replay is a designed capability:** consumers rebuildable from the stream (projections) declare it and tolerate full re-runs; side-effecting consumers (mail, provider calls) guard replay behind their idempotency keys. Rebuilding a read model must be a command, not an archaeology project.
- **Sagas choreograph the long stories:** each step consumes a fact, does one local thing, emits the next fact; compensations are explicit steps for the reversible, reconciliation sweeps for the rest; a saga's state is queryable — "where is this order stuck" is a SELECT, not a log dive.
- **Lag is the health metric:** consumer offset lag and DLQ depth on dashboards with alerts — an event system's failures are silent by nature; only measurement makes them loud.
- Test the paths that matter: duplicate delivery, out-of-order pair, crash-between-commit-and-publish (outbox proves itself here) — the happy path already works.
