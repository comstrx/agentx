# Types

The bounded universe of this core — fourteen types, their natures, and the fulfillment archetype each rides. This table IS the type lookup's seed content; the capability law that consumes it lives in the training chain.

- **The fourteen, grouped by archetype:**
  - **date-range** (calendar in/out, capacity per unit, seasonal pricing): `hotels` · `rooms` (child of hotel via self-parent) · `realestate` (rentals) · `travels` (departure-dated packages).
  - **timed-slot** (sessions/seats, schedules, capacity per slot): `tours` · `events` · `conferences`.
  - **stock-shipping** (inventory, reservation, carrier delivery): `products` physical · `books` printed.
  - **instant-digital** (entitlement + signed delivery, no stock ceiling unless declared): `products` digital · `books` digital.
  - **service-delivery** (requirements in, work performed, completion confirmed): `services` offline (scheduled visit — composes a timed-slot pick) · `services` online (gig-shaped).
  - **enrollment** (access contract over content/sessions, windows, progress): `courses`.
  - **application-processing** (documents in, external adjudication, staged outcomes): `visa`.
- **`subtype` refines the nature, never the archetype:** online/offline · digital/physical · the hotel/room kinship — a subtype may switch which archetype variant applies (offline service adds the slot pick) but never invents a new pipeline; if a subtype needs one, it is a new archetype row, a deliberate platform decision.
- **Composition over new machinery:** travels = date-range + application-processing legs (visa assistance) chained as one multi-leg intent; conferences = timed-slot + enrollment traits (session access after purchase); realestate long-stay = date-range with monthly pricing satellites — a client's "new" vertical is first tried as a composition of these before anyone touches the archetype set.
- **Per-type validation and payload shape resolve from the lookup:** required satellites (availabilities for date-range/timed-slot, stock rows for shipping, content tree for enrollment, document checklist for applications), required fields, and the search facets each type exposes — all data on the type row; the boundary and the resource read it, code never hardcodes a type's needs.
- **Cancellation/refund policy templates attach per archetype, tuned per type:** date-range types carry windowed penalty ladders, slots carry cutoff hours, shipping carries return windows, digital is final-on-delivery unless declared, applications refund by stage — policy is satellite data the transition machinery reads.
- **The provider dimension is orthogonal:** any type may fulfill locally (the tenant's own inventory) or through a provider port (paximum/viator-shaped for travel types, marketplace fan-out for gigs) — the archetype stays the same; only the confirmation leg swaps from local commit to the provider saga.
- **ERP is excluded by charter:** accounting exports exist (the ledger is export-ready), but inventory planning, HR, and procurement never enter this core — a request that smells like ERP is a scope refusal, not a new type.
- The moat test for every addition: expressible as a type row + capabilities + satellites = yes; demands a type literal in core code = refused and redesigned through the capability seam.
