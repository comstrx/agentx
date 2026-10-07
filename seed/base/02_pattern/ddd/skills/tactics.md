# Tactics

Applying the model day to day — where the thinking meets the keyboard.

- **Mine the language in requirements:** every noun the business repeats is a candidate entity/value object, every verb a candidate operation or event, every "when X then Y" a candidate domain event + policy. Requirements written in domain language convert to code almost mechanically — that is the point.
- **Design the aggregate by its invariants, then shrink it:** list the rules that must hold in one transaction; everything else leaves the aggregate and syncs eventually. The follow-up question for every member: "must this be consistent *right now*?" — most "yes" answers are habits, not requirements.
- **State machines make lifecycles honest:** order, booking, payment, fulfillment — enumerate states, legal transitions, and the event each transition emits; illegal transitions become unrepresentable instead of un-tested. The transition table IS the requirement document the domain expert can review.
- **Policies react to events:** "when booking confirmed → reserve stock, notify vendor" lives as a named policy/listener, not as extra lines inside the confirm method — the decision stays clean, the consequences stay pluggable.
- **Translate at context borders:** an anti-corruption layer is usually just a small mapper — the provider's `PaymentIntent` becomes our `Payment` at the port, and the provider's vocabulary never leaks inward. Naming the mapper after the border keeps the translation auditable.
- **Enforcement order for an invariant:** type system (unrepresentable) → aggregate guard (rejected) → database constraint (defense-in-depth) — in that order, and ideally all three for the rules money depends on.
- **Refactor toward the model:** when implementation and domain language drift (`processData` handling "settlement"), rename first — cheap, safe, and it exposes the real seams before any structural surgery.
- Model with the domain expert present: a whiteboard hour with the person who runs the process beats a week of speculative class design — capture the events they narrate; their sequence is the architecture.
