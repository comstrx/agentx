# Boundaries

- A module owns its data and its behaviour. Other modules reach it only through its **public surface** (service/facade) — never through its tables, its internals, or a copy of its logic.
- **Branch on capability or permission, never on a literal.** `if type == 'hotel'`, `if is_admin`, `if panel == 'vendor'` are the same rot: every new case reopens every branch. Ask *what can this actor/entity do*, not *what is it called*.
- Shared behaviour moves **down** into the engine or foundation — never sideways as a copy between two modules. Two copies of one rule is one bug with two addresses.
- One vocabulary across the whole system: an entity, a state, a permission has exactly one name everywhere — schema, code, API, UI. Synonyms breed drift.
- Validation lives at the module's boundary; inside it, input is trusted and typed. Re-validating deep in the stack means the boundary leaked.
- A module's events are part of its public surface: publish facts (`order.paid`), never commands to a specific consumer. Consumers subscribe; the publisher stays ignorant of them.
- Reading another module's data for display is allowed through its query surface; **writing another module's data directly is forbidden** — writes go through the owner.
- Feature flags and configuration select behaviour at the composition root or inside the owning module — never as literals sprinkled through consumers.
- When a boundary is crossed constantly in both directions, the boundary is wrong: merge the modules or re-cut the seam. Boundaries serve the domain, not the folder tree.
