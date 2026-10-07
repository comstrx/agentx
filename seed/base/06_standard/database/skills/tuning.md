# Tuning

Database performance as measurement-driven engineering.

- **The slow-query log is the work queue:** enabled with a real threshold, reviewed on rhythm — tuning starts from the measured worst query, not from a hunch. `EXPLAIN (ANALYZE, BUFFERS)` before and after every fix; the plan is the truth, the ORM is the suspect.
- **Index design follows the query, column order follows selectivity + usage:** equality columns first (scope leads), then range/sort columns; a composite index serves its left prefixes only. Partial indexes for hot subsets (`WHERE status = 'active'`), covering indexes (INCLUDE) for list projections that must never touch the heap, expression indexes for the lowercase/`->>'key'` lookups actually written.
- **Every index is a liability on writes:** creation names the query it serves; unused indexes (the stats tables tell) get dropped with the same rigor they were added.
- **Keyset beats offset always at scale:** cursor on the time-ordered id — constant cost forever; `LIMIT/OFFSET` past page fifty is a linear scan wearing pagination's clothes.
- **N+1 dies at the source:** eager-load derived from declared relations, aggregate with `withCount`-shaped queries, batch lookups into keyed `IN` sets — the query count of a list endpoint is constant regardless of rows, verified in dev with the query log.
- **Locks held short and ordered:** row locks (`FOR UPDATE`) around the contended few (wallets, stock), consistent acquisition order to kill deadlocks, `SKIP LOCKED` for queue-shaped tables, statement timeouts so a stuck query is an error, not an outage.
- **Transactions tight:** open late, commit early, never hold one across network calls; long work chunks by keyset with per-chunk commits.
- **Connection math done once:** pool size × replicas ≤ what the server actually serves; pools acquire with timeouts; a pool exhaustion is a clear fast error.
- Maintenance is scheduled truth: autovacuum tuned for the hot tables, bloat and replication lag on dashboards, `ANALYZE` after bulk loads — a database that is never observed degrades in silence.
