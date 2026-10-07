# Publishing

- **Events are facts, past tense, publisher-ignorant:** `order.paid`, `tenant.provisioned` — a statement of what happened, never a command to a specific consumer. The publisher must not know or care who listens; a consumer-shaped event couples the decision to its consequences.
- **Publish behind one facade with a swappable driver:** `Event::publish(name, payload, key)` — the broker (Redis, a queue, a log) is a driver; business code never names it. Keep the neutral interface even with one backend.
- **The envelope is uniform:** event id (unique, for dedup), name (namespaced `entity.verb`), occurred-at, actor/tenant context, correlation id, versioned payload. Consumers dedup by id and trace by correlation — every event tells its whole story alone.
- **Durability matches the stake:** in-process listeners for same-transaction concerns; broker delivery for cross-boundary concerns; **outbox** (event row committed with the state change, relayed after) the moment a lost event means corruption — money, provisioning, fulfillment. Fire-and-forget is only for events whose loss is tolerable by declared decision.
- **Emit after commit:** an event describing uncommitted state is a lie under rollback; dispatch hooks fire post-commit, always.
- **Consumers are idempotent and reorder-tolerant** — at-least-once is the delivery contract everywhere; handlers guard by event id or version, never by faith in ordering.
- **Payloads are contracts:** additive evolution only, unknown fields ignored by consumers, no retyping/repurposing fields; a breaking payload change is a new event version with a migration window.
- Events carry identity + the deltas consumers need — not entire aggregates (consumers fetch or project what more they need), and never secrets.
- One catalog: every event name, schema, and emitter documented in one discoverable place — an event nobody can find is a side effect nobody can audit.
