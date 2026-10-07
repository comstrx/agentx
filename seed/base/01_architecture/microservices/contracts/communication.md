# Communication

- **Async by default, sync by exception.** Events over direct calls wherever the caller does not need the answer to respond; every synchronous call chain is a coupled failure chain — three services deep means three availabilities multiplied.
- **Events are facts, not commands:** `order.paid`, never `send-invoice-to-billing`. The publisher states what happened and stays ignorant of consumers; consumers subscribe and react. A command-shaped event couples the publisher to a consumer's job.
- **The outbox pattern is mandatory for durable publishing:** the event row commits in the same local transaction as the state change, a relay publishes it after — a state change whose event got lost is corruption at a distance.
- **Consumers are idempotent and reorder-tolerant:** at-least-once delivery is the contract; duplicate and out-of-order events are certainties. Guard with event ids, version checks, or upsert-shaped handlers — never assume exactly-once.
- **Synchronous calls carry a survival kit:** timeout, bounded retries with jitter on idempotent calls only, circuit breaker, and a designed fallback (cached read model, degraded response, queued retry). A bare RPC call in production code is a defect.
- **The request context travels whole:** correlation id, actor, tenant, locale — propagated through every hop and every event envelope, so one user action traces across the whole system as one story.
- **Schema evolution rules:** consumers ignore unknown fields (tolerant reader), producers never repurpose or retype existing fields, deprecations announce and outlive their last consumer.
- **No service-to-service chatter for what a read model solves:** a consumer needing another service's data on every request materializes a local projection from that service's events — query-time joins across the network are the N+1 of distributed systems.
- API gateway / BFF is a translation edge (auth, shaping, fan-out), never a business-logic landfill.
