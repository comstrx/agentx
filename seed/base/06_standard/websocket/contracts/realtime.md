# Realtime

- **Realtime is a delivery lane, not a second source of truth:** the same domain events that drive everything else broadcast to sockets — a socket message is a notification of committed state, never a parallel mutation path. Writes always travel the normal API; the socket answers with the resulting event.
- **Channels are authorization surfaces:** private channels per actor (`user.{id}`), per tenant, per entity (`order.{id}`) — subscription requests authorize server-side with the same permission machinery as HTTP; channel names are namespaced by scope so cross-tenant subscription is unrepresentable, not just forbidden.
- **Latency budget decides the lane:** chat, presence, live counters broadcast immediately (the deliberate contrast with queued mail); heavy fan-out (a tenant-wide announcement) may queue — per event type, declared, never accidental.
- **Clients assume disconnection as the normal state:** auto-reconnect with backoff, and on reconnect a **catch-up read** through the normal API (since last-seen cursor) — missed-while-offline is solved by the source of truth, never by hoping the socket buffered. The socket accelerates; the API guarantees.
- **Messages are envelope-disciplined:** event name, payload, ids, occurred-at — same schema law as every event; payloads carry deltas + identity, clients hydrate details through the API when needed; secrets and other actors' private data never ride a shared channel.
- **Presence is ephemeral by design:** who-is-online lives in the socket layer's transient store with TTLs — never persisted as truth; the database is not a heartbeat monitor.
- **Scale is the broker's job:** socket servers stay stateless (subscriptions in the broker/pubsub layer), horizontally added like any web process; sticky state beyond the connection itself is a design smell.
- Observability: connection counts, channel fan-out sizes, delivery latency on dashboards; auth failures on private channels logged as the probe attempts they are.
