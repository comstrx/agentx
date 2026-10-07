# Ports

- **Every external provider lives behind a port the project owns:** a neutral interface named for the capability (`Payments`, `Bookings`, `Dns`, `Sms`, `Ai`) + one driver per provider + a manager that selects by config. Business code names the capability, never the vendor — adding or replacing a provider is one driver file plus config.
- **The provider's vocabulary stops at the driver:** requests translate in, responses translate out — provider ids, enums, and error codes map to the project's own types at the port; a provider field name appearing in a service or the schema (beyond a stored `provider_ref`) is a leak.
- **Local truth first:** the system's state machine is the reality; the provider is an executor. Every provider interaction follows the saga: record local intent `pending` → call through the port → confirm via response or webhook → reconcile stragglers by schedule. The provider being down, slow, or wrong never corrupts local state.
- **Store the evidence:** provider references, raw response snapshots (redacted), and timestamps ride with every interaction — disputes and debugging replay from the trail, not from memory.
- **Every call is armored by default in the driver base:** timeout, bounded retries with jitter (idempotent calls only), circuit breaker, rate-limit respect for the provider's quotas — individual drivers inherit the armor, never re-implement or skip it.
- **Errors classify at the port:** transient (retryable), rejected (the request is wrong — do not retry), provider-down (degrade path) — business code branches on the class, never string-matches provider messages.
- **Sandbox parity is a requirement:** every driver runs against the provider's test environment in CI-shaped smoke tests; config switches environment, code never does.
- **Provider webhooks land in the same port:** verified, deduped, translated to domain events by the driver — the rest of the system consumes its own events, unaware the trigger was external.
- Contracts with providers are versioned dependencies: SDK/API version pinned, upgrade is a reviewed change with the driver's tests as the gate.
