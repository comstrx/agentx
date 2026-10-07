# Seams

Designing the internal joints that make a monolith flexible: swap, test, and extract without surgery.

- Every infrastructure capability sits behind a **port**: a small neutral interface owned by the consumer's side (cache, storage, mail, search, payment, queue, ai). Business code names the port, never the vendor.
- The adapter anatomy: one facade/manager as the only entry point + one `Driver` interface + one concrete driver per backend. Adding or replacing a backend = **one new driver file plus config** — zero call-site changes.
- Keep the neutral interface even when only one backend exists; the interface is what makes the future swap a one-file change instead of a rewrite.
- The composition root is the only place that knows which driver is live. Selection by config, not by conditionals scattered through consumers.
- Third-party integrations get the **local-first saga**: record the intent locally as pending → call the provider through its port → confirm on response or webhook → reconcile failures with bounded retries. The provider being down never corrupts local truth.
- Events decouple modules inside the process: publish a fact, let subscribers react. When durable delivery starts to matter, add an outbox table — same seam, stronger guarantee, no consumer changes.
- Seams are the test strategy: a port gets an in-memory fake once, and every consumer tests against it for free. If something is hard to fake, its seam is badly cut.
- Cut seams along **capabilities** (what it does), never along technologies (what it uses) — `Payments`, not `StripeService`; `Search`, not `ElasticClient`.
- Do not multiply seams: one port per capability, earned by real use. A seam nothing swaps, fakes, or extracts is indirection debt.
