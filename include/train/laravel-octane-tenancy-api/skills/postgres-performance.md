# Skill — PostgreSQL performance, indexes & extensions

PostgreSQL 16 is the data layer. This is the craft of making it fast and correct under multi-tenancy: pick the
right index, lead with `tenant_id`, prove it with `EXPLAIN`, and reach for the right extension.

## Index types — and when each
- **B-tree** (default): equality, range, `ORDER BY`, `LIKE 'prefix%'`. The workhorse.
- **Composite**: a multi-column B-tree. **Leftmost-prefix rule** — `(tenant_id, status, created_at)` serves
  `WHERE tenant_id=? AND status=?` and `WHERE tenant_id=?`, but NOT `WHERE status=?` alone. **Lead every hot
  index with `tenant_id`** — every tenant query filters by it.
- **Partial**: `... WHERE deleted_at IS NULL` — indexes only live rows; smaller, faster, ideal for soft-deletes
  and `status='active'` hot paths.
- **Covering** (`INCLUDE (...)`): lets an index-only scan return extra columns without a heap fetch.
- **GIN**: `jsonb`, arrays, full-text, and `pg_trgm` fuzzy/`ILIKE` search.
- **GiST**: ranges, geometry, exclusion constraints.
- **BRIN**: huge, naturally-ordered append-only tables (time-series) — tiny index, great for range scans.

## Multi-tenant indexing
- Composite uniques are `unique(tenant_id, …)` — a slug is unique *within* a tenant, not globally.
- Every list/filter index leads with `tenant_id`; then the filter column, then the sort column.
- `jsonb`: GIN index for containment (`@>`); for one hot key, an expression index `((data->>'k'))` beats a
  whole-column GIN.
- **Uniqueness is per-tenant:** `unique(email, tenant_id)`, **never** `unique(email)` — the same email
  registers across tenants. Every business unique includes `tenant_id`.
- **Single-table polymorphism** (`products`): `(tenant_id, type, subtype)`, `(tenant_id, product_id)` for
  children; partial indexes per hot type (`WHERE type='hotel' AND active`); GIN on `jsonb attributes`.
- **Hierarchies** (locations, product trees): a closure table (`ancestor_id, descendant_id, depth`) beats
  recursive CTEs for read-heavy subtree queries.

## Pagination & N+1
- **Keyset, not OFFSET.** `WHERE (created_at, id) < (?, ?) ORDER BY created_at DESC, id DESC LIMIT n`. OFFSET
  scans and discards N rows — death on large, deep pages. Keyset is O(log n) via the index.
- **N+1** is a defect: auto eager-load from discovered relations + requested includes, with
  `preventLazyLoading()` as the dev tripwire (`design.md §7`). A list endpoint issuing per-row queries fails review.

## Proving it — `EXPLAIN (ANALYZE, BUFFERS)`
- **Seq Scan** on a large table in a hot query = a missing/unused index.
- Estimated rows wildly ≠ actual rows → stale stats (`ANALYZE`) or a bad predicate.
- Watch for the index you expect being ignored (see gotchas).

## Index gotchas that bite
- **Type mismatch kills the index** — comparing `uuid` to a `text` literal, or `bigint` to `text`. Match types
  (ids are `uuid`).
- **A function on the column kills the index** — `WHERE lower(email)=?` won't use a plain index; create an
  **expression index** `((lower(email)))` or use `citext`.
- **Low-selectivity index is useless** — indexing a boolean with 50/50 split; the planner ignores it.
- **Too many indexes slow writes** — every index is maintained on every write. Index what you query, not
  everything.
- **Composite column order matters** — wrong order = no leftmost-prefix benefit.

## Extensions worth knowing
- **pgcrypto** — `gen_random_uuid()`, `digest()`/`crypt()`. (App generates **UUIDv7**; pgcrypto for hashing/db-side.)
- **citext** — case-insensitive text; perfect for `email`, avoids `lower()` expression indexes.
- **pg_trgm** — trigram fuzzy match / fast `ILIKE '%x%'` via a GIN index; the basis for cheap in-DB search
  before you reach for Elasticsearch.
- **btree_gin** — mix scalar columns with `jsonb`/arrays in one GIN index (e.g. `(tenant_id, data jsonb)`).
- **pg_stat_statements** — find the actual slow/frequent queries in prod; tune from data, not guesses.

Reach for these through migrations and the `support/database` DSL — never raw SQL scattered in business code
(`design.md §1`).
