# Hygiene

- **Structured always:** key-value/JSON lines with message, level, timestamp, and context — never printf-prose someone will regret grepping. The log is a queryable dataset, not a diary.
- **Correlation id on every line:** one id born at the edge travels through request → service → job → provider call; a single user action reads as one thread across the whole system. Actor and tenant ride the same context, stamped once by the logging facade, never hand-added per call.
- **Secrets never — structurally, not by discipline:** the log facade redacts by key patterns (token, secret, password, key, authorization) and by known value shapes before anything is written; headers and payloads pass through the redactor always. A secret in a log is an incident with rotation duty, not a cleanup.
- **Levels mean things:** `error` = something failed that should not, actionable; `warn` = degraded/suspicious, tolerated; `info` = business-meaningful checkpoints (state changes, money movement); `debug` = development forensics, off in production by default. An error level that alerts nobody or fires hourly is mislabeled.
- **Log decisions and boundaries, not chatter:** state transitions, external calls (with duration + outcome), refusals (auth failures, limit trips, validation walls at suspicious volume) — not every function entry. Every error logged carries its context (ids, operation) — an error line a stranger cannot act on at 3 AM is noise with a stack trace.
- **One error, one log, at the level that handles it** — never log-and-rethrow at every layer; the boundary that decides logs it once with the whole story (`cause` chains preserved).
- **Logs are a stream, not files the app manages:** write to stdout/stderr, let the platform ship them; rotation, retention, and shipping are infrastructure's job.
- Cost is real: sample the high-volume happy path if needed, never the errors; a log line in a hot loop is a performance decision and gets reviewed as one.
