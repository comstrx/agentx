# Mastery

Driving actix at production depth.

- **Exploit the type system at the edge:** newtype ids in path extractors (`Path<ProductId>`), enums deserialized from query params, `#[serde(deny_unknown_fields)]` on write payloads — the request is fully typed before the handler's first line, and the invalid request never reaches it.
- **Custom extractors are the reusable boundary:** an `Actor` extractor that reads the auth middleware's context, a `Tenant` extractor that asserts token↔domain agreement, a `Pagination` extractor with capped limits — write once, appear in signatures everywhere; the handler's signature becomes its documentation.
- **The middleware sweet spots:** `from_fn` for the simple 90% (context stamping, timing, headers), full `Transform` implementations only when lifecycle hooks demand it; keep middleware allocation-light — it runs on every request of every worker.
- **Streaming for volume:** `HttpResponse::streaming` for exports and large payloads, chunked uploads consumed as streams — never buffer a file-sized body in memory to "process" it; backpressure flows through the stream.
- **Connection pool hygiene:** pools sized against database limits × replica count, acquired late and dropped early inside handlers, `acquire_timeout` set so pool exhaustion is a fast clear error, never a hang.
- **Graceful lifecycle:** workers drain on SIGTERM (built-in) — respect it: no detached tasks holding work the drain cannot see; long-running background work belongs to a spawned supervisor task with its own shutdown signal, joined at exit.
- **Test through the surface:** `actix_web::test` spins the real App with real routes and middleware against fake ports/pools — handler tests that bypass extractors test nothing that breaks in production.
- Measure before tuning: worker count defaults are usually right; the wins live in query shape, pool sizing, and payload size — flamegraph the real hot path before touching server knobs.
