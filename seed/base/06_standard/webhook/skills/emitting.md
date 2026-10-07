# Emitting

Outbound webhooks — being the provider your consumers wish they had.

- **Sign everything:** HMAC over the raw body with a per-endpoint secret + timestamp header; document the verification recipe; support secret rotation with a dual-signing window so consumers rotate without downtime.
- **Deliver from the outbox, per subscription:** domain events fan out to subscription rows; each delivery is its own queued job with its own retry schedule — one slow consumer never delays another; the event id travels so consumers can dedup.
- **Retry with published policy:** exponential backoff + jitter over a declared window (hours to a day), then dead-letter with the failure trail; consumers see the policy in the docs and the dashboard — surprises are support tickets.
- **Consumer endpoints are health-tracked:** consecutive-failure counts auto-disable a dead endpoint with a notification to its owner; a re-enable + replay-from-timestamp path exists — hammering a dead URL forever serves nobody.
- **Payloads are versioned contracts:** additive evolution, a version field, per-subscription version pinning where consumers lag — the same schema law as internal events, because that is what they are: internal events wearing a public envelope (redacted to the subscriber's scope; a tenant's webhook never carries another tenant's data).
- **The delivery ledger is a product surface:** per subscription — event, attempts, last status, response codes, latencies — visible to the consumer (panel/API) with a manual redeliver button; "did you send it" answered by their own eyes.
- **Egress hygiene:** destination URLs validated against SSRF (no internal ranges), HTTPS enforced, timeouts tight, response bodies size-capped and never trusted as instructions.
- Test the consumer experience: a sample-payload generator + a test-delivery button per subscription — integration partners onboard against real shapes in minutes, not against PDFs.
