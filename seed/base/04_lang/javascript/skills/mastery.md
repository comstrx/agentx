# Mastery

The JavaScript instincts that survive production.

- **The event loop is the architecture:** never block it — chunk heavy synchronous work, stream large payloads, move real crunching to workers. Understand microtask vs macrotask ordering well enough to never be surprised by it.
- **Async orchestration as vocabulary:** `Promise.all` for independent fan-out, `allSettled` when partial failure is data, `race` for timeouts, async iterators (`for await`) for streams and pagination. Sequential `await` in a loop that could fan out is the classic hidden 10x.
- **`AbortController` everywhere cancellation matters:** fetches, long operations, event listeners — wired through so a leaving user or a superseded request actually stops work.
- **Closures are the module system's soul:** factory functions capturing private state beat classes for most stateful units; composition of small functions beats inheritance trees everywhere the language does not force `class`.
- **Collections by intent:** `Map` for keyed lookups with non-string keys and real deletion, `Set` for membership, arrays for order; `structuredClone` for deep copies — a JSON round-trip clone is a data-loss bug (dates, undefined, bigints).
- **Timing discipline:** debounce user-driven storms, throttle scroll/resize observers, prefer `requestAnimationFrame` for visual work, `queueMicrotask` for after-this-tick logic — named wrappers, not inline timer soup.
- **Memory leaks have known shapes:** forgotten listeners, timers surviving their owner, caches without eviction, closures capturing giant scopes — every subscription's teardown is written at the moment of subscription.
- **Errors with lineage:** `throw new Error(msg, { cause })` preserves the chain; custom error classes for the domain; `try/catch` only around what can actually fail, wide enough to add context, narrow enough to not hide bugs.
- Numbers honestly: integer money in minor units, `BigInt` past 2^53, never float equality; dates through one utility, always UTC internally.
