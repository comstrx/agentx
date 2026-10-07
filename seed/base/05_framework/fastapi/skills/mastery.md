# Mastery

FastAPI driven with engineering intent.

- **Dependencies compose like functions:** build the ladder once — `session → current_actor → require(permission)` — and endpoints declare only the top rung; `dependency_overrides` swaps any rung in tests, which is the whole test strategy for auth/tenancy paths.
- **Pydantic as the cast layer:** validators normalize at the edge (trim, coerce, canonicalize) so services receive clean shapes; computed fields shape output; `model_config` strictness chosen deliberately per model — the boundary coerces, the interior trusts.
- **Typed pagination/filter dependencies:** one `Pagination` dependency (capped limit, keyset cursor) and one filter-parser dependency shared by every list endpoint — the bounded-DSL law expressed as reusable signatures, never per-endpoint query fishing.
- **Background pressure has a budget:** `BackgroundTasks` only for fire-and-forget trivia (a log, a counter); anything that must survive a crash goes to the real queue — a background task is not a job system, and pretending it is loses work silently.
- **Streaming and files:** `StreamingResponse` for exports/large payloads, `UploadFile` consumed as a stream and piped to storage — request bodies never fully buffered to "inspect" them.
- **Middleware sparingly, dependencies preferably:** middleware for the truly global (correlation id, timing, compression); dependencies for anything route-shaped — they are typed, testable, and self-documenting where middleware is stringly and invisible.
- **Concurrency instincts:** fan out independent awaits with `asyncio.gather`, guard shared clients with connection limits, time-box every outbound call — the event loop's health is the service's health; one stray blocking call shows up as everyone's latency.
- **Test through the app:** `TestClient`/async client against the real app with overridden dependencies — schema validation, auth gates, and envelopes are part of every test, not mocked away.
- Profile before tuning workers: most FastAPI slowness is a blocking call on the loop or a chatty query pattern — find it with real traces, not server-knob folklore.
