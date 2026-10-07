# Stack

- **Never assume versions or API shape from memory — `composer.json` + `composer.lock` win** over anything you think you know. Read them first; versions below are floors, the lockfile is ground truth.
- The house stack: **latest major PHP + latest major Laravel** on **Octane + FrankenPHP** (the runtime), **Horizon** (queues), **Reverb** (websockets), **Sanctum** (tokens). **PostgreSQL** (+ extensions) and **Redis** with logical DBs split by concern: default / cache / queue / horizon / reverb / session / rate_limit / lock.
- **First-party Laravel only; everything else is hand-built** behind our own Support abstractions: auth, RBAC, cache DSL, search, payments, events, storage keys, idempotency, throttling. **No new external libraries except extreme necessity** — the exceptions are crypto and money primitives, which are never DIY. Name the conflict if a new dep duplicates an existing capability.
- **Mail:** provider HTTP API transport (never SMTP), always queued (`ShouldQueue`), creds in `config/services.php`.
- **Storage:** one abstraction, the `s3` driver everywhere — real S3 in production, MinIO in dev (same driver, different endpoint). Object keys tenant-namespaced, private by default, signed temporary URLs for downloads.
- **Realtime:** Reverb; chat/live events broadcast immediately (`ShouldBroadcastNow`) — the deliberate contrast with mail, which is queued.
- **Queues:** Horizon over Redis; jobs carry `tenant_id` and restore/reset actor context. The Horizon dashboard is proxy-protected, not app-auth'd (headless API).
- **Read the config before claiming behaviour:** `octane.php`, `horizon.php`, `reverb.php`, `sanctum.php`, `mail.php` + `services.php`, `filesystems.php`, `cache.php`, `queue.php`, `database.php`, `session.php` — and `.env*` for shape (keys only, never print secret values).
- The gate: static analysis green with zero suppression, the project's test suite, and `route:cache`/`config:cache` compatibility — cacheability is part of every change's definition of done.
