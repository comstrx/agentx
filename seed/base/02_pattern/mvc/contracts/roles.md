# Roles

MVC held with discipline — three roles, each thin where it must be.

- **Controllers translate, nothing more:** parse/validate input at the boundary, call one entry point beneath (service or model operation), shape the response. A controller with a business decision, a query chain, or a loop has swallowed another layer — push it down. Target: every action readable in five lines.
- **Models own data shape and domain behaviour** — schema mapping, relations, invariants, domain operations. "Fat model" never means "landfill model": what grows past the entity's own behaviour moves to services/engines; what is reusable mechanics moves to shared traits/mixins.
- **Views are dumb by law:** render what they are handed — formatting, iteration, conditionals on *presentation* only. A view (or serializer/resource) that queries, decides business, or mutates state is the pattern's classic rot. Data arrives prepared; the view is a template, not a program.
- **The flow is one-way per request:** route → controller → domain → back as a prepared view-model/resource. Views never reach back into the domain; controllers never render fragments of business state directly.
- **View-models/resources are the translation layer outward:** the domain's shape is not the wire's shape — a dedicated presenter/resource per surface keeps the domain free to change and the API stable, the same contract law as everywhere.
- **Cross-cutting lives in middleware/filters:** auth, context, locale, throttling — declared on routes, never re-checked ad hoc inside actions.
- MVC is the *surface* pattern, not the architecture: it organizes the transport edge; the layers beneath it (services, repositories, support) carry the system. Treating MVC as the whole design is how 5000-line controllers happen.
