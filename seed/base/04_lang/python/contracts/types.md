# Types

- **Type hints on every signature:** parameters and returns, always — `def join ( self, *paths: str ) -> str:`. Untyped public functions are defects; hints are the contract reviewers and tools verify.
- **Fixed-shape data is a class, not a dict.** Known keys mean a `@dataclass` (frozen where value-shaped) or a `NamedTuple`; a dict with documented keys is an object smuggled past the type checker. Dicts are only for genuinely dynamic maps.
- **Protocols over inheritance for contracts:** a `Protocol` declares what a driver/handler must provide; implementations stay uncoupled — the port-and-adapter seam expressed structurally.
- **Enums for closed sets:** statuses, kinds, modes — `Enum`/`StrEnum` with behaviour methods where the set owns logic; string literals repeated across files are the typo waiting.
- **`None` is deliberate:** `Optional[T]` / `T | None` means absence is a valid state, never "unannotated"; guard early (`if value is None: return default`) and keep the rest of the body narrow.
- **Exceptions are the error channel** — raise domain-named exceptions carrying context (`path`, `name`, the fix); never return sentinel strings or silent `None` for failures the caller must know about. Catch narrowly, never `except:` bare, never swallow.
- Boundaries coerce, interiors trust: one cast/validation utility normalizes external input (env, files, APIs) into typed shapes at the edge; internal code never re-checks what the boundary guaranteed.
- Mutability discipline: module-level state is configuration read once, never a mutable cache without a guard; default arguments are never mutable.
- The checker at strict settings is the floor where the project configures one — no `# type: ignore` without a named reason; fix the cause.
