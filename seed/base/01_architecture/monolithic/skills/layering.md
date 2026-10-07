# Layering

Placing every line at its right altitude — the reflex that keeps a monolith clean for years.

- The placement ladder, asked for every new piece of logic:
  1. Is it a **pure capability** (string, date, fs, http, cache, math)? → foundation.
  2. Is it **behaviour two or more features share** (scoping, auditing, deriving, caching policy)? → engine.
  3. Is it a **business decision** unique to one use case? → that feature's domain pipeline.
  4. Is it **translation** (parse input, shape output, map transport codes)? → surface.
- The push-down reflex: native or infrastructure code found inside a business file is **in the wrong layer** — extract it down, name it, then call it. The business line that remains should read like the use case it implements.
- The pull-up smell: a foundation or engine unit that mentions a business noun (order, tenant, invoice) has leaked upward — foundation speaks in primitives, engine speaks in patterns, only the domain speaks the business language.
- When a method is hard to place, split it: most misplaced code is two altitudes fused in one function.
- Test shape follows altitude: foundation = fast pure units, engine = behaviour tests over fakes, domain = use-case tests, surface = thin contract tests. A test needing heavy setup usually reveals code sitting too high.
- Refactor direction is always downward: repeated code compresses into the engine, engine primitives compress into the foundation. Code never migrates up.
- Measure the altitude health cheaply: the domain layer should be the *smallest* layer in line count and the *richest* in meaning. A fat domain layer means the engine is underpowered.
