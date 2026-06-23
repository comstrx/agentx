# Skill — SaaS domain playbook (money, plans, orders, idempotency)

The recurring business systems of a multi-tenant SaaS, and the ways each goes wrong. All of it is
tenant-scoped, queued where heavy, and expressed as pipeline business logic over the engine (`design.md §1`).

## Money — double-entry ledger, integer minor units
- **Never floats. Never a mutable balance column you `UPDATE`.** Money is integer minor units (cents) handled
  by `support/num/Money` only.
- **Double-entry:** every movement is a balanced set of ledger rows (debit one account, credit another; sum =
  0). The balance is **derived** from the ledger (or a cached balance maintained atomically under a lock).
- **Race safety:** a balance read-modify-write without a lock double-spends under concurrency. Take a
  distributed lock (`support/lock`) or `SELECT … FOR UPDATE` around the mutation.
- **Idempotency on every money endpoint** (below) — a retried charge must not double-charge.

## Plans & feature-gates
- A plan declares **features** (on/off) and **limits** (quotas). Gate access by a check (`HasFeature` /
  middleware), not by scattering `if plan == 'premium'` across the code.
- Limits drive **per-plan rate limiting** (`support/throttle`, tenant+plan keyed).
- Enforce gates **server-side** — never trust a client-sent plan/permission.

## Orders & lifecycle = explicit state machines
- Model the lifecycle as **named states + allowed transitions** (a `HasState` DNA trait). A transition is
  **guarded** (is it legal from the current state?) and **emits a domain event** via the events abstraction.
- Never mutate `status` by hand from anywhere — go through the one transition method; illegal transitions fail
  loud.

## Idempotency (safe retries, zero double side-effects)
- Client sends `Idempotency-Key` on writes. Middleware: under a **lock** keyed `tenant+user+endpoint+key`,
  check the store — if a response is recorded for this key, **replay it**; else run the action and store
  `key → response`. Network retries become safe no-ops.
- Mandatory on all financial/side-effectful POSTs.

## Payments — gateway contract + lifecycle
- A `PaymentGateway` interface + a concrete driver (e.g. Stripe) + a manager; adding a provider = ONE file
  (`design.md §6`).
- Full lifecycle as a state machine: `intent → authorize → capture → refund → dispute → payout`.
- **Webhooks:** verify the signature, be **idempotent** (providers redeliver — a non-idempotent webhook
  double-applies), advance the state machine, and ACK fast (do heavy work in a queued job).

## The failure modes (hunt these)
- Floats for money · a mutable balance updated without a lock · a non-idempotent payment endpoint or webhook
  (double-charge) · `status` mutated outside the state machine · a feature gate trusted from the client · a
  financial/order row missing `tenant_id` scope · heavy payment/email work done inline instead of queued.
