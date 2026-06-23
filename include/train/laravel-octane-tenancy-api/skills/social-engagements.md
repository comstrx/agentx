# Skill — social engagements & the polymorphic morph engine

How ANY model gains a full social/engagement surface — like · dislike · view · comment · reply · review ·
favorite · report · share · file · image · qrcode · audit-log · notification — by `use`-ing one trait, with
ZERO per-model wiring. The capability that makes `Product`, `Blog`, `Order`, even a `Comment` itself instantly
engageable. Built on polymorphic morph tables + the Support DSL; every action gated by the permission ladder
(`rbac-permissions.md`); heavy fan-out queued.

## Where it lives
A dedicated trait family under **`app/Traits/Dna/Social/`** (kept apart from the rest of the DNA because it is
a large, cohesive subsystem): one focused trait per capability — `HasLikes`, `HasViews`, `HasComments`,
`HasReplies`, `HasReviews`, `HasFavorites`, `HasReports`, `HasShares`, `HasFiles`, `HasLogs`, `HasNotifications`
— plus a `HasSocial` facade trait that composes the common set. A model opts into exactly what it needs
(`use HasLikes, HasComments;`) — and that IS the whole integration. Each trait brings its own relations,
scopes, counters, boot hooks, and a tiny config surface (`design.md §5`).

## The morph backbone — ONE table per engagement kind
There is NO single "engagements" table. Each kind is its OWN polymorphic morph table — `views`, `likes`,
`favorites`, `comments`, `replies`, `reviews`, `reports`, `shares`, `files`, `logs`, `notifications` — each
attaching to any model via `(engageable_type, engageable_id)` + `tenant_id` + `user_id` (the actor), each
indexed `(tenant_id, engageable_type, engageable_id)`, each carrying exactly that kind's own fields
(`reviews.rating`, `reports.reason`, `comments`/`replies` body, `files` path/mime/size + a `kind` for
image/qrcode). One trait per table (`HasLikes` ↔ `likes`, `HasComments` ↔ `comments`, …).
- **`likes` is ONE toggle table, not two kinds:** one row per `(tenant_id, likeable_type, likeable_id,
  user_id)` with a `like` boolean — `true` = like, `false` = dislike. They are **mutually exclusive**; toggling
  flips the boolean and adjusts both counters. There is **no `dislikes` table**.
- **`views` / `favorites`** are presence toggles — one row per actor per target,
  `unique(tenant_id, *_type, *_id, user_id)`; a view also bumps the `views` counter.
Polymorphism + `tenant_id` on every row + the closure/locations tie (`polymorphic-catalog.md`) powers
recommendations ("users who liked X", "popular in this city") without a relationship table per pair.

## Counters done right (denormalized, consistent)
Hot aggregates (`likes`, `views`, `comments_count`, `rating_avg`) live as columns on the engageable, guarded by
`hasColumn` so a model without the column simply skips the counter. A toggle updates the morph row AND adjusts
the counter atomically (increment/decrement under the same path), never a recount on read. The truth is the
morph table; the counter is a maintained cache of it.

## Every action is gated and lifecycle-aware
- **Gate first:** each method asserts the permission via the ladder — `hasOrFail('allow_comments')`,
  `allow_likes`, `allow_reviews`, … — so engagement is governed by the same RBAC (`rbac-permissions.md`); a
  tenant or `super` can disable a capability per-entity or per-item by toggling the `allow_*` permission.
- **Emit events:** a new engagement runs the model boot hook (`created`) and publishes a domain event through
  the events abstraction (`design.md §6`) — feeding notifications, counters, and realtime.
- **Audit log:** `HasLogs` records `(event, changes-diff, actor, ip, agent)` on lifecycle transitions — the diff
  comes from the model's dirty attributes, never a hand-built payload.
- **Notifications:** `HasNotifications` creates the record, fans out recipients into a pivot via a bulk insert,
  attaches files, and **queues** delivery (`ShouldQueue`) + broadcasts realtime now (`ShouldBroadcastNow`,
  Reverb) — the queued/immediate split of `stack.md`.

## Files, images, qrcodes
`HasFiles` gives upload/attach/detach/`TemporaryUrl` over the `s3` Storage driver, keys tenant-namespaced and
private (`stack.md`); images and generated qrcodes are the same morph with a `kind`. No raw `Storage::` in
business code — it flows through `support/storage` (`design.md §1`).

## Octane & tenancy (non-negotiable)
Every engagement row carries `tenant_id` (auto-filled by `HasTenant`); cross-tenant engagement is a leak
(`tenancy-playbook.md`). The actor comes from `Context::userId()` — NEVER a request-bound global helper
(Octane-unsafe). Counters and morph writes under concurrency take the same care as money (`saas-domain.md`):
a like-toggle race must not double-count — guard the read-modify-write.

## The payoff & the failure modes
Payoff: `use HasSocial;` turns a bare model into a fully engageable, gated, audited, notifying resource — the
concrete model stays a near-empty declaration. Hunt: an engagement missing `tenant_id` · a counter updated
without the morph row (or vice-versa) · an ungated action (no `allow_*` check) · per-row N+1 loading counts ·
unqueued notification fan-out on the request path · `Storage::` inlined instead of the driver.
