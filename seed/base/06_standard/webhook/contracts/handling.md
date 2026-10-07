# Handling

Inbound webhooks — a public door that hostile traffic will knock on.

- **Verify before parse:** signature/HMAC checked against the provider's secret on the RAW body (before deserialization, which can normalize away the evidence); timestamp tolerance enforced against replay; unverifiable requests get a terse 4xx and a security log — never processing "just in case".
- **Ack fast, work queued:** the endpoint verifies, dedups, persists the event, returns 2xx in milliseconds — the real processing is a queued job. A webhook endpoint doing business logic inline times out into the provider's retry storm.
- **Idempotent by event id:** providers redeliver by design; the event id (or a derived hash) guards processing exactly once *in effect* — a processed-ids ledger or upsert-shaped handlers, same law as every consumer.
- **Reorder-tolerant:** the capture webhook may arrive before the authorization's; handlers transition state machines with guards (`apply only from expected states`) and park early arrivals for the sweeper — never assume the provider's chronology survived the network.
- **Verify the claims, not just the signature:** amounts, currencies, and references checked against the local intent — a validly-signed webhook carrying wrong numbers (provider bug, replay, misconfiguration) must fail the guard, alert, and park.
- **Unknown events are parked, not dropped:** an unmatched reference or unrecognized type persists in a quarantine with the payload for inspection — silently discarding is how money disappears; alert on quarantine growth.
- **Per-provider endpoints through the port's driver:** each provider's parsing/verification lives in its driver, translating to domain events; the rest of the system never sees provider payloads.
- **The raw trail is kept** (redacted of secrets): received-at, payload, verification result, processing outcome — replayable by an operator command when a handler bug is fixed.
- 2xx only after durable persistence: an ack for an event that then vanished in a crash is a lost event the provider will never resend.
