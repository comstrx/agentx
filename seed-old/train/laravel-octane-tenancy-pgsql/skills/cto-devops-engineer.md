# Skill — CTO / DevOps for this stack (Octane · Horizon · Reverb · Postgres RLS)

Shipping and operating the archetype in production. Stack-specific ops craft, not generic advice. Pairs with
`laravel-octane.md` (correctness) and `tenancy-playbook.md` (isolation).

## Runtime — Octane + FrankenPHP
- **One artifact, promoted across envs** — build the image once; config comes from the environment, never
  baked in. Run FrankenPHP as the server/worker.
- **Worker recycling:** set `max_requests` so workers recycle periodically — bounds slow memory leaks from
  accumulating state (`laravel-octane.md`). Size worker count to CPU, not request volume.
- **Zero-downtime reload:** drain in-flight requests before a worker is replaced; expose health + readiness
  endpoints; rolling deploy. A reload must never drop a request mid-flight.
- **State hygiene is an ops concern too:** a leak that's invisible in dev OOMs a long-lived worker in prod —
  monitor worker memory and request count.

## Queues — Horizon (Redis)
- Horizon supervises Redis queues; run it as a managed process with its own dashboard (auth-gated, never
  public). Scale workers per queue by depth.
- **Jobs are tenant-stamped and idempotent**, bounded retries with backoff, a dead-letter path (`support/queue`).
  A redelivered job must be safe.
- Separate queues by priority (e.g. `mail`, `default`, `payments`); mail is always queued, broadcasts are not.

## Realtime — Reverb
- Reverb behind the proxy for WebSockets; broadcasts are immediate (`ShouldBroadcastNow`). Channel
  authorization is tenant + role aware (presence/private channels) — a channel leak is a tenant leak.

## Data & migrations — Postgres 16 + RLS ops
- **The RLS role split is an ops setup:** the app connects as a **non-owner** role; the owner role (migrations)
  is separate. Owners bypass RLS — never run the app as the owner.
- Enabling RLS lives in migrations: `ENABLE` + `FORCE ROW LEVEL SECURITY` + the tenant policy, created
  additively. Grant the app role only what it needs.
- **Migrations:** reversible, additive-first; run as a gated step before the new code goes live; no destructive
  drop in the same release as code that still reads the column. Test rollback, not just roll-forward.
- Connection pooling sized to worker concurrency; the RLS GUC is transaction-local (`tenancy-playbook.md`).
- **Backups are tested by restore, not by existence.** Know RPO/RTO before the incident.

## Caching, locks, throttling — Redis (logical DBs)
- Keep Redis logical DBs separated by concern (cache / queue / horizon / reverb / session / rate_limit / lock).
- All keys tenant-prefixed; tag/index-based partial invalidation so a write refreshes only the affected tag,
  never a full flush.

## Security & observability
- **Secrets** in a manager (Vault / SSM / sealed secrets), injected at runtime; rotate without redeploy;
  nothing sensitive in repo, image, or logs (`support/log/Redact`).
- **Structured JSON logs** with a tenant + request correlation id on every line; no PII, no secrets.
- **Metrics first:** request latency/error rate per endpoint, queue depth + job failure rate, DB pool
  saturation, worker memory. Alert on symptoms (latency, errors), not causes.
- Authenticate and rate-limit every public endpoint; least privilege for DB roles and service accounts.

## The ops failure modes
App running as the DB owner (RLS silently bypassed) · session-level RLS GUC on a pooled connection (tenant
bleed) · workers never recycled (slow-leak OOM) · public Horizon/Telescope dashboard · destructive migration
shipped with code still reading the column · backups never restore-tested · secrets in the image or logs.
