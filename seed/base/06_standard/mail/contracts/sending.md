# Sending

- **Mail goes through one facade over a swappable transport driver** — provider HTTP APIs over SMTP always (deliverability, observability, retries); the provider is config, business code never names it.
- **Always queued, never inline:** no request waits on a mail provider; the send is a job with bounded retries + backoff and a dead-letter end state someone can replay. The deliberate contrast with realtime events, which broadcast immediately — the latency budget decides, per channel.
- **Idempotent by message identity:** a message key (event id + recipient) guards the resend storm — a retried job or replayed event must not double-send; the provider's dedup or a sent-ledger enforces it.
- **Templates are content, not code:** named templates with typed variable shapes, localized by the recipient's locale (keys, never hardcoded strings), previewable without sending. A template rendering business logic is a layer leak — the pipeline decides, the template displays.
- **Tenant-scoped sending in multi-tenant systems:** from-identity, branding, and reply-to resolve from the tenant's config; one tenant's mail must never wear another's identity. Sender domains are authenticated (SPF/DKIM/DMARC as provider setup) — deliverability is engineering, not luck.
- **Transactional and bulk never share a lane:** separate queues (and ideally separate sending identities) so a campaign never delays a password reset; transactional mail is latency-monitored.
- **Suppression is law:** bounces, complaints, and unsubscribes honored structurally via provider webhooks feeding a suppression list checked before every send — mailing a suppressed address is a compliance defect.
- **The mail trail is auditable:** what was sent, to whom, when, which template/version, delivery status from provider webhooks — support's first question answered by a query, not a shrug.
- Never send real mail from non-production: dev/staging route to a trap (log, mailhog-shaped catcher, whitelist) — a test run emailing customers is an incident.
