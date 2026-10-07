# Engine

The core pattern — this is the magic that keeps every concrete class near-empty.

- Each layer's real logic lives in **one engine unit** per layer (a trait / mixin / base implementation): the model engine, the repository engine, the service engine, the controller engine, the request engine, the resource engine.
- Each layer has a thin **base shell** class that does nothing but mount its engine unit and wire the inner layer through its constructor.
- Concrete classes **extend the shell and stay almost empty** — declaring only what is unique: the declared write-fields, a relation, an override. A concrete class exists to declare differences, not to re-implement the pattern.
- **New shared behaviour goes in the engine, never in a concrete class.** A concrete class that grows real logic is a smell — push it down.
- **The engine/override split:** needed by two resources → engine-level; needed by one → a concrete override that stays concrete until a second resource asks.
- **You declare; the engine materializes.** The measurable bar: a new resource costs ~4 declarations — schema migration + model (mount its traits) + repository write-fields + service (overrides only) — and gains a full gated, scoped, cached, N+1-free API for free. If adding a resource makes you copy logic, the engine is missing it.
- Most resources are near-empty declarations. Genuine domain logic the engine cannot derive — auth, an order/payment lifecycle, a settlement rule — lives in that resource's service as an explicit pipeline: the deliberate exception, not the norm. If a "simple" resource grows real logic, first ask whether it belongs in the engine.
- The magic policy: **deterministic automation the team understands, never accidental obscurity.** Derived behaviour fails loud and closed — an unknown relation is a 404, an undeclared field is rejected, a missing scope denies. Never silently wrong.
- Engine internals are built on the support layer — the engine orchestrates, support powers. An engine unit re-implementing a native capability inline is itself misplaced code.
