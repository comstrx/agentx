# contracts/tools.md — Stack, tools, libraries & the gate (LAW)

Never assume versions or API shape from memory. **`composer.json` + `composer.lock` win** over anything you
think you know; read them. Read `config/*` and `.env*` (keys only, never print secret values) before claiming
behaviour. Versions below are floors — the lockfile is ground truth. Re-read every turn.

## Stack
- **PHP `^8.5`** · **Laravel `^13`** · **Octane + FrankenPHP** (the runtime) · **Horizon** · **Reverb** ·
  **Sanctum**.
- **PostgreSQL 16** (+ extensions) · **Redis 8** (logical DBs split by concern: default / cache / queue /
  horizon / reverb / session / rate_limit / lock).
- Mail transport: Mailgun **HTTP API** (`symfony/mailgun-mailer` + `symfony/http-client`) — never SMTP.

## Subsystem tooling
- **Auth:** Sanctum, **stateless Bearer tokens everywhere** (SPA cookie mode breaks with custom domains).
  Every token is **bound to a tenant**; middleware asserts `token.tenant == domain.tenant`. Tenant resolution
  = subdomain + Bearer token. API prefix `/v1`.
- **Mail:** the `mailgun` mailer (`transport: mailgun`, creds in `config/services.php`); a `failover` chain
  may include it. **NEVER SMTP.** Mail is **always queued** (`ShouldQueue`).
- **Storage:** one `Storage` abstraction, the **`s3` driver everywhere** — AWS S3 in production, **MinIO in
  dev** (S3-compatible: same driver, only endpoint/creds differ). Object keys tenant-namespaced; private by
  default; signed `TemporaryUrl` for downloads.
- **Realtime:** Reverb; chat/live events broadcast **immediately** (`ShouldBroadcastNow`) — the deliberate
  contrast with mail, which is queued.
- **Queues:** Horizon over Redis. Jobs carry `tenant_id`, restore/reset tenant context (`support/queue/Tenant`).
  The Horizon dashboard is **web-only, no app auth** — protected by the proxy/nginx layer (headless API).
- **Domains/DNS:** Cloudflare + Vercel behind ONE swappable driver — buy / transfer / subdomain provisioning,
  queued and idempotent (`design.md §6`, `data.md`).
- **API contract:** **no OpenAPI** — the `routes/collections/` mirror is the live contract (`arch.md §8`).

## Config map (read the file before claiming how a subsystem is wired)
`config/octane.php` (FrankenPHP, RequestTerminated) · `horizon.php` · `reverb.php` · `broadcasting.php` ·
`sanctum.php` · `mail.php` + `services.php` (mailgun) · `filesystems.php` (s3/MinIO) · `cache.php` ·
`queue.php` · `database.php` (Postgres + Redis) · `session.php`. `.env*` for environment shape.

## Libraries policy
- **Build it ourselves. No new external libraries except in extreme necessity.** Auth, RBAC, cache, search,
  payments, events, storage, idempotency, throttling are **hand-built** behind our own abstractions
  (`design.md` §6). First-party Laravel only (Sanctum / Horizon / Reverb / Octane). Reuse what is already in
  the lockfile before reaching outward; name the conflict if a new dep duplicates an existing capability.
- **The only "don't roll your own" exceptions:** cryptography and money rounding/ledger primitives — use
  core / battle-tested code (`support/security` wraps crypto; `support/num/Money` does integer minor-unit math only).
- Never hand-edit `composer.lock`. Never add a dependency without a concrete, reported reason.

## Autoload
- **PSR-4** `App\ → app/` for normal classes (file name = class name).
- **Classmap** for the `index.php` folder convention in `app/Support` & `app/Traits` (`naming.md`). **Run
  `composer dump-autoload` after adding a new support/trait file** — classmap is static; the class will not
  resolve until you do. Ugly FQCNs stay hidden behind the Support facades.

## The gate (the real judge — never self-report)
- **Gate = Larastan/PHPStan at max level + `declare(strict_types=1)`** everywhere. A change is "done" only
  when the gate is green.
- **Never** suppress with `@phpstan-ignore`, baselines, casts-to-widen, or `any`-style widening — **fix the
  root cause.** A green gate obtained by suppression is a contract violation.
- **NO formatter.** The hand-style in `style.md` is intentionally not PSR-12; do not add or invoke one.
- **Tests join the gate as the spine stabilizes.** While the foundation still churns, do not write speculative
  tests against it (test maintenance on a moving skeleton is wasted effort); once the spine is stable, tests
  become a gate alongside the static analyser. The tenant-isolation cross-tenant check is the first test that
  earns its keep.

## Ground-truth checklist (before claiming any behaviour)
1. `composer.lock` for the exact installed version of any package you touch.
2. `config/<area>.php` for how a subsystem is wired (mail, cache, queue, sanctum, filesystems…).
3. `.env*` for environment shape (read keys; **never print or commit secret values**).
4. If a dependency ships docs under `vendor/*/`, read them before using an unfamiliar API.
