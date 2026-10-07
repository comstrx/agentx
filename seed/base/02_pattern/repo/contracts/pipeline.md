# Pipeline

The headline law: business code is a pure pipeline.

- A service / lifecycle method expresses business intent as a **pipeline of named calls into the support std-lib and the engine** — and nothing native inline. No raw string/array munging, no hand loops for transforms, no ad-hoc query building, no direct http/cache/storage calls sitting in a business method.
- A missing capability is added to its **matching support domain first** — named well, at the right altitude — then called; the support library and the capability traits grow richer with every feature while business files stay declarative pipelines that **read like the use case** they implement.
- Transactions wrap at the service level around the pipeline, not inside repositories; domain events are emitted by the pipeline as facts after the decision is made.
- A pipeline step is a sentence: `Wallet::debit(...)`, `Stock::reserve(...)`, `Notify::vendor(...)`. If a reviewer cannot read the method aloud as the business rule, the pipeline has leaked implementation.
- The pipeline law is what makes review cheap, reuse automatic, and the codebase teachable — hold it as a gate concern, not a style preference.
