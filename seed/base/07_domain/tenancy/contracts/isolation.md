# Isolation

The sacred invariant: a tenant never sees another tenant's data. Everything here serves that sentence.

- **Scope at the lowest layer, fail-closed:** a global scope on every tenant-owned model injects `tenant_id` into every query automatically — forgetting to filter must be impossible, not discouraged. No scope resolvable = no rows, never all rows.
- **Defense-in-depth beneath the code:** row-level security policies on every tenant-owned table, transaction-local tenant setting (never session-level), the application connecting as a non-owner role, policies FORCED — RLS catches the query the code forgot; it is the net, the global scope is the law.
- **`tenant_id` on every tenant-owned row; every business unique includes it;** hot indexes lead with it. The explicit, small list of platform-level tables (the tenant registry, the platform's own users, shared reference data) is the ONLY unscoped set — and nullable-scope uniques close the NULL-distinct hole explicitly.
- **Tenant resolution is one authority:** domain/subdomain → tenant, resolved once in middleware, stamped into the request context, asserted against the token's tenant — token and domain must agree or the request dies. Two resolution paths = one future mismatch.
- **The scope travels with the work:** queued jobs, scheduled tasks, and events carry `tenant_id`, restore it at start, reset at end — background work is requests without browsers, same law. Cache, lock, throttle, and storage keys are tenant-namespaced structurally by the key builders.
- **Cross-tenant is the audited exception:** only the platform's super context may cross, through one named, logged escape hatch (`withoutTenancy()`-shaped) — never a default, never implicit, every use greppable.
- **Tenant lifecycle is first-class:** provisioning (row + defaults + domain), suspension (requests refused cleanly, data intact), export, and deletion (complete, verifiable, satellite data included) — each a designed flow, because each is a contract with a paying client.
- Isolation is tested as behaviour: the suite's fixture is TWO tenants, and every resource's tests assert the other tenant sees nothing — an isolation test failing is a release blocker, category one.
