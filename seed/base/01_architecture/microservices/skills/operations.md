# Operations

Running a fleet — the operational discipline that decides whether microservices pay rent or collect it.

- **Observability is the entry fee, paid before the first split:** structured logs with correlation ids shipped centrally, RED metrics per service (rate, errors, duration), distributed traces across hops, and one dashboard per service a stranger can read. Debugging by ssh-and-grep dies at two services.
- **Deploys are boring and independent:** each service ships its own artifact through its own pipeline with health checks, rolling or canary rollout, and instant rollback. Two services needing a coordinated deploy have a contract bug, not a deploy problem.
- **Contract tests replace end-to-end fragility:** each consumer's expectations of a provider are verified against the provider in CI (consumer-driven contracts); the full-fleet staging replica that is always broken is not a test strategy.
- **Configuration and secrets from the environment/platform**, per service, never baked in; a config change is a rollout, auditable and reversible.
- **Failure drills are architecture verification:** kill a service in staging and watch — the fleet should degrade exactly as designed (fallbacks, queues absorbing, breakers open). Surprises found here are boundary bugs found cheap.
- **Backpressure end to end:** bounded queues, load-shedding at the edge, per-consumer rate limits — a traffic spike should queue or shed at the boundary, never cascade into every downstream store.
- **Data migrations across services are sagas too:** expand → double-write/backfill → switch reads → contract, with the event stream as the synchronization spine; no cross-service schema change happens in one leap.
- **Cost and topology reviewed with numbers:** per-service resource baselines, call-graph maps regenerated from traces — the service nobody calls and the hop nobody needed show up in the map, and get deleted. The fleet shrinks as deliberately as it grows.
