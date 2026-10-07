# Invalidation

The hard half of caching, done as engineering instead of folklore.

- **Tags are the invalidation index:** every cached entry registers under the entities it derives from (`product:{id}`, `products:list:{tenant}`); a write flushes its entity tag + the list tags it affects — one flush call at the write path, every derived shape dies together. Hand-enumerating keys at write sites does not scale past two shapes.
- **The write path owns the flush:** repository/engine persistence hooks fire invalidation centrally — creating/updating/deleting a model flushes its tags without any service remembering to. Call-site-remembered invalidation is the bug class; engine-fired invalidation is the cure.
- **Version keys where tags are unavailable:** a per-scope version counter in the key (`v{n}:tenant:{id}:products`) — bumping the version orphans the whole family atomically; the orphans expire by TTL. Cheap, lock-free, O(1) invalidation of arbitrary families.
- **Choose staleness per shape, explicitly:** decision-critical reads (balances, permissions at enforcement) skip the cache or read-through with short TTL; display reads tolerate minutes; third-party catalog data tolerates hours with background refresh. Write the tolerance down in the key builder — it is domain knowledge, not a tuning knob.
- **Stale-while-revalidate for the popular:** serve the stale copy, refresh in the background under a per-key lock — users never wait on recomputes, herds never form. Reserve hard misses for shapes too costly to serve stale.
- **Event-driven invalidation across boundaries:** when another service/process owns the write, subscribe to its events and flush locally — polling for freshness is the N+1 of caching.
- **Test invalidation as behaviour:** write → read → mutate → read must yield the fresh value in the suite; a cache bug found by a customer is a missing four-line test.
- Debug with the key inventory: reproduce, inspect the exact key (exists? TTL? scope right?), then the tag registration, then the flush firing — staleness bugs are always one of those three, in that order.
