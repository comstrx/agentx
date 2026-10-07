# Lifecycle

Running payments end to end — the flow that must survive every failure mode.

- **The spine is the local-first saga:** create the local intent `pending` (with idempotency key) → call the gateway through the port → record the provider reference → finalize on the **webhook**, never on the redirect. The user's browser closing mid-flow is a Tuesday; the webhook is the truth channel, the return URL is UX.
- **Webhook handling for money:** verify signature → dedup by event id → load the local intent by provider reference → verify amount/currency against the intent → transition the state machine → write the ledger entries → emit the domain event — all one transaction where the store allows, idempotent throughout. Unknown intent = park and alert, never discard.
- **Every stuck state has a sweeper:** authorized-but-never-captured, pending-past-timeout, refund-initiated-without-confirmation — scheduled reconcilers query the gateway and resolve or escalate; a payment state older than its SLA is on a dashboard, not in the dark.
- **Refunds are movements, not deletions:** partial or full, they get their own ledger entries, their own idempotency keys, their own state trail referencing the original — history is append-only; "fixing" a payment by editing rows is fraud-shaped.
- **Disputes/chargebacks enter as events:** provider webhook → freeze the disputed movement's consequences (hold the vendor split), open the case trail, ledger the reversal when ruled — designed flow, not support improvisation.
- **Split/marketplace money is ledger topology:** platform fee, vendor share, tax — separate accounts with entries per split at capture/settlement; payouts drain the vendor account through the same port + idempotency + reconciliation machinery as charges.
- **Test with the provider's chaos in mind:** duplicate webhooks, out-of-order events (capture before authorization's webhook), amount mismatch, timeout-then-success — the sandbox happy path proves nothing; the suite replays the ugly sequences.
- Observability tuned to money: every transition logged with references, conversion/decline rates on dashboards, alert on webhook silence — a quiet payment system is not necessarily a healthy one.
