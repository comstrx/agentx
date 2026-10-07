# Policy

- **Rate limiting is a product rule enforced at the edge, behind one facade** with a swappable driver — `Throttle::attempt(key, limit)` — never broker calls or counter math scattered at call sites.
- **Keys are scoped like everything else:** per-actor, per-tenant, per-endpoint-class — composed by one key builder. Anonymous traffic keys by IP + route class; authenticated traffic keys by actor, so one tenant's storm never spends another's budget.
- **Limits are plan-shaped, not hardcoded:** the quota is configuration attached to the plan/role/capability — reading the actor's context, resolved through the same permission machinery as every other capability. A literal `100` in a middleware is a plan decision hiding in code.
- **Choose the algorithm by the abuse shape:** fixed/sliding window for API fairness, token bucket where bursts are legitimate, concurrency caps for expensive operations (exports, media) — and cost-weight the expensive endpoints instead of counting them as one.
- **The refusal is a first-class response:** 429 with the uniform envelope, `Retry-After` honest, remaining-quota headers where the product exposes them — a limit the client cannot see or predict just teaches retry storms.
- **Layered budgets:** a global per-actor ceiling + tighter per-sensitive-endpoint budgets (auth attempts, OTP, password reset, money movement) — security throttles are separate from fairness throttles and fail closed.
- **Fail-open for fairness, fail-closed for security:** if the limiter's store is down, product endpoints degrade to allowing (log loudly); auth/abuse throttles refuse instead — availability for customers, never for attackers.
- Internal consumers obey limits too: jobs and workers hitting shared resources budget themselves — the queue absorbing a burst must not become the storm that the edge just prevented.
- Limits are observable: hit rates and top-throttled actors on a dashboard — a limit tripping constantly is either an attack or a product decision to revisit, and the data says which.
