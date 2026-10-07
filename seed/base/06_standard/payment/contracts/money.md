# Money

- **Integer minor units only — floats never touch money.** All arithmetic through one money type/utility (amount + currency, checked operations, explicit rounding rules per operation); a naked numeric crossing layers without its currency is a defect.
- **Double-entry ledger is the source of truth:** every movement is a balanced transaction of entries (debit account, credit account, amount, reference, timestamp) — balances are derived (materialized with the ledger as proof), never a mutable column that "gets updated". If the ledger and a balance disagree, the ledger wins and the discrepancy is an incident.
- **Idempotency keys on every money-moving endpoint and job:** client-supplied or event-derived, enforced before the ledger writes — duplicate delivery is a certainty; a double charge is the one bug the domain never forgives.
- **Money state machines are explicit:** payment: `pending → authorized → captured → settled` with `failed/refunded/disputed` branches; every transition emits its event, illegal transitions are unrepresentable, and each state names who may move it (webhook, reconciler, operator).
- **The gateway is a port:** one neutral payments interface + provider drivers (charge, capture, refund, payout, webhook parsing) — business code never names a provider; adding one is a driver file. Provider vocabulary translates at the port and never leaks inward.
- **Never store what the provider should hold:** no PAN/CVV ever — tokens and provider references only; the compliance surface stays at the provider by design.
- **Amounts verified at every trust boundary:** the webhook's amount against the local intent, the refund against the captured remainder, the payout against the settled balance — client-supplied and provider-supplied numbers are claims to verify, not facts.
- **Reconciliation is scheduled truth:** provider reports diffed against the ledger on rhythm; drift alerts a human with the exact references. Unreconciled money ages loudly, never silently.
- Multi-currency is explicit: conversion at a recorded rate + timestamp, stored with both sides — recomputing history from "today's rate" is corruption.
