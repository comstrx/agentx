# Architecture

- **Routers are the surface, thin by law:** an endpoint function validates via its signature, calls one service entry point, returns a typed model — five lines. APIRouter per resource/panel, mounted at the composition root with prefix + tags + shared dependencies; business logic in an endpoint is misplaced code.
- **Pydantic models are the boundary:** request/response schemas as models with real constraints (lengths, ranges, enums, patterns) — validation is the signature, never manual checks inside the body. Separate the shapes honestly: `XCreate`, `XUpdate`, `XOut` — one god-model reused for input and output leaks columns both directions.
- **`Depends` is the seam system:** the actor/tenant context, the database session, pagination, permission gates — all injected dependencies, declared in signatures, overridable in tests. A dependency chain (`get_session → get_actor → require_permission`) IS the middleware ladder, typed and per-route.
- **The layered pattern holds unchanged:** router → service (use-case pipeline, owns the transaction) → repository (query building) → support. Endpoints never build queries; services never import request/response types.
- **Async honestly:** `async def` for IO-bound paths with async drivers end-to-end; sync work or sync drivers go in `def` endpoints (thread-pooled) — an `async def` calling blocking IO stalls the loop for everyone; mixing is a defect, not a style.
- **Response models are contracts:** `response_model` declared on every route — output is filtered and validated on the way out; leaking extra fields because "the dict had them" is the classic FastAPI hole.
- **Errors through one exception family:** domain exceptions raised in services, translated by exception handlers into the uniform envelope + status — endpoints never craft error responses by hand.
- **Settings via typed config** (environment-driven, validated at boot, injected — never `os.getenv` scattered); lifespan hooks own pools and clients: created at startup, closed at shutdown.
- OpenAPI is generated truth: schemas, examples, and tags maintained as code — if the generated docs are wrong, the types are wrong; fix the types.
