# Mastery

Python written by an engineer, not a scripter.

- **Comprehensions and generators are the loops:** `[clean(p) for p in paths if p]` over append-loops; generator expressions for single-pass volume; `yield` streams big datasets with flat memory — files, table walks, API pages. A function building a giant list for one consumer is a generator not yet written.
- **Context managers own every lifecycle:** files, locks, connections, temp state, timers — `with` guarantees the release on every path; a hand-written `try/finally` pair is a context manager waiting to be extracted (`@contextmanager` makes it three lines).
- **Decorators are the cross-cutting layer:** retry, cache, timing, guard, registration — behaviour declared at the definition and implemented once; stacked decorators compose like the layers they mirror.
- **Dunder methods make types native:** `__call__` for engine objects, `__enter__/__exit__` for lifecycles, `__iter__` for walkable domains, `__repr__` that a human debugging at 3 AM will thank — the language rewards types that behave like the language.
- **Concurrency by workload:** threads for IO-bound fan-out, processes for CPU-bound crunching, `asyncio` only when the whole edge is async — never mixed casually; queues move work, no shared mutable state without a lock.
- **The stdlib is the first dependency:** `pathlib`/`os.path`, `itertools`, `functools` (`lru_cache`, `partial`, `reduce`), `collections` (`defaultdict`, `Counter`, `deque`), `subprocess` with timeouts, `concurrent.futures` — a pip install must beat all of these to earn its place.
- **Introspection with discipline:** `getattr`-driven dispatch and registry patterns are backed by an explicit, printable map that fails loudly on unknown names — dynamic, never mysterious.
- Performance instincts: measure with `timeit`/profilers before believing anything; string-join over concatenation loops; set membership over list scans; slots on hot small classes; lazy imports for heavy optional paths.
