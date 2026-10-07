# Model

DDD in its pragmatic form — the thinking tools, none of the ceremony.

- **Ubiquitous language is the first deliverable:** the domain's words — one term per concept, used identically in conversation, schema, code, API, and UI. When the business says "booking" the code does not say "reservation"; renaming code to match discovered language is a bug fix, not cosmetics.
- **Bounded contexts before class diagrams:** the same word may mean different things in different contexts (a "customer" in billing vs support) — draw the boundary where the meaning changes, and translate explicitly at the border. One model stretched across contexts becomes everyone's compromise and no one's truth.
- **Aggregates are consistency boundaries, not object graphs:** an aggregate clusters exactly what must be transactionally consistent, guarded by one root — all changes enter through the root, invariants hold at every commit. Small aggregates by default; references between aggregates are ids, never object pointers.
- **Value objects carry the domain's arithmetic:** money, date ranges, addresses, percentages — immutable, equality by value, validating at construction, owning their operations (`Money::add`, `Range::overlaps`). A primitive `float $price` passed around naked is domain logic homeless.
- **Invariants live in the domain layer:** a rule the business states ("a confirmed booking cannot shrink below paid amount") is enforced in the aggregate/service — never scattered across controllers and UI checks that each remember it differently.
- **Domain events record decisions:** past-tense facts emitted when an aggregate commits a meaningful change — the hook for everything downstream without coupling the decision to its consequences.
- **The tolerance line:** entities, value objects, domain events, and clear boundaries — yes, always. Repositories-of-repositories, abstract factories, event-sourcing-by-default, and layers whose only job is passing calls through — no, until a real force demands them. DDD is the language and the boundaries; the pattern zoo is optional equipment.
